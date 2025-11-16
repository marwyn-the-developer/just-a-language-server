use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct State {
    documents: HashMap<String, String>,
}

impl State {
    pub fn new() -> State {
        State {
            documents: HashMap::new(),
        }
    }

    pub fn open_document(&self, uri: String, text: String) {
        todo!()
    }
}
