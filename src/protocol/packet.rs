use std::time::Duration;
use std::io::{Read, Write};
use std::thread::sleep;
use nom::bytes::complete::take;
use nom::sequence::tuple;
use nom::combinator::{map, all_consuming};
use nom::error::ParseError;
use nom::IResult;
use crate::protocol::message::{Message, PARAMS_SIZE};


pub const SYNC_BYTE:u8 = 0xAA;
pub const MAX_PACKET_SIZE:usize = 256usize;

#[derive(Debug)]
pub struct PacketHeader {
    sync_bytes: [u8; 2],
    payload_len: u8,
}

#[derive(Debug)]
pub struct PacketPayload {
    id: u8,
    ctrl: u8,
    params: Vec<u8>,
}


#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
    pub checksum: u8,
}



impl Packet {
    pub fn from_message(message: &Message) -> Self {
        let header = PacketHeader{
            sync_bytes: [SYNC_BYTE, SYNC_BYTE],
            payload_len: message.params_len + 2,
        };
        let payload = PacketPayload{
            id: message.id,
            ctrl: ((message.rw & 0x01) | ((message.is_queued << 1) & 0x02)),
            params: message.params.to_vec(),
        };

        let checksum = Self::checksum(&header, &payload);
        Packet {
            header,
            payload,
            checksum,
        }
    }

    pub fn to_message(&self) -> Message {
        let mut params = [0u8; PARAMS_SIZE];
        let params_len = (self.header.payload_len - 2) as usize;
        let mut params_buf = &mut params[..PARAMS_SIZE];
        params_buf.write(&self.payload.params);
        Message {
            id: self.payload.id,
            rw: self.payload.ctrl & 0x01,
            is_queued: (self.payload.ctrl >> 1) & 0x01,
            params_len: params_len as u8,
            params,
        }
    }

    fn checksum(header: &PacketHeader, payload: &PacketPayload) -> u8 {
        let mut sum = 0u8;

        sum = sum.wrapping_add(payload.id);
        sum = sum.wrapping_add(payload.ctrl);
        for i in 0..((header.payload_len - 2) as usize) {
            sum = sum.wrapping_add(payload.params[i]);
        }
        0u8.wrapping_sub(sum) as u8
    }

    pub fn to_bytes(&self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let mut size = 0usize;
        size += buf.write(&[self.header.sync_bytes[0], self.header.sync_bytes[1], self.header.payload_len])?;
        size += buf.write(&[self.payload.id, self.payload.ctrl])?;
        size += buf.write(&self.payload.params[0..(self.header.payload_len as usize - 2)])?;
        size += buf.write(&[self.checksum])?;
        Ok(size)
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        let (_, packet) = Self::from_bytes_impl(buf).ok()?;

        let calc_checksum = Self::checksum(&packet.header, &packet.payload);

        if packet.checksum != calc_checksum {
            return None;
        }

        Some(packet)
    }

    pub fn from_bytes_impl(input: &[u8]) -> IResult<&[u8], Packet> {
        let (remain, header) = map(
            tuple((
                take(1usize),
                take(1usize),
                take(1usize),
            )
            ), |(x1, x2, x3): (&[u8], _, _)| {
                PacketHeader { sync_bytes: [x1[0], x2[0]], payload_len: x3[0] }
            }
        )(input)?;

        let (remain, payload) = map(tuple((
            take(1usize),
            take(1usize),
            take(header.payload_len - 2),
        )), |(id, ctrl, params): (&[u8], _, _)| {
            PacketPayload { id: id[0], ctrl: ctrl[0], params: params.to_vec()}
        })(remain)?;

        let (remain, checksum) = map(
            take(1usize), |x: &[u8] | x[0]
        )(remain)?;

        Ok((remain, Packet {
            header,
            payload,
            checksum,
        }))
    }
}
