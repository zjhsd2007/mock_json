use crate::mock::MockFn;
use crate::mock::{create_sentence, create_word, CN_LETTER_POOL, LETTER_POOL};
use rand::Rng;
use serde_json::{json, Value};

#[derive(Debug, Default, Clone)]
pub struct MockParagraphFn;
impl MockFn for MockParagraphFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.first());
        let (min, max) = args
            .as_ref()
            .and_then(|args| args.get(1))
            .map(|args| {
                let opts = args
                    .split('~')
                    .map(|v| v.parse::<u8>().ok())
                    .collect::<Vec<Option<u8>>>();
                let min = opts.get(0).map(|v| v.unwrap()).unwrap_or(3);
                let max = opts.get(1).map(|v| v.unwrap()).unwrap_or(10);
                (min, max)
            })
            .unwrap_or((3, 10));
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(min..=max);
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
