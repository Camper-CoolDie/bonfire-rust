use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawParams {
    pub is_public: bool,
    #[serde(rename = "allowUserInvite")]
    pub allow_invites: bool,
    #[serde(rename = "allowUserNameAndImage")]
    pub allow_changes: bool,
}
