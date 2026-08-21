use thiserror::Error;

#[derive(Error, Debug)]
pub enum AndroidRegistrationError {
    #[error("phone registration failed")]
    PhoneRegistration,
    #[error("authentication failed")]
    Authentication,
    #[error("invalid sender")]
    InvalidSender,
    #[error("invalid parameters")]
    InvalidParameters,
    #[error("internal server error")]
    InternalServerError,
    #[error("quota exceeded")]
    QuotaExceeded,
    #[error("too many registrations")]
    TooManyRegistrations,
    #[error("too many subscribers")]
    TooManySubscribers,
    #[error("invalid target version")]
    InvalidTargetVersion,
    #[error("FIS authentication failed")]
    FisAuthentication,
    #[error("unknown registration error: {0}")]
    Other(String),
}
impl AndroidRegistrationError {
    pub(crate) fn new(code: String) -> AndroidRegistrationError {
        match code.as_str() {
            "PHONE_REGISTRATION_ERROR" => AndroidRegistrationError::PhoneRegistration,
            "AUTHENTICATION_FAILED" => AndroidRegistrationError::Authentication,
            "INVALID_SENDER" => AndroidRegistrationError::InvalidSender,
            "INVALID_PARAMETERS" => AndroidRegistrationError::InvalidParameters,
            "INTERNAL_SERVER_ERROR" => AndroidRegistrationError::InternalServerError,
            "QUOTA_EXCEEDED" => AndroidRegistrationError::QuotaExceeded,
            "TOO_MANY_REGISTRATIONS" => AndroidRegistrationError::TooManyRegistrations,
            "TOO_MANY_SUBSCRIBERS" => AndroidRegistrationError::TooManySubscribers,
            "INVALID_TARGET_VERSION" => AndroidRegistrationError::InvalidTargetVersion,
            "FIS_AUTH_ERROR" => AndroidRegistrationError::FisAuthentication,
            _ => AndroidRegistrationError::Other(code),
        }
    }
}
