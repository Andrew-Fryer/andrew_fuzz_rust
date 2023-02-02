use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, ParsingProgress, DataModelBase, Named};

pub struct Sequence {
    base: Rc<DataModelBase>, // todo: I should have a static DataModelBase for each thing in library. Then, we store a Rc<DataModelBase> in each DataModel...
    // bnt: BranchingNonTerminal,
    children: HashMap<String, Rc<dyn DataModel>>,
}

impl Sequence {
    pub fn new(children: HashMap<String, Rc<dyn DataModel>>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Sequence".to_string())),
            children,
        }
    }
    // todo: this should probably be an interface or something...
    // I think this is meant for making this better, but it still sucks IMHO: https://docs.rs/delegate/latest/delegate/#
    pub fn name(&self) -> &String {
        self.base.name()
    }
}

impl DataModel for Sequence {}

impl Cloneable for Sequence {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self{
            base: self.base.clone(),
            children: self.children.clone(),
        })
    }
}

impl Breed for Sequence {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Sequence {
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

impl Named for Sequence {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for Sequence {}

impl Serializer for Sequence {
    fn do_serialization(&self, ba: &mut BitArray) {
        for (child_name, c) in &self.children {
            c.do_serialization(ba);
        }
    }
}
