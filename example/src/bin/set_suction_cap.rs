use lazy_static::lazy_static;

use dobot_api::api::types::{HHTTrigMode, PTPCmd, PTPCommonParams};
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
    dobot
        .start(
            async {
                dobot.set_queued_cmd_start_exec().await.unwrap();
                dobot
                    .set_end_effector_suctions_cap(true, true, true)
                    .await
                    .unwrap();
                delay_for(Duration::from_secs(5)).await;
                dobot
                    .set_end_effector_suctions_cap(false, true, true)
                    .await
                    .unwrap();
            }
            .boxed(),
        )
        .await;
}
