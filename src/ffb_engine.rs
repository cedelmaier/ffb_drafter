use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FFBEngine {
    pub players: HashMap<String, String>,
}

impl FFBEngine {
    pub fn new() -> FFBEngine {
        let p: HashMap<String, String> = HashMap::new();
        FFBEngine{ players: p }
    }
}
