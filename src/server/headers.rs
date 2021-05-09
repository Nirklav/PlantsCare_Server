use std::str;
use std::fmt;

use hyper::header::{Header, Raw, Formatter};
use hyper::{Error, Result};

#[derive(Clone, Debug)]
pub struct ServerMethod {
    name: String
}

impl ServerMethod {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Header for ServerMethod {

    fn header_name() -> &'static str {
        "Server-Method"
    }

    fn parse_header(raw: &Raw) -> Result<Self> {
        if raw.len() == 1 {
            let line = &raw[0];
            let method = str::from_utf8(line)?;
            return Ok(ServerMethod {
                name: String::from(method)
            });
        }

        Err(Error::Header)
    }

    fn fmt_header(&self, f: &mut Formatter) -> fmt::Result {
        f.fmt_line(&format!("{}", self.name))
    }
}