use log::info;

use crate::lsp::{
    analysis::state::State, messages::text_document::did_open::DidOpenTextDocumentParams,
};

#[derive(Default)]
pub struct TextDocumentDidOpenHandler {
    state: State,
}

impl TextDocumentDidOpenHandler {
    pub fn new(state: State) -> TextDocumentDidOpenHandler {
        TextDocumentDidOpenHandler { state }
    }

    pub fn handle(&self, params: &DidOpenTextDocumentParams) {
        info!(
            "Opened: {} {}",
            params.text_document.uri, params.text_document.text
        );
        self.state.open_document(
            params.text_document.uri.clone(),
            params.text_document.text.clone(),
        );
    }
}
