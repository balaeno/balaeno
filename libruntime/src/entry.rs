pub trait Entry: 'static {
    fn new(json: String) -> Self;
    fn get() -> String;
    fn set(data: String);
}

pub struct SerdeJsonEntry {
    data: serde_json::Value,
}

impl Entry for SerdeJsonEntry {
    fn new(json: String) -> Self {
        Self {
            // TODO: unsafe
            data: serde_json::from_str(&json).unwrap(),
        }
    }

    fn get(&self) -> String {
        self.data.to_string()
    }

    fn set(&mut self, data: String) {
        self.data = serde_json::from_str(&data).unwrap();
    }
}
