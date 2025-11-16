use serde::{Deserialize, Serialize};

use crate::lsp::messages::{core::Notification, text_document::TextDocumentItem};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentNotification {
    #[serde(flatten)]
    pub notification: Notification,
    pub params: DidOpenTextDocumentParams,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
    pub text_document: TextDocumentItem,
}
