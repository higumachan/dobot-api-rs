use lazy_static::lazy_static;

use dobot_api::api::types::{EndEffectorSuctionCapState, HHTTrigMode, PTPCmd, PTPCommonParams};
use dobot_api::api::Dobot;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use std::time::Duration;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    let dobot = Dobot::connect_dobot(None, None, None, None).unwrap();
    let ptp_deck_position = PTPCmd {
        ptp_mode: 0u8,
        x: 300.13538,
        y: -0.142999,
        z: -70.29747,
        r: -1.6940882,
    };
    let ptp_card_position = PTPCmd {
        ptp_mode: 0u8,
        x: 193.74258,
        y: -20.302736,
        z: -60.709778,
        r: -5.9823236,
    };
    let ptp_relay1 = PTPCmd {
        ptp_mode: 0u8,
        x: 300.13538,
        y: -0.142999,
        z: 0.29747,
        r: -1.6940882,
    };
    let ptp_relay2 = PTPCmd {
        ptp_mode: 0u8,
        x: 193.74258,
        y: -20.302736,
        z: 0.0,
        r: -5.9823236,
    };
    dobot
        .start(
            async {
                loop {
                    dobot.set_queued_cmd_start_exec().await.unwrap();
                    let _ = dobot.set_ptp_cmd(ptp_relay1, true).await.unwrap().unwrap();
                    let index = dobot
                        .set_ptp_cmd(ptp_deck_position, true)
                        .await
                        .unwrap()
                        .unwrap();
                    dobot.wait_queued_command(index).await;
                    let index = dobot
                        .set_end_effector_suctions_cap(EndEffectorSuctionCapState::In, true)
                        .await
                        .unwrap()
                        .unwrap();
                    dobot.wait_queued_command(index).await;
                    let _ = dobot.set_ptp_cmd(ptp_relay1, true).await.unwrap().unwrap();
                    let _ = dobot.set_ptp_cmd(ptp_relay2, true).await.unwrap().unwrap();
                    let _ = dobot
                        .set_ptp_cmd(ptp_card_position, true)
                        .await
                        .unwrap()
                        .unwrap();
                    let index = dobot
                        .set_end_effector_suctions_cap(EndEffectorSuctionCapState::Out, true)
                        .await
                        .unwrap()
                        .unwrap();
                    dobot.wait_queued_command(index).await;
                }
            }
            .boxed(),
        )
        .await;
}
