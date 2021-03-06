use crate::api::types::{
    EndEffectorParams, EndEffectorSuctionCapParams, EndEffectorSuctionCapState, HHTTrigMode,
    PTPCmd, PTPCommonParams, Pose,
};
use crate::api::DobotError::CommunicationError;
use crate::communicator::{CommunicateStatus, Communicator};
use crate::connector::{Connector, ConnectorError};
use crate::protocol::message::{FromParams, Message, ReadWrite};
use crate::protocol::protocol_id::ProtocolID;
use futures::channel::oneshot;
use futures::future::join_all;
use serialport::SerialPortType::UsbPort;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::delay_for;

use futures::future::{BoxFuture, FutureExt};
use futures::{pin_mut, select};

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
    checking_queue_indices: Arc<RwLock<Vec<(QueueIndex, oneshot::Sender<QueueIndex>)>>>,
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct QueueIndex(u64);

type ResultQueueIndex = Result<Option<QueueIndex>>;

impl Dobot {
    pub async fn start(&self, dobot_main_future: BoxFuture<'_, ()>) {
        let dm = dobot_main_future.fuse();
        let start = self.start_communicator_loop().fuse();
        let cq = self.check_queue_index_loop().fuse();

        pin_mut!(start, dm, cq);

        loop {
            select! {
                () = start => {
                },
                () = cq => {
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
            delay_for(Duration::from_millis(10)).await;
        }
    }

    async fn check_queue_index_loop(&self) {
        loop {
            let queue_index = self
                .get_queue_index()
                .await
                .expect("some error when get_queue_index");

            let mut chi = self.checking_queue_indices.write().await;
            let mut i = 0;

            while i < chi.len() {
                if queue_index >= chi[i].0 {
                    let c = chi.remove(i);
                    c.1.send(queue_index);
                } else {
                    i += 1;
                }
            }
            delay_for(Duration::from_millis(10)).await;
        }
    }

    pub async fn wait_queued_command(&self, index: QueueIndex) {
        let (tx, rx) = oneshot::channel::<QueueIndex>();
        self.checking_queue_indices.write().await.push((index, tx));
        rx.await;
    }

    pub async fn wait_queued_commands(&self, indices: &Vec<QueueIndex>) {
        join_all(indices.iter().map(|x| self.wait_queued_command(*x))).await;
    }

    pub async fn get_queue_index(&self) -> Result<QueueIndex> {
        let mes = Message::new::<()>(
            ProtocolID::ProtocolQueuedCmdCurrentIndex,
            ReadWrite::Read,
            false,
            &None,
        );

        let status = self.send_command_message_and_wait_execution(&mes).await;

        match status {
            CommunicateStatus::NoError(message) => Ok(QueueIndex(u64::from_params(
                message.params_len as usize,
                message.params,
            ))),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    pub fn search_dobot() -> Vec<String> {
        let ports = serialport::available_ports().unwrap();

        ports
            .iter()
            .filter_map(|x| {
                if let UsbPort(usb) = &x.port_type {
                    let product = usb.product.as_ref()?;
                    if product.contains("USB-to-Serial")
                        || product.contains("CH340")
                        || product.contains("CP210")
                        || product.contains("USB2.0-Serial")
                        || product.contains("USB Serial Port")
                    {
                        return Some(x.port_name.clone());
                    }
                }
                None
            })
            .collect()
    }

    pub fn connect_dobot(
        port_name: Option<&str>,
        boudrate: Option<u32>,
        fw_type: Option<&str>,
        version: Option<&str>,
    ) -> Result<Self> {
        let boudrate = boudrate.unwrap_or(115200);
        let port_name = port_name.map(|x| x.to_string()).unwrap_or(
            Self::search_dobot()
                .first()
                .ok_or(DobotError::PortNotFound)?
                .clone(),
        );

        Ok(Self {
            communicator: Arc::new(RwLock::new(Communicator::new(
                Connector::connect(port_name.as_str(), boudrate, fw_type, version)
                    .map_err(|e| DobotError::ConnectorError(e))?,
                None,
            ))),
            checking_queue_indices: Arc::new(RwLock::new(vec![])),
        })
    }

    pub fn disconnect_dobot(&self) {}

    pub async fn set_end_effector_params(
        &self,
        end_effector_params: EndEffectorParams,
        is_queued: bool,
    ) -> ResultQueueIndex {
        let mes = Message::new(
            ProtocolID::ProtocolEndEffectorParams,
            ReadWrite::Write,
            is_queued,
            &Some(end_effector_params),
        );

        self.send_command_message(&mes).await
    }

    pub async fn set_PTP_common_params(
        &self,
        params: PTPCommonParams,
        is_queued: bool,
    ) -> ResultQueueIndex {
        let mes = Message::new(
            ProtocolID::ProtocolPTPCommonParams,
            ReadWrite::Write,
            is_queued,
            &Some(params),
        );
        self.send_command_message(&mes).await
    }

    pub async fn set_ptp_cmd(&self, ptp_cmd: PTPCmd, is_queued: bool) -> ResultQueueIndex {
        let mes = Message::new(
            ProtocolID::ProtocolPTPCmd,
            ReadWrite::Write,
            is_queued,
            &Some(ptp_cmd),
        );
        self.send_command_message(&mes).await
    }

    pub async fn set_queued_cmd_start_exec(&self) -> Result<()> {
        let mes = Message::new::<()>(
            ProtocolID::ProtocolQueuedCmdStartExec,
            ReadWrite::Write,
            false,
            &None,
        );

        let status_recv = { self.communicator.write().await.insert_message(&mes) };
        let status = status_recv.await.unwrap();
        match status {
            CommunicateStatus::NoError(_) => Ok(()),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    pub async fn set_end_effector_suctions_cap(
        &self,
        suctions_cap_state: EndEffectorSuctionCapState,
        is_queued: bool,
    ) -> ResultQueueIndex {
        let mes = Message::new::<EndEffectorSuctionCapParams>(
            ProtocolID::ProtocolEndEffectorSuctionCup,
            ReadWrite::Write,
            is_queued,
            &Some(suctions_cap_state.into()),
        );
        self.send_command_message(&mes).await
    }

    pub async fn set_hht_trig_mode(&self, mode: HHTTrigMode) -> Result<()> {
        let mes = Message::new(
            ProtocolID::ProtocolHHTTrigMode,
            ReadWrite::Write,
            false,
            &Some(mode),
        );

        let status = self.send_command_message_and_wait_execution(&mes).await;

        match status {
            CommunicateStatus::NoError(_) => Ok(()),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    pub async fn set_hht_trig_output_enabled(&self, is_enabled: bool) -> Result<()> {
        let mes = Message::new(
            ProtocolID::ProtocolHHTTrigOutputEnabled,
            ReadWrite::Write,
            false,
            &Some(is_enabled),
        );

        let status = self.send_command_message_and_wait_execution(&mes).await;

        match status {
            CommunicateStatus::NoError(_) => Ok(()),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    pub async fn get_pose(&self) -> Result<Pose> {
        let mes = Message::new::<()>(ProtocolID::ProtocolGetPose, ReadWrite::Read, false, &None);

        let status = self.send_command_message_and_wait_execution(&mes).await;

        match status {
            CommunicateStatus::NoError(message) => Ok(Pose::from_params(
                message.params_len as usize,
                message.params,
            )),
            _ => Err(DobotError::CommunicationError(status)),
        }
    }

    async fn send_command_message(&self, message: &Message) -> ResultQueueIndex {
        let status_recv = { self.communicator.write().await.insert_message(message) };
        let status = status_recv.await.unwrap();
        if let CommunicateStatus::NoError(ack_mes) = status {
            if message.is_queued != 0 {
                Ok(Some(QueueIndex(ack_mes.params[0] as u64)))
            } else {
                Ok(None)
            }
        } else {
            Err(CommunicationError(status))
        }
    }

    async fn send_command_message_and_wait_execution(
        &self,
        message: &Message,
    ) -> CommunicateStatus {
        let status_recv = { self.communicator.write().await.insert_message(message) };
        status_recv.await.unwrap()
    }
}
