use crate::protocol::message::{ToParams, PARAMS_SIZE};
use derives::ToParams;
use std::io::Write;

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
