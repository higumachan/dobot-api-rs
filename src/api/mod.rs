use crate::api::types::PTPCmd;
use std::sync::{Arc};
use tokio::sync::RwLock;
use crate::communicator::{Communicator, CommunicateStatus};
use crate::protocol::message::Message;
use crate::protocol::protocol_id::ProtocolID;
use crate::api::DobotError::CommunicationError;
use serialport::SerialPortType::UsbPort;
use crate::connector::{ConnectorError, Connector};

pub mod types;

#[derive(Debug)]
pub enum DobotError {
    CommunicationError(CommunicateStatus),
    ConnectorError(ConnectorError),
    PortNotFound,
}

pub type Result<T> = std::result::Result<T, DobotError>;

pub struct Dobot {
    communicator: Arc<RwLock<Communicator>>,
}

impl Dobot {
    pub async fn start(&self) {
        self.communicator.write().await.run().await;
    }
    pub fn search_dobot() -> Vec<String> {
        let ports = serialport::available_ports().unwrap();

        ports.iter().filter_map(|x| {
            if let UsbPort(usb) = &x.port_type {
                let product = usb.product.as_ref()?;
                if product.contains("USB-to-Serial") ||
                    product.contains("CH340") ||
                    product.contains("CP210") ||
                    product.contains("USB2.0-Serial") ||
                    product.contains("USB Serial Port") {
                    return Some(x.port_name.clone())
                }
            }
            None
        }).collect()
    }

    pub fn connect_dobot(port_name: Option<&str>, boudrate: Option<u32>, fw_type: Option<&str>, version: Option<&str>) -> Result<Self> {
        let boudrate = boudrate.unwrap_or(115200);
        let port_name = port_name.map(|x| x.to_string()).unwrap_or(Self::search_dobot().first().ok_or(DobotError::PortNotFound)?.clone());


        Ok(Self {
            communicator: Arc::new(RwLock::new(Communicator::new(Arc::new(RwLock::new(Connector::connect(
                port_name.as_str(),
                boudrate,
                fw_type,
                version
            ).map_err(|e| DobotError::ConnectorError(e))?)), None)))
        })
    }

    pub fn disconnect_dobot(&self) {

    }

    pub async fn set_ptp_cmd(&self, ptp_cmd: PTPCmd, is_queued: bool) -> Result<Option<u64>>  {
        let mes = Message::new(
            ProtocolID::ProtocolPTPCmd,
            1,
            is_queued,
            &Some(ptp_cmd),
        );

        let status = self.communicator.write().await.insert_message(&mes).await.unwrap();
        if let CommunicateStatus::NoError(ack_mes) = status {
            if is_queued {
                Ok(Some(ack_mes.params[0] as u64))
            } else {
                Ok(None)
            }
        } else {
            Err(CommunicationError(status))
        }
    }

    pub async fn set_queued_cmd_start_exec(&self) -> Result<()> {
        let mes = Message::new::<()>(
            ProtocolID::ProtocolQueuedCmdStartExec,
            1,
            false,
            &None,
        );

        let status = self.communicator.write().await.insert_message(&mes).await.unwrap();
        match status {
            CommunicateStatus::NoError(_) => Ok(()),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }
}
