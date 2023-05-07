use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

// impl std::fmt::Display for Error {
// fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Formatter> {
// write!(fmt, "{self:?}")
// match self {
// Error::LoginFail => write!(fmt, "LoginFail"),
// }
// }
// }

// impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED ERROR").into_response()
    }
}
