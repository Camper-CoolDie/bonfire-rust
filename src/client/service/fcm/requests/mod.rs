mod android;
mod register;
mod unregister;

const REGISTRATION_URI: &str = "https://fcmregistrations.googleapis.com/v1";

pub(super) use android::{
    CheckinRequest as AndroidCheckinRequest, InstallationRequest,
    RegisterRequest as AndroidRegisterRequest,
};
pub(super) use register::RegisterRequest;
pub(super) use unregister::UnregisterRequest;
