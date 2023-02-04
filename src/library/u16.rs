use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, Named, DataModelBase, Contextual};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;

pub struct U16 {
    base: Rc<DataModelBase>,
    data: BitArray,
}

impl U16 {
    pub fn new() -> Self {
        Self::from_u16(0x00)
    }
    pub fn from_u16(data: u16) -> Self {
        let b0 = (data >> 8) as u8;
        let b1 = (data & 0x00FF) as u8;
        Self {
            base: Rc::new(DataModelBase::new("U16".to_string())),
            data: BitArray::new(vec![b0, b1], None),
        }
        
    }
}

impl DataModel for U16 {}

impl Contextual for U16 {
    fn int(&self) -> i32 {
        let mut data = self.data.clone();
        (data.eat(8).unwrap().peek(8) << 8) as i32 | data.eat(8).unwrap().peek(8) as i32
    }
}

impl Cloneable for U16 {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            data: self.data.clone(),
        })
    }
}

impl Breed for U16 {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for U16 {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context<'_>>) -> Option<Box<dyn DataModel>> {
        if let Some(data) = input.eat(16) { // crap, I think I need `eat` to take &self instead of &mut self
            let data_model = Self {
                base: self.base.clone(),
                data,
            };
            Some(Box::new(data_model))
        } else {
            None
        }
    }
}

impl Ast for U16 {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "{:X}", self.int());
        result
    }
}

impl Fuzzer for U16 {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        vec![Rc::new(U16::from_u16(0xFFFF)), Rc::new(U16::from_u16(0xAAAA))]
    }
}

impl Named for U16 {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for U16 {}

impl Serializer for U16 {
    fn do_serialization(&self, ba: &mut BitArray) {
        ba.extend(&self.data);
    }
}
