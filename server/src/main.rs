use std::net::SocketAddr;
use std::sync::Arc;
use tungstenite::Message;
use async_trait::async_trait;
use tokio::sync::mpsc::*;
use workflow_websocket::server::WebSocketHandler;
use workflow_websocket::server::WebSocketServer;
use workflow_websocket::server::Result;
use workflow_log::*;

// Struct representing a websocket connection
pub struct MyContext {
    pub peer : SocketAddr,
}

// A simple WebSocket handler struct
pub struct MyWsHandler;

#[async_trait]
impl WebSocketHandler for MyWsHandler {
    type Context = Arc<MyContext>;

    // store peer address for each connection into context
    async fn connect(self : &Arc<Self>, peer: SocketAddr) -> Result<Self::Context> {
        let ctx = MyContext { peer };
        Ok(Arc::new(ctx))
    }

    // receive and echo text and binary messages 
    // while logging the ip address and received data
    async fn message(self : &Arc<Self>, ctx : &Self::Context, msg : Message, sink : &UnboundedSender<tungstenite::Message>) -> Result<()> {
        match &msg {
            Message::Binary(data) => {
                log_trace!("[{}] {:?}",ctx.peer, data);
                sink.send(msg)?;
            },            
            Message::Text(text) => {
                log_trace!("[{}] {}",ctx.peer, text);
                sink.send(msg)?;
            },
            _ => { },
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let addr = "127.0.0.1:9090";
    log_info!("WebSocket Server is listening on {}", addr);

    // create our handler instance
    let handler = Arc::new(MyWsHandler { });
    // create websocket server and install our handler in it
    let ws = WebSocketServer::<MyWsHandler>::new(handler);
    // listen for incoming connections
    ws.listen(addr).await?;

    Ok(())
}
