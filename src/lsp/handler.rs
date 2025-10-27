use crate::{
    lsp::{
        handler::initialize::InitializeHandler,
        messages::core::{error_codes, Method, Response, ResponseError},
    },
    rpc::server::Message,
};

pub mod initialize;

impl Message {
    fn method(&self) -> &Method {
        match self {
            Message::Request(r) => &r.method,
            Message::Notification(n) => &n.method,
        }
    }
}

impl Default for JustLspDispatcher {
    fn default() -> Self {
        Self {
            initialize: InitializeHandler,
        }
    }
}
pub trait Dispatcher {
    fn dispatch(&self, message: &Message) -> Option<Response>;
}
pub struct JustLspDispatcher {
    initialize: InitializeHandler,
}
impl JustLspDispatcher {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Dispatcher for JustLspDispatcher {
    fn dispatch(&self, message: &Message) -> Option<Response> {
        let result = match message.method() {
            Method::Initialize(params) => Some(self.initialize.handle(params)),
            _ => None,
        };

        match message {
            Message::Request(r) => {
                if let Some(res) = result {
                    Some(Response {
                        jsonrpc: String::from("2.0"),
                        id: Some(r.id.clone()),
                        result: Some(serde_json::to_value(res).expect(
                            "Unexpected error: result should convert to Value type without fail",
                        )),
                        error: None,
                    })
                } else {
                    Some(Response {
                        jsonrpc: String::from("2.0"),
                        id: Some(r.id.clone()),
                        result: None,
                        error: Some(ResponseError {
                            code: error_codes::METHOD_NOT_FOUND,
                            message: "Method not found".to_string(),
                            data: None,
                        }),
                    })
                }
            }
            Message::Notification(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lsp::messages::{
        core::{EmptyParams, Id, Notification, Request},
        initialize::{ClientCapabilities, InitializeParams},
    };

    use super::*;

    #[test]
    fn test_incoming_request() {
        let req = Request {
            jsonrpc: String::from("2.0"),
            id: Id::String(String::from("1234")),
            method: Method::Initialize(InitializeParams {
                capabilities: ClientCapabilities {
                    workspace: None,
                    experimental: None,
                    general: None,
                    text_document: None,
                    window: None,
                },
                client_info: None,
                initialization_options: None,
                locale: None,
                process_id: None,
                root_path: None,
                root_uri: None,
                trace: None,
                work_done_token: None,
                workspace_folders: None,
            }),
        };
        let msg = Message::Request(req);
        let dispatcher = JustLspDispatcher::default();

        let resp = dispatcher.dispatch(&msg);

        assert!(resp.is_some())
    }
    #[test]
    fn test_incoming_notifaction() {
        let notif = Notification {
            jsonrpc: String::from("2.0"),
            method: Method::Initialized(EmptyParams {}),
        };
        let msg = Message::Notification(notif);
        let dispatcher = JustLspDispatcher::default();

        let res = dispatcher.dispatch(&msg);

        assert!(res.is_none())
    }
    #[test]
    fn test_unsupported_message() {
        let msg = Message::Request(Request {
            jsonrpc: String::from("2.0"),
            id: Id::String(String::from("1234")),
            method: Method::WorkspaceSemanticRefresh,
        });

        let dispatcher = JustLspDispatcher::default();

        let res = dispatcher.dispatch(&msg).unwrap();

        assert!(res.error.is_some());
        assert!(res.result.is_none());
        assert_eq!(res.error.unwrap().message, "Method not found".to_string())
    }
}
