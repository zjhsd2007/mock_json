use crate::mock::{
    create_number, pick_str, MockFn, ALL_LETTER_POOL, NUMBER_POOL, PHONE_PREFIX, STR_POOL,
};
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockGuidFn;
impl MockFn for MockGuidFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "{}-{}-{}-{}-{}",
            pick_str(&STR_POOL, 8),
            pick_str(&STR_POOL, 4),
            pick_str(&STR_POOL, 4),
            pick_str(&STR_POOL, 4),
            pick_str(&STR_POOL, 12)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockIdFn;
impl MockFn for MockIdFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let len = args
            .as_ref()
            .and_then(|args| args.first())
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(12);
        json!(pick_str(&ALL_LETTER_POOL, len.into()))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockIdNumberFn;
impl MockFn for MockIdNumberFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "{}{}{:02}{:02}{}{}",
            create_number(110000, 990100),
            create_number(1900, 2030),
            create_number(1, 12),
            create_number(1, 31),
            create_number(100, 999),
            create_number(0, 9)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockPhoneFn;
impl MockFn for MockPhoneFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let phone_num = pick_str(&NUMBER_POOL, 8);
        let len = PHONE_PREFIX.len() as u8;
        let phone_str = format!(
            "{}{}",
            PHONE_PREFIX
                .get((rand::random::<u8>() % len) as usize)
                .unwrap(),
            phone_num
        );
        return if let Some(prefix) = args.as_ref().and_then(|args| args.first()) {
            json!(format!("+{} {}", prefix, phone_str))
        } else {
            json!(phone_str)
        };
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockBoolFn;
impl MockFn for MockBoolFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(rand::random::<f32>() > 0.5)
    }
}
