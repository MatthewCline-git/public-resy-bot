use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ResyError {
    NoAvailableReservations,
    CannotFindReservation,
    UnknownError(String),
}

// Implement Display for ResyError
impl fmt::Display for ResyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResyError::NoAvailableReservations => write!(f, "No available reservations"),
            ResyError::CannotFindReservation => write!(f, "Cannot find reservation"),
            ResyError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

// Implement Error trait
impl StdError for ResyError {}

// Implement From<anyhow::Error> for ResyError
impl From<anyhow::Error> for ResyError {
    fn from(err: anyhow::Error) -> Self {
        ResyError::UnknownError(err.to_string())
    }
}

#[derive(Debug)]
pub struct BookingDetails {
    pub payment_method_id: i32,
    pub booking_token: String,
}
