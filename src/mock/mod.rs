mod address;
mod basic;
mod color;
mod constants;
mod date;
mod name;
mod number;
mod text;
mod traits;
mod util;
mod web;

pub use address::*;
pub use basic::*;
pub use color::*;
pub use constants::*;
pub use date::*;
pub use name::*;
pub use number::*;
use std::collections::HashMap;
pub use text::*;
pub use traits::*;
pub use util::*;
pub use web::*;

#[derive(Default)]
pub struct MockFns {
    fns: HashMap<&'static str, Box<dyn MockFn + 'static>>,
}

impl MockFns {
    pub fn get_mock_fn(&self, mock_name: &str) -> Option<&Box<dyn MockFn>> {
        self.fns.get(mock_name)
    }

    pub fn registry(&mut self, mock_name: &'static str, mock_fn: impl MockFn + 'static) {
        self.fns.entry(mock_name).or_insert(Box::new(mock_fn));
    }
}
