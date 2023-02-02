use std::{collections::HashSet, rc::Rc};

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, Named, DataModelBase};

pub struct Set {
    base: Rc<DataModelBase>, // todo: I think DataModels should share DataModelBases
    children: Vec<Rc<dyn DataModel>>,
}

impl Set {
    pub fn new(children: Vec<Rc<dyn DataModel>>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Set".to_string())),
            children,
        }
    }
}

impl DataModel for Set {}

impl Cloneable for Set {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            children: self.children.clone(), // todo: make sure this isn't a deep (recursive) clone
        })
    }
}

impl Breed for Set {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Set {
    fn parse(&self, input: &mut BitArray, ctx: &Context) -> Option<Box<dyn DataModel>>{
        if let Some(data) = input.eat(8) { // crap, I think I need `eat` to take &self instead of &mut self
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
    // todo: avoid duplicate code between here and Sequence by introducing a `BranchingNonTerminal` trait
    fn do_serialization(&self, ba: &mut BitArray) {
        for c in &self.children {
            c.do_serialization(ba);
        }
    }
}
