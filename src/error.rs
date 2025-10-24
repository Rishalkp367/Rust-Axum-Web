use axum::{ http::{ StatusCode }, response::{ IntoResponse, Response } };
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    //Auth
    AuthFailNoTokenCookie,
    AuthFailTokenNoFormat,
    AuthFailCtxNotInRequestExt,

    //Model
    TicketDeleteFailIdNotFound {
        id: u64,
    },
}

//Error Boilerplate
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - ERROR - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            | Self::AuthFailNoTokenCookie
            | Self::AuthFailTokenNoFormat
            | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            // Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
        }
    }
}
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
