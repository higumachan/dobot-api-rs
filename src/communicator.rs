use std::time::Duration;

use futures::channel::oneshot;
use nom::lib::std::collections::VecDeque;

use crate::connector::Connector;
use crate::protocol::message::Message;
use crate::protocol::packet::Packet;

const MAX_MESSAGES: usize = 128;

#[derive(Debug)]
pub enum CommunicateStatus {
    NoError(Message),
    BufferFull,
    Timeout,
    InvalidParams,
}

struct MessageHandler {
    message: Message,
    sender: oneshot::Sender<CommunicateStatus>,
}

pub struct Communicator {
    connector: Connector,
    message_handlers: VecDeque<MessageHandler>,
    left_space: usize,
    wait_time: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Control {
    Retry,
    Abort,
}

impl Communicator {
    pub fn new(connector: Connector, wait_time: Option<Duration>) -> Self {
        let wait_time = wait_time.unwrap_or(Duration::from_millis(500));
        Communicator {
            connector,
            message_handlers: VecDeque::new(),
            left_space: 128,
            wait_time,
        }
    }

    pub fn insert_message(&mut self, message: &Message) -> oneshot::Receiver<CommunicateStatus> {
        let (tx, rx) = oneshot::channel::<CommunicateStatus>();

        if self.message_handlers.len() > MAX_MESSAGES {
            tx.send(CommunicateStatus::BufferFull).unwrap();
            return rx;
        }
        self.message_handlers.push_back(MessageHandler {
            message: message.clone(),
            sender: tx,
        });
        rx
    }

    pub async fn run(&mut self) {
        let mh = self.message_handlers.pop_front();
        if mh.is_none() {
            return;
        }
        let mh = mh.unwrap();
        if mh.message.is_queued != 0 {
            let message = Message::new_get_left_space();
            let mut ctl = self.send_and_wait_command_ack(&message).await;
            let mut num_retry = 0i32;
            while check_retry(&ctl) && num_retry < 3 {
                ctl = self.send_and_wait_command_ack(&message).await;
                num_retry += 1;
            }
            if ctl.is_err() {
                mh.sender.send(CommunicateStatus::Timeout).unwrap();
                return; // このメッセージは終了
            }
        }
        let mut ctl = self.send_and_wait_command_ack(&mh.message).await;
        let mut num_retry = 0;
        while check_retry(&ctl) && num_retry < 3 {
            ctl = self.send_and_wait_command_ack(&mh.message).await;
            num_retry += 1;
        }
        if ctl.is_err() {
            mh.sender.send(CommunicateStatus::Timeout).unwrap();
            return; // このメッセージは終了
        }
        mh.sender
            .send(CommunicateStatus::NoError(ctl.unwrap()))
            .unwrap();
    }

    async fn send_and_wait_command_ack(&mut self, message: &Message) -> Result<Message, Control> {
        self.connector
            .write_packet(&Packet::from_message(message))
            .await
            .unwrap();

        match self
            .connector
            .read_packet_with_timeout(self.wait_time)
            .await
        {
            Some(packet) => {
                let mes = packet.to_message();
                if mes.id != message.id {
                    return Err(Control::Retry);
                }
                Ok(mes)
            }
            None => {
                eprintln!("timeout");
                Err(Control::Retry)
            }
        }
    }
}

fn check_retry<T>(ctl: &Result<T, Control>) -> bool {
    match ctl {
        Err(Control::Retry) => true,
        _ => false,
    }
}
