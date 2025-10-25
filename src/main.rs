use std::{error::Error, fs::File};

use env_logger::Builder;
use just_a_language_server::rpc::server::JsonRpcServer;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target = Box::new(File::create("lsp.log").expect("Can't create a log file"));

    Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter_level(log::LevelFilter::Debug)
        .init();

    let server = JsonRpcServer::default();

    info!("Just LSP starting");

    if let Err(e) = server.start().await {
        error!("Error while running server: {}", e);
        return Err(e);
    }

    Ok(())
}
