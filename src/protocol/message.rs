
use crate::protocol::packet::SYNC_BYTE;
use crate::protocol::protocol_id::ProtocolID;
use std::io::Read;
use nom::lib::std::fmt::{Debug, Formatter};

const MAX_PAYLOAD_SIZE: u8 = (SYNC_BYTE - 1);
pub const PARAMS_SIZE: usize = MAX_PAYLOAD_SIZE as usize - 2;

pub trait ToParams {
    fn to_params(&self) -> (usize, [u8; PARAMS_SIZE]);
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
    fn to_params(&self) -> (usize, [u8; PARAMS_SIZE]) {
        unimplemented!()
    }
}

impl Message {
    pub fn new<T: ToParams>(protocol_id: ProtocolID, rw: u8, is_queued: bool, params_value: &Option<T>) -> Self {
        let (size, params) = if let Some(p) = params_value {
            p.to_params()
        } else {
            (0, [0; PARAMS_SIZE])
        };

        Self {
            id: protocol_id as u8,
            rw,
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