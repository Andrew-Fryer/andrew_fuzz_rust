use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, Named, DataModelBase, Contextual};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;

pub struct U8 {
    base: Rc<DataModelBase>,
    // data: u8,
    data: BitArray,
}

impl U8 {
    pub fn new() -> Self {
        Self::from_u8(0x00)
    }
    pub fn from_u8(data: u8) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("U8".to_string())),
            data: BitArray::new(vec![data], None),
        }
        
    }
    pub fn value(&self) -> u8 {
        self.data.peek(8)
    }
}

impl DataModel for U8 {}

impl Contextual for U8 {
    fn int(&self) -> i32 {
        self.data.peek(8) as i32
    }
}

impl Cloneable for U8 {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            data: self.data.clone(),
        })
    }
}

impl Breed for U8 {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for U8 {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context<'_>>) -> Option<Box<dyn DataModel>> {
        if let Some(data) = input.eat(8) { // crap, I think I need `eat` to take &self instead of &mut self
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

impl Ast for U8 {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "{:X}", self.data.peek(8));
        result
    }
}

impl Fuzzer for U8 {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        vec![Rc::new(U8::from_u8(0xFF)), Rc::new(U8::from_u8(0xAA))]
    }
}

impl Named for U8 {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for U8 {}

impl Serializer for U8 {
    fn do_serialization(&self, ba: &mut BitArray) {
        ba.extend(&self.data);
    }
}
