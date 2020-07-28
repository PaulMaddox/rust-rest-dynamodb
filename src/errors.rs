use rocket::response::Responder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Responder)]
pub enum Error {
    #[response(status = 500, content_type = "plain")]
    InternalError(String),

    #[response(status = 404, content_type = "plain")]
    NotFoundError(String),

    #[response(status = 400, content_type = "plain")]
    BadRequestError(String),
}
