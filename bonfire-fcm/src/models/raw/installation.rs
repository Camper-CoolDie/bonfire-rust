#[derive(Clone, Debug)]
pub(crate) struct Installation {
    pub id: String,
    pub auth_token: String,
    pub refresh_token: String,
}
