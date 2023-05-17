use crate::mock::{format_datetime, MockFn};
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockDateFn;
impl MockFn for MockDateFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let format = args
            .as_ref()
            .and_then(|args| args.first())
            .unwrap_or(&"YYYY-MM-DD");
        let mut rng = rand::thread_rng();
        let year = rng.gen_range(1900..2100);
        let month = rng.gen_range(1..13);
        let day = rng.gen_range(1..=31);
        json!(format_datetime(format, year, month, day, 0, 0, 0))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockTimeFn;
impl MockFn for MockTimeFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let format = args
            .as_ref()
            .and_then(|args| args.get(0))
            .unwrap_or(&"hh:mm:ss");
        let mut rng = rand::thread_rng();
        let hour = rng.gen_range(1..=23);
        let minute = rng.gen_range(0..=59);
        let second = rng.gen_range(0..=59);
        json!(format_datetime(format, 2000, 0, 0, hour, minute, second))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockDateTimeFn;
impl MockFn for MockDateTimeFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let format = args
            .as_ref()
            .and_then(|args| args.get(0))
            .unwrap_or(&"YYYY-MM-DD hh:mm:ss");
        let mut rng = rand::thread_rng();
        let year = rng.gen_range(1900..2100);
        let month = rng.gen_range(1..13);
        let day = rng.gen_range(1..=31);
        let hour = rng.gen_range(1..=23);
        let minute = rng.gen_range(0..=59);
        let second = rng.gen_range(0..=59);
        json!(format_datetime(
            format, year, month, day, hour, minute, second
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockTimestampFn;
impl MockFn for MockTimestampFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        let mut rng = rand::thread_rng();
        json!(rng.gen_range::<u64, _>(1600000000000..1900000000000))
    }
}
