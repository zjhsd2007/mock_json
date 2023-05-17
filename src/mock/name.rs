use crate::mock::traits::MockFn;
use crate::mock::{pick, CN_FIRST_NAME, CN_LAST_NAME, FIRST_NAME, LAST_NAME};
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockNameFn;
impl MockFn for MockNameFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.get(0));
        match lang {
            Some(&"cn") => json!(format!("{}{}", pick(&CN_FIRST_NAME), pick(&CN_LAST_NAME))),
            _ => json!(format!("{} {}", pick(&FIRST_NAME), pick(&LAST_NAME))),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockFirstNameFn;
impl MockFn for MockFirstNameFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.get(0));
        match lang {
            Some(&"cn") => json!(pick(&CN_FIRST_NAME)),
            _ => json!(pick(&FIRST_NAME)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockLastNameFn;
impl MockFn for MockLastNameFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.get(0));
        match lang {
            Some(&"cn") => json!(pick(&CN_LAST_NAME)),
            _ => json!(pick(&LAST_NAME)),
        }
    }
}
