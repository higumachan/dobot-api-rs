use std::io::{Write, Read, Result};
use crate::protocol::message::{ToParams, PARAMS_SIZE};


#[derive(Debug, Default, Copy, Clone)]
pub struct PTPCmd {
    pub ptp_mode: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}


impl ToParams for PTPCmd {
    fn to_params(&self) -> (usize, [u8; PARAMS_SIZE]) {
        let mut b = [0u8; PARAMS_SIZE];
        let mut size = 0;
        {
            let mut buf = &mut b[0..PARAMS_SIZE];
            size += buf.write(&[self.ptp_mode]).unwrap();
            size += buf.write(&self.x.to_le_bytes()).unwrap();
            size += buf.write(&self.y.to_le_bytes()).unwrap();
            size += buf.write(&self.z.to_le_bytes()).unwrap();
            size += buf.write(&self.r.to_le_bytes()).unwrap();
            buf.flush().unwrap();
        }
        (size, b)
    }
}
