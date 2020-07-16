use lazy_static::lazy_static;

use dobot_api::api::Dobot;
use dobot_api::api::types::PTPCmd;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use futures::future::BoxFuture;
use futures::future::{FutureExt};


#[tokio::main]
async fn main() {
    let dobot = Dobot::connect_dobot(None, None, None, None).unwrap();
    dobot.start(async {
        dobot.set_queued_cmd_start_exec().await.unwrap();
        let cmd = PTPCmd {
            ptp_mode: 0u8,
            x: 300.13538,
            y: -9.142999,
            z: -70.29747,
            r: -1.6940882,
        };
        dobot.set_ptp_cmd(cmd, true).await.unwrap();
    }.boxed()).await;
}
