use std::io::{IoResult, File, MemReader};
use std::path::BytesContainer;

use hyper::header::Headers;
use hyper::status;
use hyper::status::StatusCode;
use anymap::AnyMap;

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Option<Box<Reader + Send>>,
    pub ext: AnyMap
}

impl Response {

    pub fn from_reader(status: StatusCode, body: Box<Reader + Send>) -> Response {
        Response {
            status: status,
            headers: Headers::new(),
            body: Some(body),
            ext: AnyMap::new()
        }
    }

    pub fn from_string(status: StatusCode, body: String) -> Response {
        Response::from_reader(status, box MemReader::new(body.into_bytes()) as Box<Reader + Send>)
    }

    pub fn from_file(path: &Path) -> IoResult<Response> {
        let file = try!(File::open(path));
        let mut response = Response::from_reader(
            status::Ok,
            box file as Box<Reader + Send>
        );
        // TODO: content_type
        Ok(response)
    }

}