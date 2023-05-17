use crate::mock::MockFn;
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockNumberFn;
impl MockFn for MockNumberFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let mut min = 0_i64;
        let mut max = 1000_i64;
        if let Some(args) = args.as_ref().and_then(|args| args.get(0)) {
            let opts = args.split('~').collect::<Vec<&str>>();
            if let Some(min_num) = opts.get(0).and_then(|min_str| min_str.parse::<i64>().ok()) {
                min = min_num;
            }
            if let Some(max_num) = opts.get(1).and_then(|max_str| max_str.parse::<i64>().ok()) {
                max = max_num;
            }
        };
        let mut rng = rand::thread_rng();
        json!(rng.gen_range(min..max))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockFloatFn;
impl MockFn for MockFloatFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let precise = args
            .as_ref()
            .and_then(|args| args.get(0))
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(2) as f64;
        let range = args.as_ref().and_then(|args| args.get(1)).and_then(|v| {
            let opts = v.split('~').collect::<Vec<&str>>();
            let min = opts
                .get(0)
                .and_then(|min_str| min_str.parse::<f64>().ok())
                .unwrap_or(f64::MIN);
            let max = opts
                .get(1)
                .and_then(|max_str| max_str.parse::<f64>().ok())
                .unwrap_or(f64::MAX);
            Some((min, max))
        });
        let p = 10.0_f64.powf(precise.min(5.0));
        return if let Some((min, max)) = range {
            let mut rng = rand::thread_rng();
            let num = rng.gen_range(min..max);
            json!((num * p).round() / p)
        } else {
            let num = rand::random::<f64>();
            json!((num * p).round() / p)
        };
    }
}
