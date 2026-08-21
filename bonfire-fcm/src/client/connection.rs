use std::io;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE;
use bytes::{Bytes, BytesMut};
use ece::EcKeyComponents;
use ece::legacy::{AesGcmEncryptedBlock, decrypt_aesgcm};
use http::Uri;
use prost::Message as _;
use socket2::{SockRef, TcpKeepalive};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;

use crate::models::{RawMessage, RawMessageKind};
use crate::{Error, Result, proto};

const SERVER_ADDR: &str = "mtalk.google.com:5228";
const MCS_VERSION: u8 = 41;

// Connection idle time before sending probe
const TCP_KEEPALIVE_TIME: Duration = Duration::from_secs(45);
// Duration between successive probes (if the first one gets no response)
const TCP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(5);
// Maximum unacknowledged probes before connection drop
const TCP_KEEPALIVE_RETRIES: u32 = 3;
// Keepalive probes cannot be sent during write operations, so writes should also have a timeout
const WRITE_TIMEOUT: Duration = Duration::from_secs(30);

static SERVER_NAME: LazyLock<ServerName> = LazyLock::new(|| {
    let host = SERVER_ADDR
        .parse::<Uri>()
        .expect("failed to parse MSC address as Uri")
        .host()
        .expect("missing host in MSC address")
        .to_owned();
    ServerName::try_from(host).expect("failed to create ServerName from MSC host")
});

type Reader = BufReader<ReadHalf<TlsStream<TcpStream>>>;
type Writer = BufWriter<WriteHalf<TlsStream<TcpStream>>>;

