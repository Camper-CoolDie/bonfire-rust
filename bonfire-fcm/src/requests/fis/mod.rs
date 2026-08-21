mod installation;
mod refresh;

use std::sync::LazyLock;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub(crate) use installation::{InstallationRequest, InstallationResponse};
pub(crate) use refresh::RefreshRequest;
use serde_json::json;

const URI: &str = "https://firebaseinstallations.googleapis.com/v1";

static HEARTBEATS: LazyLock<String> = LazyLock::new(|| {
    let payload = json!({"version": 2, "heartbeats": []});
    let serialized = serde_json::to_vec(&payload).expect("failed to serialize heartbeats");
    URL_SAFE_NO_PAD.encode(serialized)
});
