//! A very simple and easy-to-use JSON generation tool that supports customizable formats and can be extended with custom placeholders.
//! ### Example
//! ``` rust
//!  use mock_json::mock;
//!  use serde_json::json;
//!  
//!  let val = mock(&json!({
//!         "code":0,
//!        "msg":"just text",
//!         "data":[{
//!            "id":"@Id|10",
//!             "title": "@Title",
//!             "datetime":"@DateTime",
//!             "author":{
//!                 "name":"@Name",
//!                 "id":"@Guid",
//!                 "email":"@Email",
//!                 "id_number":"@IdNumber",
//!                 "ip":"@Ip",
//!                 "phones":["@Phone", 1, 3],
//!                 "blog":"@Url",
//!                 "avatar":"@Image|80X80|f7f7f7|fff"
//!             }
//!         }, 5, 20]
//!     }));
//! ```
//!
//! The above code will return a `serde_json::Value`, and after calling `val.to_string()`, its content will be as follows:
//!
//! ```json
//! {
//!   "code": 0,
//!   "msg": "just text",
//!   "data": [
//!     {
//!       "author": {
//!         "avatar": "https://dummyimage.com/80X80/f7f7f7/fff",
//!         "blog": "https://fvopl.ktuh.int/dcvr",
//!         "email": "vpar@hslsl.org",
//!         "id": "ceE68058-5EaB-4bc2-cCc8-F4a2Dff1Fe6A",
//!         "id_number": "646734191701136174",
//!         "ip": "132.86.194.66",
//!         "name": "Patricia Garcia",
//!         "phones": [
//!           "13318945147",
//!           "14923999763"
//!         ]
//!       },
//!       "datetime": "1960-02-12 03:49:48",
//!       "id": "3217A3bAAa",
//!       "title": "bymebox wpvvpv udp pcb lky onigkew sywtnhq"
//!     },
//!     ...
//!   ]
//! }
//! ```
#![allow(clippy::borrow_interior_mutable_const)]
#[allow(clippy::declare_interior_mutable_const)]
#[allow(clippy::borrowed_box)]
mod mock;

pub use crate::mock::MockFn;
use crate::mock::{
    MockAddressFn, MockBoolFn, MockColorFn, MockDateFn, MockDateTimeFn, MockDomainFn, MockEmailFn,
    MockFirstNameFn, MockFloatFn, MockFns, MockGuidFn, MockHSLFn, MockIdFn, MockIdNumberFn,
    MockImageFn, MockIpFn, MockLastNameFn, MockNameFn, MockNumberFn, MockParagraphFn, MockPhoneFn,
    MockRGBAFn, MockRGBFn, MockSentenceFn, MockTimeFn, MockTimestampFn, MockTitleFn, MockTokenFn,
    MockUrlFn, MockZipFn,
};
use once_cell::sync::Lazy;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Mutex;

static MOCK_FNS: Lazy<Mutex<MockFns>> = Lazy::new(|| {
    let mut mock_fns = MockFns::default();

    // basic
    mock_fns.registry("@Guid", MockGuidFn::default());
    mock_fns.registry("@Id", MockIdFn::default());
    mock_fns.registry("@IdNumber", MockIdNumberFn::default());
    mock_fns.registry("@Phone", MockPhoneFn::default());
    mock_fns.registry("@Bool", MockBoolFn::default());

    // name
    mock_fns.registry("@Name", MockNameFn::default());
    mock_fns.registry("@FirstName", MockFirstNameFn::default());
    mock_fns.registry("@LastName", MockLastNameFn::default());

    // web
    mock_fns.registry("@Token", MockTokenFn::default());
    mock_fns.registry("@Url", MockUrlFn::default());
    mock_fns.registry("@Email", MockEmailFn::default());
    mock_fns.registry("@Ip", MockIpFn::default());
    mock_fns.registry("@Domain", MockDomainFn::default());
    mock_fns.registry("@Image", MockImageFn::default());

    //number
    mock_fns.registry("@Number", MockNumberFn::default());
    mock_fns.registry("@Float", MockFloatFn::default());

    // address
    mock_fns.registry("@Zip", MockZipFn::default());
    mock_fns.registry("@Address", MockAddressFn::default());

    // text
    mock_fns.registry("@Paragraph", MockParagraphFn::default());
    mock_fns.registry("@Sentence", MockSentenceFn::default());
    mock_fns.registry("@Title", MockTitleFn::default());

    // date
    mock_fns.registry("@Date", MockDateFn::default());
    mock_fns.registry("@Time", MockTimeFn::default());
    mock_fns.registry("@DateTime", MockDateTimeFn::default());
    mock_fns.registry("@Timestamp", MockTimestampFn::default());

    // color
    mock_fns.registry("@Color", MockColorFn::default());
    mock_fns.registry("@RGB", MockRGBFn::default());
    mock_fns.registry("@RGBA", MockRGBAFn::default());
    mock_fns.registry("@HSL", MockHSLFn::default());

    Mutex::new(mock_fns)
});

