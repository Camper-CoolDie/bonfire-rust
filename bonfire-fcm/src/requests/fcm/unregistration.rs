use http::Method;
use http_body_util::{Either, Empty};

use super::URI;
use crate::{Error, HyperClient, Result};

pub(crate) struct UnregistrationRequest<'a> {
    project_id: &'a str,
    api_key: &'a str,
    installation_auth_token: &'a str,
    push_token: &'a str,
}
impl<'a> UnregistrationRequest<'a> {
    pub(crate) fn new(
        project_id: &'a str,
        api_key: &'a str,
        installation_auth_token: &'a str,
        push_token: &'a str,
    ) -> Self {
        Self {
            project_id,
            api_key,
            installation_auth_token,
            push_token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<()> {
        let uri = format!(
            "{URI}/projects/{}/registrations/{}",
            self.project_id, self.push_token
        );

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .header("x-goog-api-key", self.api_key)
            .header(
                "x-goog-firebase-installations-auth",
                self.installation_auth_token,
            )
            .body(Either::Right(Empty::new()))?;

        let response = client.request(request).await?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
