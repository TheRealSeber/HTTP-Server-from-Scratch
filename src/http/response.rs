use super::StatusCode;
use std::io::{Result as IoResult, Write};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    // we can pass any type that implements the Write trait
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.body.clone().unwrap_or_default()
        )
    }
}
