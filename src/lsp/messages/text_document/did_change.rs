use serde::{Deserialize, Serialize};

use crate::lsp::messages::{core::Notification, text_document::VersionedTextDocumentIdentifier};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentNotification {
    #[serde(flatten)]
    pub notification: Notification,
    pub params: DidChangeTextDocumentParams,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentContentChangeEvent {
    Incremental(IncrementalTextDocumentContentChangeEvent),
    Full(FullTextDocumentContentChangeEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalTextDocumentContentChangeEvent {
    /**
     * The range of the document that changed.
     */
    pub range: Range,

    /**
     * The optional length of the range that got replaced.
     *
     * @deprecated use range instead.
     */
    pub range_length: Option<u32>,

    /**
     * The new text for the provided range.
     */
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullTextDocumentContentChangeEvent {
    /**
     * The new text of the whole document.
     */
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /**
     * The range's start position.
     */
    pub start: Position,

    /**
     * The range's end position.
     */
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /**
     * Line position in a document (zero-based).
     */
    pub line: u32,

    /**
     * Character offset on a line in a document (zero-based). The meaning of this
     * offset is determined by the negotiated `PositionEncodingKind`.
     *
     * If the character value is greater than the line length it defaults back
     * to the line length.
     */
    pub character: u32,
}
