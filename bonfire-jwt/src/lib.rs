mod claims;
mod error;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub use claims::Claims;
pub use error::{Error, Result};

pub fn decode(token: &str) -> Result<Claims> {
    token
        .split('.')
        .nth(1)
        .ok_or(Error::NoPayload)
        .and_then(|data| URL_SAFE_NO_PAD.decode(data).map_err(Error::from))
        .and_then(|decoded| serde_json::from_slice::<Claims>(&decoded).map_err(Error::from))
        .inspect(|claims| {
            tracing::debug!(
                subject = claims.subject,
                expires_at = ?claims.expires_at,
                "decoded token"
            );
        })
        .inspect_err(|error| tracing::error!(?error, "failed to decode token"))
}
