use std::io::{Read, Write};
use std::net::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use serde_json;
use serde::{Serialize};
use serde::de::{DeserializeOwned};

use crate::server::server_error::{LogicError, ServerError};

pub struct Command {
    address: SocketAddr,
    method_id: Option<i32>,
    input: Option<Vec<u8>>,
}

impl Command {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, ServerError> {
        let mut addrs = addr.to_socket_addrs()?;
        let first = addrs.next().ok_or(LogicError::CommandSocketAddressNotFound)?;

        Ok(Command {
            address: first,
            method_id: None,
            input: None
        })
    }

    pub fn input<I: Serialize>(mut self, dto: I) -> Result<Self, ServerError> {
        self.input = Some(serde_json::to_vec(&dto)?);
        Ok(self)
    }

    pub fn method_id(mut self, id: i32) -> Self {
        self.method_id = Some(id);
        self
    }

    pub fn execute<O: DeserializeOwned>(self) -> Result<O, ServerError> {
        let mut stream = TcpStream::connect(self.address)?;
        let input = self.input.ok_or(LogicError::CommandInputNotSet)?;
        let method_id = self.method_id.ok_or(LogicError::CommandMethodIdNotSet)?;

        stream.write_i32::<LittleEndian>(method_id)?;
        stream.write_i32::<LittleEndian>(input.len() as i32)?;
        stream.write_i32::<LittleEndian>(1)?;
        stream.write(&input)?;

        let size = stream.read_i32::<LittleEndian>()? as usize;
        let content_type = stream.read_i32::<LittleEndian>()?;
        if content_type != 1 {
            return Err(LogicError::CommandUnsupportedContentType.into());
        }

        let mut buf = vec![0u8; size];
        stream.read_exact(&mut buf)?;
        let output = serde_json::from_slice::<O>(&buf)?;

        stream.shutdown(Shutdown::Both)?;

        Ok(output)
    }
}
