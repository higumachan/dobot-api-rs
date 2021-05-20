use crate::protocol::packet::SYNC_BYTE;
use crate::protocol::protocol_id::ProtocolID;

use byteorder::{LittleEndian, ReadBytesExt};
use nom::lib::std::fmt::{Debug, Formatter};
use std::io::{Cursor, Read, Write};

const MAX_PAYLOAD_SIZE: u8 = (SYNC_BYTE - 1);
pub const PARAMS_SIZE: usize = MAX_PAYLOAD_SIZE as usize - 2;

pub trait ToParams {
    fn to_params(&self) -> std::io::Result<(usize, [u8; PARAMS_SIZE])>;
}

pub trait ToParamable {
    fn to_params(&self, buf: &mut [u8]) -> usize;
}

impl ToParamable for f32 {
    fn to_params(&self, buf: &mut [u8]) -> usize {
        let a = self.to_le_bytes();
        a.as_ref().read(buf).unwrap()
    }
}

impl ToParamable for u8 {
    fn to_params(&self, buf: &mut [u8]) -> usize {
        buf[0] = *self;
        1
    }
}

impl ToParamable for bool {
    fn to_params(&self, buf: &mut [u8]) -> usize {
        buf[0] = if *self { 1 } else { 0 };
        1
    }
}

impl<T: ToParamable> ToParams for T {
    fn to_params(&self) -> std::io::Result<(usize, [u8; PARAMS_SIZE])> {
        let mut b = [0u8; PARAMS_SIZE];
        let mut size = 0;
        {
            let mut buf = &mut b[0..];
            size += ToParamable::to_params(self, buf);
            buf.flush()?
        }
        Ok((size, b))
    }
}

pub trait FromParams {
    fn from_params(size: usize, params: [u8; PARAMS_SIZE]) -> Self;
}

impl FromParams for u64 {
    fn from_params(size: usize, params: [u8; PARAMS_SIZE]) -> Self {
        let mut rdr = Cursor::new(&params);

        rdr.read_u64::<LittleEndian>().unwrap()
    }
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct Message {
    pub id: u8,
    pub rw: u8,
    pub is_queued: u8,
    pub params_len: u8,
    pub params: [u8; PARAMS_SIZE],
}

#[derive(Clone, Copy)]
pub enum ReadWrite {
    Read = 0,
    Write = 1,
}

impl Into<u8> for ReadWrite {
    fn into(self) -> u8 {
        match self {
            Self::Read => 0,
            Self::Write => 1,
        }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("id", &self.id)
            .field("rw", &self.rw)
            .field("is_queued", &self.is_queued)
            .field("params_len", &self.params_len)
            .finish()
    }
}

impl ToParams for () {
    fn to_params(&self) -> std::io::Result<(usize, [u8; PARAMS_SIZE])> {
        unreachable!()
    }
}

impl Message {
    pub fn new<T: ToParams>(
        protocol_id: ProtocolID,
        rw: ReadWrite,
        is_queued: bool,
        params_value: &Option<T>,
    ) -> Self {
        let (size, params) = if let Some(p) = params_value {
            p.to_params().unwrap()
        } else {
            (0, [0; PARAMS_SIZE])
        };

        Self {
            id: protocol_id as u8,
            rw: rw.into(),
            is_queued: if is_queued { 1u8 } else { 0u8 },
            params_len: size as u8,
            params,
        }
    }

    pub fn new_get_left_space() -> Self {
        Self {
            id: ProtocolID::ProtocolQueuedCmdLeftSpace as u8,
            rw: 0,
            is_queued: 0,
            params_len: 0,
            params: [0u8; PARAMS_SIZE],
        }
    }
}
