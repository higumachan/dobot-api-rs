use crate::protocol::message::{ToParams, PARAMS_SIZE};
use derives::ToParams;
use std::io::{Write, Read};

trait ToParamable {
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
        buf[0] = if *self {
            1
        } else {
            0
        };
        1
    }
}

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


#[cfg(test)]
mod tests {
    use crate::api::types::{PTPCommonParams, EndEffectorSuctionCapParams};
    use crate::protocol::message::{ToParams, PARAMS_SIZE};

    #[test]
    fn test_ptp_common_params_to_params() {
        let params = PTPCommonParams{ velocity_ratio: 0.1, acceleration_ratio: 0.1 };

        let x = params.to_params().unwrap();
        assert_eq!(x.0, 8);
        let mut b = [0u8; PARAMS_SIZE];
        b[0] = params.velocity_ratio.to_le_bytes()[0];
        b[1] = params.velocity_ratio.to_le_bytes()[1];
        b[2] = params.velocity_ratio.to_le_bytes()[2];
        b[3] = params.velocity_ratio.to_le_bytes()[3];
        b[4] = params.acceleration_ratio.to_le_bytes()[0];
        b[5] = params.acceleration_ratio.to_le_bytes()[1];
        b[6] = params.acceleration_ratio.to_le_bytes()[2];
        b[7] = params.acceleration_ratio.to_le_bytes()[3];
        assert_eq!(x.1.to_vec(), b.to_vec());
    }

    #[test]
    fn test_end_effector_suction_cap() {
        let params = EndEffectorSuctionCapParams { enable_ctrl: true, suck: false };

        let x = params.to_params().unwrap();
        assert_eq!(x.0, 2);
        let mut b = [0u8; PARAMS_SIZE];
        b[0] = 1;
        b[1] = 0;
        assert_eq!(x.1.to_vec(), b.to_vec());
    }
}