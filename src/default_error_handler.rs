use hyper::status::StatusCode::{NotFound, BadRequest, InternalServerError};
use request::Request;
use response::Response;
use ResponseFinalizer;
use middleware::{ErrorHandler, MiddlewareResult};
use nickel_error::{NickelError, ErrorWithStatusCode};

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn invoke<'a, 'b>(&self, err: &NickelError, _req: &mut Request, res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        let r = match err.kind {
            ErrorWithStatusCode(NotFound) => (NotFound, "Not Found"),
            ErrorWithStatusCode(BadRequest) => (BadRequest, "Bad Request"),
            _ => (InternalServerError, "Internal Server Error")
        };

        r.respond(res)
    }
}
