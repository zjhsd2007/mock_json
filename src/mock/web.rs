use crate::mock::{
    create_ip_v4, create_ip_v6, create_word, pick, MockFn, ALL_LETTER_POOL, LETTER_POOL, WEB,
};
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockTokenFn;
impl MockFn for MockTokenFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "{}.{}.{}",
            create_word(&ALL_LETTER_POOL, 19, 20),
            create_word(&ALL_LETTER_POOL, 19, 20),
            create_word(&ALL_LETTER_POOL, 19, 20)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockUrlFn;
impl MockFn for MockUrlFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let protocol = args
            .as_ref()
            .and_then(|args| args.first())
            .unwrap_or(&"https");
        json!(format!(
            "{}://{}.{}.{}/{}",
            protocol,
            create_word(&LETTER_POOL, 3, 5),
            create_word(&LETTER_POOL, 3, 5),
            pick(&WEB),
            create_word(&LETTER_POOL, 3, 5)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockEmailFn;
impl MockFn for MockEmailFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "{}@{}.{}",
            create_word(&LETTER_POOL, 3, 5),
            create_word(&LETTER_POOL, 3, 5),
            pick(&WEB)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockIpFn;
impl MockFn for MockIpFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        return if let Some(&ip_type) = args.as_ref().and_then(|args| args.first()) {
            match ip_type {
                "ipV6" => json!(create_ip_v6()),
                _ => json!(create_ip_v4()),
            }
        } else {
            json!(create_ip_v4())
        };
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockDomainFn;
impl MockFn for MockDomainFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(format!(
            "{}.{}",
            create_word(&LETTER_POOL, 3, 5),
            pick(&WEB)
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockImageFn;
impl MockFn for MockImageFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let size = args
            .as_ref()
            .and_then(|args| args.first())
            .unwrap_or(&"320X240");
        let bg_color = args.as_ref().and_then(|args| args.get(1)).unwrap_or(&"000");
        let fg_color = args.as_ref().and_then(|args| args.get(2)).unwrap_or(&"fff");
        json!(format!(
            "https://dummyimage.com/{}/{}/{}",
            size, bg_color, fg_color
        ))
    }
}
