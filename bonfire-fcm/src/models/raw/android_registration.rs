use ece::EcKeyComponents;

#[derive(Clone, Debug)]
pub(crate) struct AndroidRegistration {
    pub android_id: u64,
    pub security_token: u64,
    pub gcm_token: String,
    pub key_components: EcKeyComponents,
    pub auth_secret: [u8; 16],
}
