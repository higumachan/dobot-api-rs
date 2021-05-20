use lazy_static::lazy_static;

use dobot_api::api::types::{HHTTrigMode, PTPCmd, PTPCommonParams};
use dobot_api::api::Dobot;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use std::time::Duration;
use tokio::prelude::*;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let dobot = Dobot::connect_dobot(None, None, None, None).unwrap();
    dobot
        .start(
            async {
                dobot
                    .set_hht_trig_mode(HHTTrigMode::TriggeredOnPeriodicInterval)
                    .await
                    .unwrap();
                dobot.set_hht_trig_output_enabled(true).await.unwrap();

                loop {
                    let pose = dobot.get_pose().await.unwrap();
                    println!("{:?}", pose);
                    tokio::time::delay_for(Duration::from_secs(1)).await;
                }
            }
            .boxed(),
        )
        .await;
}
