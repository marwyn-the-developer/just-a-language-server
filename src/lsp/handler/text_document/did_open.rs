use log::info;

use crate::lsp::messages::text_document::did_open::DidOpenTextDocumentParams;

#[derive(Default)]
pub struct TextDocumentDidOpenHandler;

impl TextDocumentDidOpenHandler {
    pub fn handle(&self, params: &DidOpenTextDocumentParams) {
        info!(
            "Opened: {} {}",
            params.text_document.uri, params.text_document.text
        )
    }
}