/// Register custom placeholders, where mock_fn needs to implement the MockFn trait.
/// ```rust
///     use rand::Rng;
///     use serde_json::{json, Value};
///     use mock_json::{mock, MockFn, registry};
///
///     pub struct MockCMYKFn;
///     impl MockFn for MockCMYKFn {
///          // Ignore the parameter since it is not used here.
///          fn mock(&self, _:Option<Vec<&str>>) -> Value {
///              let mut rng = rand::thread_rng();
///             let c = rng.gen_range(0..=100);
///              let m = rng.gen_range(0..=100);
///             let y = rng.gen_range(0..=100);
///              let k = rng.gen_range(0..=100);
///              json!(format!("cmyk({},{},{},{})", c, m, y, k))
///          }
///      }
///
///      // Register
///     registry("@CMYK", MockCMYKFn);
///  
///      // Usage
///      mock(&json!("@CMYK")); // String("cmyk(99,20,87,54)))
/// ```
pub fn registry(mock_name: &'static str, mock_fn: impl MockFn + 'static) {
    let mut mock_fns = MOCK_FNS.lock().unwrap();
    mock_fns.registry(mock_name, mock_fn);
}

/// mock json
/// ```rust
/// use serde_json::json;
/// use mock_json::mock;
/// let val = mock(&json!([{"user_name": "@Name", "email": "@Email", "age":"@Number|18~100"}, 2, 5]));
///  val.to_string();
/// ```
/// its content will be
/// ```json
/// [
///     {
///         "age": 43,
///        "email": "dbjfm@drgmz.com",
///         "user_name":"Laura White"
///     },
///     {
///        "age": 35,
///         "email": "fbac@yefq.edu",
///         "user_name":"Frank Hernandez"
///    },
///     {
///         "age": 31,
///         "email": "kkhbo@vbqv.cn",
///        "user_name":"Jose Wilson"
///    }
/// ]
/// ```
pub fn mock(val: &Value) -> Value {
    match val {
        Value::String(str) => {
            if str.starts_with('@') {
                let fns = MOCK_FNS.lock().unwrap();
                let mut fields = str.split('|').collect::<Vec<&str>>();
                let mock_name = fields.remove(0);
                if let Some(mock_fn) = fns.get_mock_fn(mock_name) {
                    return if fields.is_empty() {
                        mock_fn.mock(None)
                    } else {
                        mock_fn.mock(Some(fields))
                    };
                }
                val.to_owned()
            } else {
                val.to_owned()
            }
        }
        Value::Object(map) => {
            let mut val_map = HashMap::new();
            for (key, val) in map {
                val_map.insert(key, mock(val));
            }
            json!(val_map)
        }
        Value::Array(arr) => {
            return if arr.len() == 3 {
                let value = arr.get(0);
                let min = arr.get(1).and_then(|v| v.as_u64());
                let max = arr.get(2).and_then(|v| v.as_u64());
                match (value, min, max) {
                    (Some(value), Some(min), Some(max)) => {
                        let mut vec = vec![];
                        let mut rng = rand::thread_rng();
                        let len = rng.gen_range(min..max);
                        for _ in 0..len {
                            vec.push(mock(value));
                        }
                        json!(vec)
                    }
                    _ => val.to_owned(),
                }
            } else {
                val.to_owned()
            };
        }
        _ => val.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        dbg!(mock(&json!("@Float")));
        dbg!(mock(&json!("@Float|3")));
        dbg!(mock(&json!("@Float|-100~100")));
        dbg!(mock(&json!("@Float|3|-100~100")));

        let val = mock(&json!({
            "code":0,
            "msg":"just text",
            "data":[{
                "id":"@Id|10",
                "title": "@Title",
                "datetime":"@DateTime",
                "author":{
                    "name":"@Name",
                    "id":"@Guid",
                    "email":"@Email",
                    "id_number":"@IdNumber",
                    "ip":"@Ip",
                    "phones":["@Phone", 1, 3],
                    "blog":"@Url",
                    "avatar":"@Image|80X80|f7f7f7|fff"
                }
            }, 1, 3]
        }));
        assert!(!val.to_string().is_empty());
    }
}
