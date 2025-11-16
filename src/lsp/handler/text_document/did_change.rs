use crate::lsp::analysis::state::State;

#[derive(Debug)]
pub struct TextDocumentDidChange {
    state: State,
}
