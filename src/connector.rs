use crate::protocol::packet::{Packet, MAX_PACKET_SIZE};
use serialport::posix::TTYPort;
use serialport::{DataBits, Error, FlowControl, Parity, SerialPortSettings, StopBits};
use std::time::Duration;
use tokio::time::{delay_for, timeout};

use std::io::{Read, Write};
use std::path::Path;

pub struct Connector {
    io_device: TTYPort,
    red_bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum ConnectorError {
    SerialPortError(Error),
}

type Result<T> = std::result::Result<T, ConnectorError>;

impl Connector {
    pub fn connect(
        port_name: &str,
        boudrate: u32,
        _fw_type: Option<&str>,
        _version: Option<&str>,
    ) -> Result<Self> {
        // TODO(higumachan): UDP Connect and checking fw and version

        let settings = SerialPortSettings {
            baud_rate: boudrate,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(0),
            flow_control: FlowControl::None,
        };
        Ok(Self {
            io_device: TTYPort::open(&Path::new(port_name), &settings)
                .map_err(|e| ConnectorError::SerialPortError(e))?,
            red_bytes: vec![],
        })
    }

    pub async fn read_packet(&mut self) -> Packet {
        loop {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            let size = self.io_device.read(&mut buf);
            if size.is_err() {
                delay_for(Duration::from_millis(10)).await;
                continue;
            }
            let size = size.unwrap();
            self.red_bytes.extend(buf[0..size].iter());
            let packet = Packet::from_bytes(&self.red_bytes);
            if let Ok((remain, packet)) = packet {
                self.red_bytes = Vec::from(remain);
                return packet;
            }
            delay_for(Duration::from_millis(10)).await;
        }
    }

    pub async fn read_packet_with_timeout(&mut self, wait_duration: Duration) -> Option<Packet> {
        timeout(wait_duration, self.read_packet()).await.ok()
    }

    pub async fn write_packet(&mut self, packet: &Packet) -> std::io::Result<usize> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        packet.to_bytes(&mut buf)?;
        self.io_device.write(&buf)
    }
}
