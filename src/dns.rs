use crate::{library::{set::Set, sequence::Sequence, u8::U8}, core::DataModel};

pub fn dns() -> Box<dyn DataModel> {
    Box::new(U8::new())
}