use crate::mock::MockFn;
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockNumberFn;
impl MockFn for MockNumberFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let mut min = 0_i64;
        let mut max = 1000_i64;
        if let Some(args) = args.as_ref().and_then(|args| args.first()) {
            let opts = args
                .split('~')
                .map(|v| v.trim().parse::<i64>())
                .collect::<Vec<Result<i64, _>>>();

            let mut opts_iter = opts.iter();
            if let Some(Ok(min_num)) = opts_iter.next() {
                min = *min_num;
            }
            if let Some(Ok(max_num)) = opts_iter.next() {
                max = *max_num;
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
            .and_then(|args| args.first())
            .and_then(|v| v.trim().parse::<u8>().ok())
            .unwrap_or(2) as f64;

        let p = 10.0_f64.powf(precise.min(5.0));

        let range = args
            .as_ref()
            .and_then(|args| args.get(1))
            .and_then(|sec_arg| {
                let opts = sec_arg
                    .split('~')
                    .map(|v| v.trim().parse::<f64>())
                    .collect::<Vec<Result<f64, _>>>();
                let mut opts_iter = opts.iter();
                match (opts_iter.next(), opts_iter.next()) {
                    (Some(Ok(min)), Some(Ok(max))) => Some((*min, *max)),
                    _ => None,
                }
            });
        if let Some((min, max)) = range {
            let mut rng = rand::thread_rng();
            let num = rng.gen_range(min..max);
            json!((num * p).round() / p)
        } else {
            let num = rand::random::<f64>();
            json!((num * p).round() / p)
        }
    }
}
