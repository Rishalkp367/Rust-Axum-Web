use axum::{ http::{ StatusCode }, response::{ IntoResponse, Response } };

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    LoginFail,

    TicketDeleteFailIdNotFound {
        id: u64,
    }, 
}

//Error Boilerplate
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - ERROR - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
    }
}
