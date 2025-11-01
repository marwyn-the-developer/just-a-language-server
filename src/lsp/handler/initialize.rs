use crate::lsp::messages::initialize::{
    InitializeParams, InitializeResult, ServerCapabilities, ServerInfo, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};

use log::info;

pub struct InitializeHandler;

impl InitializeHandler {
    pub fn handle(params: &InitializeParams) -> InitializeResult {
        if let Some(client_info) = &params.client_info {
            match &client_info.version {
                Some(v) => info!("Connected to {} {}", client_info.name, v),
                None => info!("Connected to {}", client_info.name),
            }
        }

        InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                code_action_provider: None,
                code_lens_provider: None,
                completion_provider: None,
                definition_provider: None,
                hover_provider: None,
                document_formatting_provider: None,
                document_highlight_provider: None,
                document_range_formatting_provider: None,
                document_symbol_provider: None,
                execute_command_provider: None,
                experimental: None,
                folding_range_provider: None,
                implementation_provider: None,
                references_provider: None,
                rename_provider: None,
                semantic_tokens_provider: None,
                signature_help_provider: None,
                type_definition_provider: None,
                workspace: None,
                workspace_symbol_provider: None,
            },
            server_info: Some(ServerInfo {
                name: String::from("just_a_language_server"),
                version: Some(String::from("0.0.0")),
            }),
        }
    }
}
