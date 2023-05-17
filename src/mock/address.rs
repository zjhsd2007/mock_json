use crate::mock::MockFn;
use crate::mock::{pick_str, NUMBER_POOL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub const CN_ADDRESS_DATA: Lazy<Result<Address, Box<dyn Error>>> =
    Lazy::new(|| Address::from_path("./src/data/cn_address.json", AddressType::Cn));
pub const EN_ADDRESS_DATA: Lazy<Result<Address, Box<dyn Error>>> =
    Lazy::new(|| Address::from_path("./src/data/en_address.json", AddressType::En));

#[derive(Debug, Default, Clone)]
pub struct MockZipFn;
impl MockFn for MockZipFn {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        json!(pick_str(&NUMBER_POOL, 6))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockAddressFn;
impl MockFn for MockAddressFn {
    fn mock(&self, args: Option<Vec<&str>>) -> Value {
        let lang = args.as_ref().and_then(|args| args.get(0));
        match lang {
            Some(&"cn") => json!(CN_ADDRESS_DATA.as_ref().unwrap().random()),
            _ => json!(EN_ADDRESS_DATA.as_ref().unwrap().random()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AddressType {
    Cn,
    En,
}
impl Default for AddressType {
    fn default() -> Self {
        Self::En
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AddressItems(Vec<AddressItem>);

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Address {
    r#type: AddressType,
    address_items: AddressItems,
}

impl Address {
    pub fn from_path(path: &str, r#type: AddressType) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut v = serde_json::Deserializer::from_reader(reader);
        let address_items = AddressItems::deserialize(&mut v)?;
        Ok(Self {
            r#type,
            address_items,
        })
    }

    pub fn random(&self) -> String {
        let mut str_vec: Vec<String> = vec![];
        get_address_name(&mut str_vec, &self.address_items);
        // 英文地址是倒序的
        if matches!(self.r#type, AddressType::En) {
            str_vec.reverse();
            return str_vec.join(" ");
        }
        str_vec.join("")
    }
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AddressItem {
    pub name: String,
    pub sub: Option<Box<AddressItems>>,
}

fn get_address_name(re: &mut Vec<String>, address_items: &AddressItems) {
    let len = address_items.0.len();
    let idx = rand::random::<u32>() % len as u32;
    if let Some(item) = address_items.0.get(idx as usize) {
        re.push(item.name.clone());
        if let Some(sub) = item.sub.as_ref() {
            get_address_name(re, sub)
        }
    }
}
