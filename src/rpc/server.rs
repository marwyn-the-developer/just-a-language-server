use std::error::Error;

use futures::StreamExt;
use log::{debug, error};
use serde::Deserialize;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::FramedRead;

use crate::{
    lsp::{
        handler::Dispatcher,
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

pub struct JsonRpcServer<D, R, W>
where
    D: Dispatcher,
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    handler: D,
    reader: R,
    writer: W,
}

impl<D, R, W> JsonRpcServer<D, R, W>
where
    D: Dispatcher,
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    pub fn new(handler: D, reader: R, writer: W) -> Self {
        Self {
            handler,
            reader,
            writer,
        }
    }

    pub async fn start(mut self) -> Result<(), Box<dyn Error>> {
        let mut framed = FramedRead::new(self.reader, LspDecoder);
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

            if let Some(r) = res {
                let res_json = frame_message(r)?;
                self.writer.write_all(&res_json).await?;
                self.writer.flush().await?;
                debug!("Sent response");
            } else {
                debug!("Notification handled")
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::Cursor,
        sync::{Arc, Mutex},
        task::Poll,
    };

    use tokio::io::AsyncWrite;

    use crate::{
        lsp::{handler::Dispatcher, messages::core::Response},
        rpc::server::{JsonRpcServer, Message},
    };

    struct MockDispatcher;

    impl Dispatcher for MockDispatcher {
        fn dispatch(&self, message: &Message) -> Option<Response> {
            match message {
                Message::Request(r) => Some(Response {
                    jsonrpc: "2.0".to_string(),
                    id: Some(r.clone().id),
                    result: None,
                    error: None,
                }),
                Message::Notification(_) => None,
            }
        }
    }

    struct SpyWriter {
        buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl SpyWriter {
        pub fn new() -> (Self, Arc<Mutex<Vec<u8>>>) {
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let spy = Self {
                buffer: buffer.clone(),
            };
            (spy, buffer)
        }
    }

    impl AsyncWrite for SpyWriter {
        fn poll_write(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<Result<usize, std::io::Error>> {
            self.buffer.lock().unwrap().extend_from_slice(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), std::io::Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), std::io::Error>> {
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn test_handle_request() {
        let json_request =
            r#"{"jsonrpc":"2.0","id":"1234","method":"initialize","params":{"capabilities":{}}}"#;
        let incoming_msg = format!(
            "Content-Length: {}\r\n\r\n{}",
            json_request.len(),
            json_request
        );

        let input = Cursor::new(incoming_msg.into_bytes());
        let (output, buffer) = SpyWriter::new();
        let handler = MockDispatcher {};
        let server = JsonRpcServer::new(handler, input, output);
        server.start().await.unwrap();

        let result = buffer.lock().unwrap();

        let json_response = r#"{"jsonrpc":"2.0","id":"1234","result":null,"error":null}"#;
        let expected = format!(
            "Content-Length: {}\r\n\r\n{}",
            json_response.len(),
            json_response
        );

        assert_eq!(*result, expected.as_bytes());
    }

    #[tokio::test]
    async fn test_handle_notification() {
        let json_notification = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
        let incoming_msg = format!(
            "Content-Length: {}\r\n\r\n{}",
            json_notification.len(),
            json_notification
        );

        let input = Cursor::new(incoming_msg.into_bytes());
        let (output, buffer) = SpyWriter::new();
        let handler = MockDispatcher {};
        let server = JsonRpcServer::new(handler, input, output);
        server.start().await.unwrap();

        let result = buffer.lock().unwrap();
        let expected: Vec<u8> = Vec::new();

        assert_eq!(*result, expected);
    }
}
