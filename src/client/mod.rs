mod query;
mod request;
mod session;

use crate::model::Auth;
use crate::ConnectorError;
use http::header::ToStrError;
use http::{header, HeaderMap, StatusCode, Uri};
use hyper::body::Bytes;
pub use query::{MeliorError, QueryLocation, QueryPath};
use query::{MeliorResponse, Query};
pub use request::RootError;
use request::{Request, RootResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;
use session::Session;
use std::error::Error as StdError;
use std::fmt;
use std::io::Write;
use std::num::ParseIntError;
use std::result::Result as StdResult;

const ROOT_URI: &str = "https://cf2.bonfire.moe";
const MELIOR_URI: &str = "https://api.bonfire.moe";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectorError(ConnectorError),
    JsonError(serde_json::Error),
    HttpError(http::Error),
    HyperError(hyper::Error),
    IoError(std::io::Error),
    MeliorServerError(Vec<MeliorError>),
    ParseIntError(ParseIntError),
    ResponseMissingLength,
    ResponseMissingData,
    RootServerError(RootError),
    ToStrError(ToStrError),
    UnsuccessfulResponse(StatusCode),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectorError(_) => write!(f, "connector error"),
            Self::JsonError(_) => write!(f, "JSON error"),
            Self::HttpError(_) => write!(f, "HTTP error"),
            Self::HyperError(_) => write!(f, "hyper error"),
            Self::IoError(_) => write!(f, "IO error"),
            Self::MeliorServerError(e) => {
                if e.len() > 1 {
                    write!(f, "melior server sent multiple errors")
                } else {
                    write!(f, "melior server sent an error: {}", e[0].message)
                }
            }
            Self::ParseIntError(_) => write!(f, "error parsing an integer"),
            Self::ResponseMissingLength => write!(f, "response is missing length"),
            Self::ResponseMissingData => write!(f, "response is missing data"),
            Self::RootServerError(e) => write!(f, "root server sent an error: {}", e.code),
            Self::ToStrError(_) => write!(f, "error converting to string"),
            Self::UnsuccessfulResponse(status) => {
                write!(f, "response is unsuccessful: {}", status)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::ConnectorError(ref e) => Some(e),
            Self::JsonError(ref e) => Some(e),
            Self::HttpError(ref e) => Some(e),
            Self::HyperError(ref e) => Some(e),
            Self::IoError(ref e) => Some(e),
            Self::ParseIntError(ref e) => Some(e),
            Self::ToStrError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<ConnectorError> for Error {
    fn from(e: ConnectorError) -> Self {
        Self::ConnectorError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::HyperError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ToStrError> for Error {
    fn from(e: ToStrError) -> Self {
        Self::ToStrError(e)
    }
}

pub struct Client {
    pub auth: Option<Auth>,
    root_session: Session,
    melior_session: Session,
    user_agent: String,
    bot_token: String,
}
impl Client {
    pub async fn connect() -> Result<Self> {
        Self::builder().connect().await
    }

    pub(crate) async fn send_request<R: Serialize, S: DeserializeOwned>(
        &mut self,
        request_name: &'static str,
        content: R,
        attachments: Vec<Option<&[u8]>>,
    ) -> Result<S> {
        // the length of each attachment is written to dataOutput and the bytes are sequentially
        // appended at the end of payload in the same order they appear in dataOutput
        let mut data_length = 0;
        let data_output = attachments
            .iter()
            .map(|option| {
                option.map(|slice| {
                    data_length += slice.len();
                    slice.len() as u32
                })
            })
            .collect::<Vec<Option<u32>>>();
        let mut data = Vec::with_capacity(data_length);
        for attachment in attachments.into_iter().flatten() {
            data.write_all(attachment)?;
        }

        let mut headers = HeaderMap::new();
        let request = Request {
            content,
            request_name,
            data_output,
            api_access_token: self
                .auth
                .clone()
                .map_or(String::default(), |auth| auth.access_token),
            api_bot_token: self.bot_token.clone(),
        };

        let mut body = Vec::new();
        serde_json::to_writer(&mut body, &request)?;
        let body_length = (body.len() as u32).to_be_bytes().to_vec();
        body = [body_length, body, data].concat();

        headers.insert(header::USER_AGENT, self.user_agent.parse().unwrap());
        let response = self
            .root_session
            .send_raw(Bytes::from(body), &headers)
            .await?;

        match serde_json::from_slice::<RootResponse<S>>(&response)? {
            RootResponse::Ok(content) => Ok(content),
            RootResponse::Error(error) => Err(Error::RootServerError(error)),
        }
    }

    pub(crate) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &mut self,
        query: &'static str,
        variables: R,
    ) -> Result<S> {
        let mut headers = HeaderMap::new();
        let query = Query { variables, query };

        let mut body = Vec::new();
        serde_json::to_writer(&mut body, &query)?;

        headers.insert(header::USER_AGENT, self.user_agent.parse().unwrap());
        let response = self
            .melior_session
            .send_raw(Bytes::from(body), &headers)
            .await?;

        let response = serde_json::from_slice::<MeliorResponse<S>>(&response)?;
        if response.errors.clone().is_none_or(|e| e.is_empty()) {
            Ok(response.data.ok_or(Error::ResponseMissingData)?)
        } else {
            Err(Error::MeliorServerError(response.errors.unwrap()))
        }
    }

    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

pub struct ClientBuilder {
    root_uri: Uri,
    melior_uri: Uri,
    user_agent: String,
    bot_token: String,
}
impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            root_uri: Uri::try_from(ROOT_URI).unwrap(),
            melior_uri: Uri::try_from(MELIOR_URI).unwrap(),
            user_agent: USER_AGENT.to_owned(),
            bot_token: String::default(),
        }
    }

    pub async fn connect(self) -> Result<Client> {
        Ok(Client {
            auth: None,
            root_session: Session::connect(&self.root_uri).await?,
            melior_session: Session::connect(&self.melior_uri).await?,
            user_agent: self.user_agent,
            bot_token: self.bot_token,
        })
    }

    pub fn root_uri<T>(&mut self, root_uri: T) -> StdResult<&mut Self, <Uri as TryFrom<T>>::Error>
    where
        Uri: TryFrom<T>,
    {
        self.root_uri = Uri::try_from(root_uri)?;
        Ok(self)
    }

    pub fn melior_uri<T>(
        &mut self,
        melior_uri: T,
    ) -> StdResult<&mut Self, <Uri as TryFrom<T>>::Error>
    where
        Uri: TryFrom<T>,
    {
        self.melior_uri = Uri::try_from(melior_uri)?;
        Ok(self)
    }

    pub fn user_agent<T>(&mut self, user_agent: T) -> &mut Self
    where
        String: From<T>,
    {
        self.user_agent = String::from(user_agent);
        self
    }

    pub fn bot_token<T>(&mut self, bot_token: T) -> &mut Self
    where
        String: From<T>,
    {
        self.bot_token = String::from(bot_token);
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
