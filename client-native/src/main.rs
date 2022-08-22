use std::time::Duration;
use client_common::client_example;
use workflow_log::log_info;

#[tokio::main]
async fn main() {

    let result = client_example(Duration::from_millis(1000)).await;
    log_info!("{:#?}", result);

}

