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
    x_bias: f32,
    y_bias: f32,
    z_bias: f32,
}
