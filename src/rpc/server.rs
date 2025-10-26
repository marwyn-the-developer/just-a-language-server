use std::error::Error;

use futures::StreamExt;
use log::{debug, error};
use serde::Deserialize;
use tokio::io::{stdin, stdout, AsyncWriteExt, BufWriter};
use tokio_util::codec::FramedRead;

use crate::{
    lsp::{
        handler::JustLspDispatcher,
        messages::core::{Notification, Request},
    },
    rpc::codec::{frame_message, unframe_message, LspDecoder},
};
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Notification(Notification),
}

#[derive(Default)]
pub struct JsonRpcServer {
    handler: JustLspDispatcher,
}

impl JsonRpcServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut framed = FramedRead::new(stdin, LspDecoder);
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);

        while let Some(message) = framed.next().await {
            let msg = match message {
                Ok(m) => m,
                Err(e) => {
                    error!("Error while retrieving message: {}", e);
                    continue;
                }
            };
            let content = match unframe_message(msg) {
                Ok(m) => m,
                Err(e) => {
                    error!("Error while reading message: {}", e);
                    continue;
                }
            };

            debug!("Request content: {}", String::from_utf8_lossy(&content));
            let msg: Message = serde_json::from_slice(&content)?;

            let res = self.handler.dispatch(&msg);

            let res_json = match frame_message(res) {
                Ok(m) => m,
                Err(e) => {
                    error!("Error while framing response: {}", e);
                    continue;
                }
            };
            debug!("Response content: {}", String::from_utf8_lossy(&res_json));
            writer.write_all(&res_json).await?;
            writer.flush().await?;
            debug!("Sent response");
        }

        Ok(())
    }
}
