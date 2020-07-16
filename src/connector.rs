use futures::prelude::*;
use serialport::{SerialPort, open_with_settings, SerialPortSettings, DataBits, Parity, StopBits, FlowControl, Error};
use crate::protocol::packet::{Packet, MAX_PACKET_SIZE};
use tokio::time::{delay_for, timeout};
use std::time::Duration;
use serialport::posix::TTYPort;
use tokio::sync::RwLock;
use std::path::{Path};
use std::io::{Write, Read};
use std::sync::Arc;

pub struct Connector {
    io_device: Arc<RwLock<TTYPort>>,
    red_bytes: Vec<u8>,
}


#[derive(Debug)]
pub enum ConnectorError {
    SerialPortError(Error)
}

type Result<T> = std::result::Result<T, ConnectorError>;

impl Connector {
    pub fn connect(port_name: &str, boudrate: u32, fw_type: Option<&str>, version: Option<&str>) -> Result<Self> {
        // TODO(higumachan): UDP Connect and checking fw and version

        let settings = SerialPortSettings{
            baud_rate: boudrate,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(0),
            flow_control: FlowControl::None,
        };
        Ok(Self {
            io_device: Arc::new(RwLock::new((TTYPort::open(&Path::new(port_name), &settings).map_err(|e| ConnectorError::SerialPortError(e))?))),
            red_bytes: vec![],
        })
    }

    pub async fn read_packet(&mut self) -> Packet {
        loop {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            let size = self.io_device.write().await.read(&mut buf);
            if size.is_err() {
                delay_for(Duration::from_millis(10)).await;
                continue;
            }
            let size = size.unwrap();
            self.red_bytes.extend(buf[0..size].iter());
            let packet = Packet::from_bytes_impl(&self.red_bytes);
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
        self.io_device.write().await.write(&buf)
    }
}