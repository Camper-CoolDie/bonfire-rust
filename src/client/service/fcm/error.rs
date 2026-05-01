use thiserror::Error;

/// Represents errors that can occur during FCM registration or unregistration.
#[non_exhaustive]
#[derive(Error, Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "fcm")))]
pub enum Error {
    /// The Android registration request was rejected by the server
    #[error("Android registration error: {0}")]
    AndroidRegistrationError(String),
    /// An error occurred during ECE (Encrypted Content-Encoding) encryption or decryption
    #[error("ECE error")]
    EceError(#[from] ece::Error),
    /// The server returned an invalid Android registration response.
    ///
    /// This typically happens when the response contains neither a valid token nor an error
    /// message.
    #[error("invalid Android registration response")]
    InvalidAndroidRegistrationResponse,
    /// An error occurred while decoding Protocol Buffers data
    #[error("Protobuf decode error")]
    ProtobufDecodeError(#[from] prost::DecodeError),
    /// An error occurred while encoding Protocol Buffers data
    #[error("Protobuf encode error")]
    ProtobufEncodeError(#[from] prost::EncodeError),
    /// An error occurred while deserializing URL-encoded data
    #[error("URL-encoded deserialize error")]
    UrlEncodedDeserializeError(#[from] serde_urlencoded::de::Error),
    /// An error occurred while serializing data to URL-encoded format
    #[error("URL-encoded serialize error")]
    UrlEncodedSerializeError(#[from] serde_urlencoded::ser::Error),
}
