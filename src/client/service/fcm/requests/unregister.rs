use bytes::Bytes;
use http::Method;
use http_body_util::{BodyExt as _, Either, Empty};

use super::REGISTRATION_URI;
use crate::client::HyperClient;
use crate::{Error, Result};

pub(crate) struct UnregisterRequest<'a> {
    project_id: &'a str,
    api_key: &'a str,
    installation_auth_token: &'a str,
    token: &'a str,
}
impl<'a> UnregisterRequest<'a> {
    pub(crate) fn new(
        project_id: &'a str,
        api_key: &'a str,
        installation_auth_token: &'a str,
        token: &'a str,
    ) -> Self {
        Self {
            project_id,
            api_key,
            installation_auth_token,
            token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<Bytes> {
        let uri = format!(
            "{REGISTRATION_URI}/projects/{}/registrations/{}",
            self.project_id, self.token
        );

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .header("x-goog-api-key", self.api_key)
            .header(
                "x-goog-firebase-installations-auth",
                self.installation_auth_token,
            )
            .body(Either::Right(Empty::<Bytes>::new()))?;

        let response = client.request(request).await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.collect().await?.to_bytes())
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
