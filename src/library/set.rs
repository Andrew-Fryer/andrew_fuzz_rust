use std::collections::HashSet;

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, ParsingProgress, Named, DataModelBase};

pub struct Set {
    base: DataModelBase, // todo: I think DataModels should share DataModelBases
    children: Vec<Box<dyn DataModel>>,
}

impl Cloneable for Set {
    fn clone(&self) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Breed for Set {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Set {
    fn parse(&self, input: BitArray, ctx: Context) -> Option<ParsingProgress> {
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

impl Ast for Set {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Set {
    fn fuzz(&self) -> Vec<Box<dyn DataModel>> {
        todo!()
    }
}

impl Named for Set {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for Set {}

impl Serializer for Set {
    fn serialize(&self) -> BitArray {
        todo!()
    }
}
