mod registration;
mod unregistration;

pub(crate) use registration::RegistrationRequest;
pub(crate) use unregistration::UnregistrationRequest;

const URI: &str = "https://fcmregistrations.googleapis.com/v1";
