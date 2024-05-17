/// Represents the type of a request.
#[derive(Clone, Default)]
pub enum RequestKind {
    /// The request body will have only the JSON string.
    #[default]
    Standart,
    /// The request body will have the body length put before the JSON string.
    /// Useful when sending a request to the Bonfire server.
    Bonfire,
}
