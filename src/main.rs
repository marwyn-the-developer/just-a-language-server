use bytes::Bytes;
use std::{error::Error, fs::File};
use tokio::io::{self, stdin, AsyncWriteExt, BufWriter};

use env_logger::Builder;
use futures::StreamExt;
use just_a_language_server::{
    lsp::initialize::{JustLspInitializeRequest, JustLspInitializeResponse},
    rpc::codec::{encode_message, BaseMessage, LspDecoder},
};
use log::{error, info};
use tokio_util::codec::FramedRead;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target = Box::new(File::create("lsp.log").expect("Can't create a log file"));

    Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("Just LSP starting");

    let stdin = stdin();

    let mut framed = FramedRead::new(stdin, LspDecoder);

    while let Some(message) = framed.next().await {
        let msg = message?;
        let result = BaseMessage::from_bytes(msg);
        let (msg, content) = match result {
            Ok(m) => m,
            Err(e) => {
                error!("Error while parsing message: {}", e);
                continue;
            }
        };
        info!(
            "Request content: {}",
            String::from_utf8(content.to_vec()).unwrap()
        );
        if let Err(e) = handle_message(msg, content).await {
            error!("Error while handling message: {}", e);
            continue;
        };
    }

    Ok(())
}

async fn handle_message(message: BaseMessage, content: Bytes) -> Result<(), Box<dyn Error>> {
    match message.method.as_str() {
        "initialize" => {
            let req: JustLspInitializeRequest = serde_json::from_slice(&content)?;
            if let Some(client_info) = req.params.client_info {
                match client_info.version {
                    Some(v) => info!("Connected to {} {}", client_info.name, v),
                    None => info!("Connected to {}", client_info.name),
                }
            }
            let stdout = io::stdout();
            let mut writer = BufWriter::new(stdout);

            let response = JustLspInitializeResponse::new(req.request.id.to_string());
            let response_json = encode_message(response)?;
            info!("Response content: {}", response_json);
            writer.write_all(response_json.as_bytes()).await?;
            writer.flush().await?;
            info!("Sent reply");
            Ok(())
        }
        _ => Err(format!("Unsupported method: {}", message.method).into()),
    }
}
