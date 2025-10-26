use crate::lsp::messages::initialize::{
    InitializeParams, InitializeResult, ServerCapabilities, ServerInfo,
};

use log::info;

pub struct InitializeHandler;

impl InitializeHandler {
    pub fn handle(&self, params: &InitializeParams) -> InitializeResult {
        if let Some(client_info) = &params.client_info {
            match &client_info.version {
                Some(v) => info!("Connected to {} {}", client_info.name, v),
                None => info!("Connected to {}", client_info.name),
            }
        }

        InitializeResult {
            capabilities: ServerCapabilities::default(),
            server_info: Some(ServerInfo {
                name: String::from("just_a_language_server"),
                version: Some(String::from("0.0.0")),
            }),
        }
    }
}
