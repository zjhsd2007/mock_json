use crate::mock::MockFn;
use crate::mock::{create_sentence, create_word, CN_LETTER_POOL, LETTER_POOL};
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockParagraphFn;
impl MockFn for MockParagraphFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.first());
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(3..=10);
        match lang {
            Some(&"cn") => {
                let mut ts = vec![];
                for _ in 0..=count {
                    ts.push(create_word(&CN_LETTER_POOL, 7, 15));
                }
                let mut str = ts.join("，");
                str.push('。');
                json!(str)
            }
            _ => {
                let mut ts = vec![];
                for _ in 0..=count {
                    ts.push(create_sentence(&LETTER_POOL, 3, 15));
                }
                let mut str = ts.join(", ");
                str.push('.');
                json!(str)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockSentenceFn;
impl MockFn for MockSentenceFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.first());
        match lang {
            Some(&"cn") => json!(create_word(&CN_LETTER_POOL, 7, 15)),
            _ => json!(create_sentence(&LETTER_POOL, 3, 15)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockTitleFn;
impl MockFn for MockTitleFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.first());
        match lang {
            Some(&"cn") => json!(create_word(&CN_LETTER_POOL, 7, 15)),
            _ => json!(create_sentence(&LETTER_POOL, 4, 8)),
        }
    }
}
