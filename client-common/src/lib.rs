use std::time::Duration;

use workflow_log::*;
use workflow_websocket::client::{
    WebSocket, Result, Message, Settings
};

pub async fn client_example(message_delay: Duration) -> Result<()> {

    let ws = WebSocket::new("ws://localhost:9090", Settings::default())?;
    ws.connect(true).await?;

    let ws_ = ws.clone();
    workflow_core::task::spawn(async move {
        let mut seq = 0;
        loop {
            log_info!("▷ sending message {}" ,seq);
            let msg = format!("message {}", seq).into();
            // let result = ws_.post(Message::Text(msg)).await;;
            let result = ws_.send(Message::Text(msg)).await;
            match result {
                Ok(_) => {  },
                Err(err) => {
                    log_error!("Error sending message: {}", err);
                }
            }

            // workflow_core::task::sleep(Duration::from_millis(1000)).await;
            workflow_core::task::sleep(message_delay).await;

            seq += 1;
        }
    });


    let ws_ = ws.clone();
    loop {
        let message = ws_.recv().await.unwrap();
        log_info!("◁ receiving message: {:?}", message);
    }

    // Ok(())
}

