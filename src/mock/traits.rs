use serde_json::Value;

pub trait MockFn: Send {
    fn mock(&self, args: Option<Vec<&str>>) -> Value;
}
