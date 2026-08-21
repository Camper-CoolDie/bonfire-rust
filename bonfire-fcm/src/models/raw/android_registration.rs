#[derive(Clone, Debug)]
pub(crate) struct AndroidRegistration {
    pub android_id: u64,
    pub security_token: u64,
    pub gcm_token: String,
}
