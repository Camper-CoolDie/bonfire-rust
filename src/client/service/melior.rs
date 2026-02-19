use bytes::Bytes;
use http::{header, HeaderMap, HeaderValue, Method, Uri};
use http_body_util::{BodyExt as _, Full};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;

use crate::client::service::USER_AGENT;
use crate::client::{Request, RequestError};
use crate::queries::RawMeliorError;
use crate::{Error, MeliorError, MeliorQuery, MeliorResponse, Result};

pub(crate) struct MeliorService {
    hyper_client: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    uri: Uri,
}
impl MeliorService {
    pub(crate) fn new(uri: &Uri) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_all_versions()
            .build();

        Self {
            hyper_client: HyperClient::builder(TokioExecutor::new()).build(connector),
            uri: uri.clone(),
        }
    }

    pub(crate) async fn send_query<R: Request>(
        &self,
        query: MeliorQuery<'_, R>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<R::Response>
    where
        for<'a> &'a <R::Error as RequestError>::Source: From<&'a MeliorError>,
    {
        let body = serde_json::to_vec(&query)?;

        let bytes = self.send_raw(Bytes::from(body), &headers).await?;
        let response = serde_json::from_slice::<MeliorResponse<R>>(&bytes)
            .inspect_err(|error| tracing::error!(?error, "failed to parse melior response"))?;

        response
            .data
            .ok_or_else(|| Self::map_errors::<R::Error>(response.errors))
    }

    fn map_errors<E: RequestError>(errors: Option<Vec<RawMeliorError>>) -> Error
    where
        for<'a> &'a E::Source: From<&'a MeliorError>,
    {
        errors
            .and_then(|errors| errors.into_iter().next())
            .ok_or(Error::InvalidMeliorResponse)
            .and_then(|error| {
                Error::try_from_melior::<E>(MeliorError::from(error)).inspect_err(|error| {
                    tracing::error!(?error, "failed to parse a request-specific melior error");
                })
            })
            .unwrap_or_else(|error| error)
    }

    async fn send_raw(&self, body: Bytes, headers: &HeaderMap<HeaderValue>) -> Result<Bytes> {
        let builder = http::Request::builder()
            .uri(&self.uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &**USER_AGENT);

        let builder = headers
            .iter()
            .fold(builder, |builder, (key, value)| builder.header(key, value));

        let request = builder.body(Full::new(body))?;
        let response = self.hyper_client.request(request).await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.collect().await?.to_bytes())
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
