use crate::mock::MockFn;
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockColorFn;
impl MockFn for MockColorFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "#{:02X}{:02X}{:02X}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockRGBFn;
impl MockFn for MockRGBFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "rgb({},{},{})",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockRGBAFn;
impl MockFn for MockRGBAFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "rgba({},{},{},{})",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            (rand::random::<f32>() * 100.0).round() / 100.0
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockHSLFn;
impl MockFn for MockHSLFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        let mut rng = rand::thread_rng();
        let h = rng.gen_range(0..=360);
        let s = rng.gen_range(0..=100);
        let l = rng.gen_range(0..=100);
        json!(format!("hsl({},{},{})", h, s, l))
    }
}
