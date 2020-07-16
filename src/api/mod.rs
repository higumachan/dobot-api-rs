use crate::api::types::{PTPCmd, EndEffectorParams};
use std::sync::{Arc};
use tokio::sync::RwLock;
use crate::communicator::{Communicator, CommunicateStatus};
use crate::protocol::message::Message;
use crate::protocol::protocol_id::ProtocolID;
use crate::api::DobotError::CommunicationError;
use serialport::SerialPortType::UsbPort;
use crate::connector::{ConnectorError, Connector};
use std::time::Duration;
use tokio::time::delay_for;
use futures::Future;
use futures::{select, pin_mut};
use futures::future::{Fuse, FusedFuture, FutureExt, BoxFuture};

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

type ResultQueueIndex = Result<Option<u64>>;

impl Dobot {
    pub async fn start(&self, dobot_main_future: BoxFuture<'_, ()>) where
    {
        let dm = dobot_main_future.fuse();
        let start = self.start_communicator_loop().fuse();

        pin_mut!(start, dm);

        loop {
            select! {
                () = start => {
                },
                () = dm => {
                    break;
                },
            }
        }
    }

    async fn start_communicator_loop(&self) {
        loop {
            self.communicator.write().await.run().await;
            delay_for(Duration::from_millis(10)).await
        }
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
            communicator: Arc::new(RwLock::new(Communicator::new(Connector::connect(
                port_name.as_str(),
                boudrate,
                fw_type,
                version
            ).map_err(|e| DobotError::ConnectorError(e))?, None)))
        })
    }

    pub fn disconnect_dobot(&self) {

    }

    pub async fn set_end_effector_params(&self, end_effector_params: EndEffectorParams, is_queued: bool) -> ResultQueueIndex {
        let mes = Message::new(
            ProtocolID::ProtocolEndEffectorParams,
            1,
            is_queued,
            &Some(end_effector_params),
        );

        self.send_command_message(&mes).await
    }

    pub async fn set_ptp_cmd(&self, ptp_cmd: PTPCmd, is_queued: bool) -> ResultQueueIndex {
        let mes = Message::new(
            ProtocolID::ProtocolPTPCmd,
            1,
            is_queued,
            &Some(ptp_cmd),
        );
        self.send_command_message(&mes).await
    }

    pub async fn set_queued_cmd_start_exec(&self) -> Result<()> {
        let mes = Message::new::<()>(
            ProtocolID::ProtocolQueuedCmdStartExec,
            1,
            false,
            &None,
        );

        let status_recv = {
            self.communicator.write().await.insert_message(&mes)
        };
        let status = status_recv.await.unwrap();
        match status {
            CommunicateStatus::NoError(_) => Ok(()),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    async fn send_command_message(&self, message: &Message) -> Result<Option<u64>> {
        let status_recv = {
            self.communicator.write().await.insert_message(message)
        };
        let status = status_recv.await.unwrap();
        if let CommunicateStatus::NoError(ack_mes) = status {
            if message.is_queued != 0 {
                Ok(Some(ack_mes.params[0] as u64))
            } else {
                Ok(None)
            }
        } else {
            Err(CommunicationError(status))
        }
    }
}
