use std::collections::HashMap;

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector};

pub struct Sequence {
    // bnt: BranchingNonTerminal,
    children: HashMap<String, Box<dyn DataModel>>,
}

impl Cloneable for Sequence {
    fn clone(&self) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Breed for Sequence {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Sequence {
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

impl Ast for Sequence {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Sequence {
    fn fuzz(&self) -> Vec<Box<dyn DataModel>> {
        todo!()
    }
}

impl Vectorizer for Sequence {
    fn features(&self) -> FeatureVector {
        todo!();
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally("Sequence".to_string(), depth);
    }
}

impl Serializer for Sequence {
    fn serialize(&self) -> BitArray {
        todo!()
    }
}
