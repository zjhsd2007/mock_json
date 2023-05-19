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
        let mut precise = None;
        let mut range_str = None;
        if let Some(args) = args.as_ref() {
            if args.len() == 1 {
                let first_arg = args.first().unwrap();
                if first_arg.contains('~') {
                    range_str = Some(first_arg);
                } else {
                    precise = first_arg.parse::<u8>().ok();
                }
            }
            if args.len() == 2 {
                precise = args.first().and_then(|v| v.parse::<u8>().ok());
                range_str = args.get(1);
            }
        }
        let p = precise.map_or(10_f64.powf(2.0), |v| 10_f64.powf((v as f64).min(5.0)));
        if let Some(range_str) = range_str {
            let opts = range_str
                .split('~')
                .map(|v| v.trim().parse::<f64>())
                .collect::<Vec<Result<f64, _>>>();
            let mut opts_iter = opts.iter();
            if let Some((Ok(min), Ok(max))) = opts_iter.next().zip(opts_iter.next()) {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(*min..*max);
                return json!((num * p).round() / p);
            }
        }
        let num = rand::random::<f64>();
        json!((num * p).round() / p)
    }
}
