mod fcm;
mod fis;
mod gcm;
mod mcs;

pub(super) use fcm::{
    RegistrationRequest as PushRegistrationRequest,
    UnregistrationRequest as PushUnregistrationRequest,
};
pub(super) use fis::{InstallationRequest, InstallationResponse, RefreshRequest};
pub(super) use gcm::{
    CheckInRequest, CheckInResponse, RegistrationRequest as AndroidRegistrationRequest,
    RegistrationResponse as AndroidRegistrationResponse,
};
pub(super) use mcs::LoginRequest;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Token {
    pub token: String,
}
