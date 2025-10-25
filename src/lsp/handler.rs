use crate::lsp::{
    handler::initialize::InitializeHandler,
    messages::core::{error_codes, Method, Request, Response, ResponseError},
};

pub mod initialize;

impl Default for JustLspHandlers {
    fn default() -> Self {
        Self {
            initialize: InitializeHandler,
        }
    }
}
pub struct JustLspHandlers {
    initialize: InitializeHandler,
}

impl JustLspHandlers {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dispatch(&self, req: Request) -> Response {
        match req.method {
            Method::Initialize(params) => {
                let result = self.initialize.handle(params);
                Response {
                    jsonrpc: String::from("2.0"),
                    id: Some(req.id),
                    result: Some(serde_json::to_value(result).expect(
                        "Unexpected error: result should convert to Value type without fail",
                    )),
                    error: None,
                }
            }

            _ => Response {
                jsonrpc: String::from("2.0"),
                id: Some(req.id),
                result: None,
                error: Some(ResponseError {
                    code: error_codes::METHOD_NOT_FOUND,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }
}
