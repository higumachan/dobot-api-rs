use lazy_static::lazy_static;

use dobot_api::api::types::{PTPCmd, PTPCommonParams};
use dobot_api::api::Dobot;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use tokio::prelude::*;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let dobot = Dobot::connect_dobot(None, None, None, None).unwrap();
    dobot
        .start(
            async {
                dobot.set_queued_cmd_start_exec().await.unwrap();

                let cmd = PTPCmd {
                    ptp_mode: 0u8,
                    x: 300.13538,
                    y: -9.142999,
                    z: -70.29747,
                    r: -1.6940882,
                };
                dobot
                    .set_PTP_common_params(
                        PTPCommonParams {
                            velocity_ratio: 10.0,
                            acceleration_ratio: 10.0,
                        },
                        true,
                    )
                    .await
                    .unwrap();
                dobot.set_ptp_cmd(cmd, true).await.unwrap();
                let cmd = PTPCmd {
                    ptp_mode: 0u8,
                    x: 300.13538,
                    y: -0.142999,
                    z: -70.29747,
                    r: -1.6940882,
                };
                dobot
                    .set_PTP_common_params(
                        PTPCommonParams {
                            velocity_ratio: 100.0,
                            acceleration_ratio: 100.0,
                        },
                        true,
                    )
                    .await
                    .unwrap();
                dobot.set_ptp_cmd(cmd, true).await.unwrap();
            }
            .boxed(),
        )
        .await;
}
