use std::{time::Duration, ops::DerefMut};

use workflow_log::*;
use workflow_websocket::client::{
    WebSocket, Settings, Result, Message, DispatchMessage,
    DispatchStrategy
};

#[tokio::main]
async fn main() -> Result<()> {

    // let settings = Settings {
    //     strategy : DispatchStrategy::Ack
    // };
    let ws = WebSocket::new("ws://localhost:9090", Settings::default())?;
    let ws = WebSocket::new("ws://localhost:9090", Settings::default())?;

    ws.connect().await?;

    // let tx = ws.dispatcher_tx.clone();

    let ws_ = ws.clone();
    workflow_core::task::spawn(async move {
        let mut seq = 0;
        loop {
            log_trace!("send loop starting {}" ,seq);
            let msg = format!("hello world {}", seq).into();
            ws_.send(Message::Text(msg)).await.expect("error sending message ");
            // tokio::time::sleep(Duration::from_millis(0)).await;
            // workflow_core::task::yield_now().await;
            log_trace!("send loop iterating {}", seq);
            seq += 1;
        }
        log_trace!("send loop exit");
    });


    let ws_ = ws.clone();
    // tokio::task::spawn_blocking(|| {
    //     async move {

            // let mut receiver_rx = ws_.receiver_rx.lock();
    loop {
        log_trace!("loop running");
        // let message = receiver_rx.deref_mut().recv().await.unwrap();
        let message = ws_.receiver_rx.recv().await.unwrap();
        log_info!("receiving message: {:?}", message);
    }
            log_trace!("loop exit");
    Ok(())
}

