use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::{Display, Formatter, Result};

use crate::lsp::messages::initialize::InitializeParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    String(String),
    Number(i32),
}
impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Id::String(s) => write!(f, "{}", s),
            Id::Number(n) => write!(f, "{}", n),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Id,
    #[serde(flatten)]
    pub method: Method,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde_with::skip_serializing_none]
pub struct Response {
    pub jsonrpc: String,
    pub id: Option<Id>,
    pub result: Option<Value>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotificationMessageParams {
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct NotificationMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<NotificationMessageParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelParams {
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressParams<T> {
    pub token: ProgressToken,
    pub value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverParams {
    #[serde(rename = "textDocument")]
    pub text_document: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverResult {
    pub value: String,
}

pub mod error_codes {
    // JSON-RPC errors
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;

    // LSP specific
    pub const SERVER_NOT_INITIALIZED: i32 = -32002;
    pub const UNKNOWN_ERROR_CODE: i32 = -32001;
    pub const REQUEST_FAILED: i32 = -32803;
    pub const SERVER_CANCELLED: i32 = -32802;
    pub const CONTENT_MODIFIED: i32 = -32801;
    pub const REQUEST_CANCELLED: i32 = -32800;
}

/// Represents all LSP request and notification methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum Method {
    // Lifecycle
    #[serde(rename = "initialize")]
    Initialize(InitializeParams),
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "exit")]
    Exit,
}
