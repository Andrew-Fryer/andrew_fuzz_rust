use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;

pub struct U8 {
    // data: u8,
    data: BitArray,
}

impl U8 {
    pub fn new() -> U8 {
        Self {
            data: BitArray::new(vec![0x00], None)
        }
    }
}

impl DataModel for U8 {}

impl Cloneable for U8 {
    fn clone(&self) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Breed for U8 {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for U8 {
    fn parse(&self, input: BitArray, ctx: Context) -> Option<Box<dyn DataModel>> {
        if let Some(data) = input.clone().eat(8) { // crap, I think I need `eat` to take &self instead of &mut self
            todo!()
            // Some(Box::new(Self {
            //     data,
            // }))
        } else {
            None
        }
    }
}

impl Ast for U8 {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for U8 {
    fn fuzz(&self) -> Vec<Box<dyn DataModel>> {
        todo!()
    }
}

impl Vectorizer for U8 {
    fn features(&self) -> FeatureVector {
        todo!();
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally("U8".to_string(), depth);
    }
}

impl Serializer for U8 {
    fn serialize(&self) -> BitArray {
        todo!()
    }
}