#[derive(Clone)]
pub(crate) struct Connection {
    reader: Arc<Mutex<Reader>>,
    writer: Arc<Mutex<Writer>>,
    key_pair: Arc<(EcKeyComponents, [u8; 16])>,
}
impl Connection {
    pub(crate) async fn connect(
        key_components: EcKeyComponents,
        auth_secret: [u8; 16],
    ) -> Result<Self> {
        let keepalive = TcpKeepalive::new()
            .with_time(TCP_KEEPALIVE_TIME)
            .with_interval(TCP_KEEPALIVE_INTERVAL)
            .with_retries(TCP_KEEPALIVE_RETRIES);
        let config = ClientConfig::builder()
            .with_root_certificates(RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            })
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));
        let tcp_stream = TcpStream::connect(SERVER_ADDR).await?;
        let socket_ref = SockRef::from(&tcp_stream);
        socket_ref.set_tcp_keepalive(&keepalive)?;

        let stream = connector.connect(SERVER_NAME.clone(), tcp_stream).await?;
        let (reader, writer) = tokio::io::split(stream);
        Ok(Self {
            reader: Arc::new(Mutex::new(BufReader::new(reader))),
            writer: Arc::new(Mutex::new(BufWriter::new(writer))),
            key_pair: Arc::new((key_components, auth_secret)),
        })
    }

    pub(crate) async fn shutdown(&self) -> Result<()> {
        self.writer.lock().await.shutdown().await?;
        Ok(())
    }

    pub(crate) async fn read_and_check_version(&self) -> Result<()> {
        let mut guard = self.reader.lock().await;
        let version = guard.read_u8().await?;

        if version >= MCS_VERSION {
            Ok(())
        } else {
            Err(Error::McsProtocolError(format!(
                "unsupported version: {version}"
            )))
        }
    }

    pub(crate) async fn read(&self) -> Result<Option<RawMessage>> {
        let mut guard = self.reader.lock().await;
        let (kind, bytes) = Self::read_raw(&mut guard).await?;

        let message = match kind {
            RawMessageKind::DataMessageStanza => {
                let message = proto::DataMessageStanza::decode(bytes)?;
                Self::decode_data(&self.key_pair.0, &self.key_pair.1, message)?
            }
            RawMessageKind::HeartbeatPing => {
                RawMessage::HeartbeatPing(proto::HeartbeatPing::decode(bytes)?)
            }
            RawMessageKind::HeartbeatAck => {
                RawMessage::HeartbeatAck(proto::HeartbeatAck::decode(bytes)?)
            }
            RawMessageKind::LoginResponse => {
                RawMessage::LoginResponse(proto::LoginResponse::decode(bytes)?)
            }
            RawMessageKind::Close => RawMessage::Close,
            RawMessageKind::IqStanza => return Ok(None),
            RawMessageKind::Unknown(unknown) => {
                return Err(Error::McsProtocolError(format!("unknown tag: {unknown:?}")));
            }
            other @ RawMessageKind::LoginRequest => {
                return Err(Error::McsProtocolError(format!(
                    "unexpected message: {other:?}"
                )));
            }
        };

        tracing::debug!(?kind, "received message");
        Ok(Some(message))
    }

    async fn read_raw(reader: &mut Reader) -> Result<(RawMessageKind, Bytes)> {
        let tag = reader.read_u8().await?;
        let size = Self::read_varint(reader).await?;

        let mut buffer = BytesMut::zeroed(size);
        reader.read_exact(&mut buffer).await?;
        Ok((tag.into(), buffer.freeze()))
    }

    async fn read_varint(reader: &mut Reader) -> Result<usize> {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let byte = reader.read_u8().await?;
            // Extract 7 bits and shift them
            value |= ((byte & 0x7f) as usize) << shift;
            // Check for continuation bit
            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= usize::BITS {
                return Err(Error::McsProtocolError(format!(
                    "varint cannot be larger than {} bits",
                    usize::BITS
                )));
            }
        }
        Ok(value)
    }

    fn decode_data(
        key_components: &EcKeyComponents,
        auth_secret: &[u8; 16],
        message: proto::DataMessageStanza,
    ) -> Result<RawMessage> {
        // RawMessage::Data
        let mut dh = None;
        let mut salt = None;
        // RawMessage::MessagesDeleted
        let mut message_kind = None;
        let mut total_deleted = None;

        for (key, value) in message
            .app_data
            .into_iter()
            .map(|data| (data.key, data.value))
        {
            match key.as_str() {
                "crypto-key" => {
                    if dh.is_some() {
                        return Err(Error::duplicate_field("crypto-key", "data message"));
                    }

                    if let Some(("dh", key)) = value.split_once('=') {
                        dh = Some(URL_SAFE.decode(key)?);
                    } else {
                        return Err(Error::invalid_format("crypto-key", "data message"));
                    }
                }
                "encryption" => {
                    if salt.is_some() {
                        return Err(Error::duplicate_field("encryption", "data message"));
                    }

                    if let Some(("salt", key)) = value.split_once('=') {
                        salt = Some(URL_SAFE.decode(key)?);
                    } else {
                        return Err(Error::invalid_format("encryption", "data message"));
                    }
                }
                "message_type" => {
                    if message_kind.is_some() {
                        return Err(Error::duplicate_field("message_type", "data message"));
                    }
                    message_kind = Some(value);
                }
                "total_deleted" => {
                    if total_deleted.is_some() {
                        return Err(Error::duplicate_field("total_deleted", "data message"));
                    }
                    total_deleted = Some(value.parse::<usize>().map_err(|error| {
                        Error::conversion(error, "total_deleted", "usize", "data message")
                    })?);
                }
                _ => {}
            }
        }

        let persistent_id = message
            .persistent_id
            .ok_or(Error::missing_field("persistent_id", "data message"))?;

        if let Some(count) = total_deleted
            && matches!(message_kind.as_deref(), Some("deleted_messages"))
        {
            Ok(RawMessage::MessagesDeleted {
                persistent_id,
                count,
            })
        } else {
            let dh = dh.ok_or(Error::missing_field("crypto-key", "data message"))?;
            let salt = salt.ok_or(Error::missing_field("encryption", "data message"))?;
            let raw_data = message
                .raw_data
                .ok_or(Error::missing_field("raw_data", "data message"))?;

            let block = AesGcmEncryptedBlock::new(&dh, &salt, 4096, raw_data)?;
            let body = decrypt_aesgcm(key_components, auth_secret, &block)?.into();

            Ok(RawMessage::Data {
                persistent_id,
                body,
            })
        }
    }

    pub(crate) async fn write_version(&self) -> Result<()> {
        self.writer
            .lock()
            .await
            .write_u8(MCS_VERSION)
            .await
            .map_err(Into::into)
    }

    pub(crate) async fn write(&self, message: RawMessage) -> Result<()> {
        let mut guard = self.writer.lock().await;
        let kind = message.kind();
        tracing::debug!(?kind, "sending message");

        timeout(WRITE_TIMEOUT, async {
            match message {
                RawMessage::HeartbeatPing(ping) => Self::write_raw(&mut guard, kind, ping).await,
                RawMessage::HeartbeatAck(ack) => Self::write_raw(&mut guard, kind, ack).await,
                RawMessage::LoginRequest(request) => {
                    Self::write_raw(&mut guard, kind, request).await
                }
                _ => unreachable!("message cannot be sent: {kind:?}"),
            }
        })
        .await
        .map_err(|_| Error::IoError(io::ErrorKind::TimedOut.into()))
        .and_then(|error| error)?;
        Ok(())
    }

    async fn write_raw<T: prost::Message>(
        writer: &mut Writer,
        kind: RawMessageKind,
        message: T,
    ) -> Result<()> {
        let mut bytes = BytesMut::with_capacity(message.encoded_len());
        message.encode(&mut bytes)?;
        let bytes = bytes.freeze();

        writer.write_u8(kind.into()).await?;
        Self::write_varint(writer, bytes.len()).await?;
        writer.write_all(&bytes).await?;
        writer.flush().await?;
        Ok(())
    }

    #[expect(clippy::cast_possible_truncation)]
    async fn write_varint(writer: &mut Writer, mut value: usize) -> Result<()> {
        loop {
            // Take lower 7 bits and shift them
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            // Set the continuation bit
            if value != 0 {
                byte |= 0x80;
            }
            writer.write_u8(byte).await?;

            // Last byte
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}
