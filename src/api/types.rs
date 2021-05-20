use crate::protocol::message::ToParamable;
use crate::protocol::message::{FromParams, ToParams, PARAMS_SIZE};
use byteorder::{LittleEndian, ReadBytesExt};
use derives::ToParams;
use std::io::{Cursor, Write};

#[derive(Debug, Default, Copy, Clone, ToParams)]
pub struct PTPCmd {
    pub ptp_mode: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

#[derive(Debug, Default, Copy, Clone, ToParams)]
pub struct EndEffectorParams {
    pub x_bias: f32,
    pub y_bias: f32,
    pub z_bias: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum EndEffectorSuctionCapState {
    Off,
    In,
    Out,
}

impl Into<EndEffectorSuctionCapParams> for EndEffectorSuctionCapState {
    fn into(self) -> EndEffectorSuctionCapParams {
        match self {
            Self::Off => EndEffectorSuctionCapParams {
                enable_ctrl: false,
                suck: false,
            },
            Self::In => EndEffectorSuctionCapParams {
                enable_ctrl: true,
                suck: true,
            },
            Self::Out => EndEffectorSuctionCapParams {
                enable_ctrl: true,
                suck: false,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, ToParams)]
pub struct EndEffectorSuctionCapParams {
    pub enable_ctrl: bool,
    pub suck: bool,
}

#[derive(Debug, Default, Copy, Clone, ToParams)]
pub struct PTPCommonParams {
    pub velocity_ratio: f32,
    pub acceleration_ratio: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum HHTTrigMode {
    TriggeredOnKeyReleased,
    TriggeredOnPeriodicInterval,
}

impl ToParams for HHTTrigMode {
    fn to_params(&self) -> std::io::Result<(usize, [u8; PARAMS_SIZE])> {
        Ok((
            1,
            [match self {
                HHTTrigMode::TriggeredOnKeyReleased => 0u8,
                HHTTrigMode::TriggeredOnPeriodicInterval => 1u8,
            }; PARAMS_SIZE],
        ))
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub joint_angle: [f32; 4],
}

impl FromParams for Pose {
    fn from_params(size: usize, params: [u8; PARAMS_SIZE]) -> Self {
        let mut rdr = Cursor::new(&params);
        let x = rdr.read_f32::<LittleEndian>().unwrap();
        let y = rdr.read_f32::<LittleEndian>().unwrap();
        let z = rdr.read_f32::<LittleEndian>().unwrap();
        let r = rdr.read_f32::<LittleEndian>().unwrap();
        let (joint_angle1, joint_angle2, joint_angle3, joint_angle4) = (
            rdr.read_f32::<LittleEndian>().unwrap(),
            rdr.read_f32::<LittleEndian>().unwrap(),
            rdr.read_f32::<LittleEndian>().unwrap(),
            rdr.read_f32::<LittleEndian>().unwrap(),
        );

        Self {
            x,
            y,
            z,
            r,
            joint_angle: [joint_angle1, joint_angle2, joint_angle3, joint_angle4],
        }
    }
}
