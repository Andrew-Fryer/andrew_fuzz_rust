use std::{collections::HashSet, rc::Rc};

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, Named, DataModelBase};

pub struct Set {
    base: Rc<DataModelBase>, // todo: I think DataModels should share DataModelBases
    child_prototype: Rc<dyn DataModel>,
    children: Vec<Rc<dyn DataModel>>,
    predicate: Rc<dyn Fn(&Context) -> bool>,
}

impl Set {
    // Should I just make children an empty vec?
    pub fn new(child_prototype: Rc<dyn DataModel>, children: Vec<Rc<dyn DataModel>>, predicate: Rc<dyn Fn(&Context) -> bool>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Set".to_string())),
            child_prototype,
            children,
            predicate,
        }
    }
}

impl DataModel for Set {}

impl Cloneable for Set {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            child_prototype: self.child_prototype.clone(),
            children: self.children.clone(), // todo: make sure this isn't a deep (recursive) clone
            predicate: self.predicate.clone(),
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
        let mut new_children = Vec::new();
        let child_ctx = Context::new();
        while (self.predicate)(&child_ctx) {
            if let Some(new_child) = self.child_prototype.parse(input, &child_ctx) {
                new_children.push(Rc::from(new_child));
            } else {
                return None;
            }
        }
        Some(Box::new(Self {
            base: self.base.clone(),
            child_prototype: self.child_prototype.clone(),
            children: new_children,
            predicate: self.predicate.clone(),
        }))
    }
}

impl Ast for Set {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Set {
    // todo: this is kind of a silly way to fuzz in all honesty...
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        let mut result: Vec<Rc<dyn DataModel>> = Vec::new();
        for (i, c) in self.children.iter().enumerate() {
            for mutated_child in c.fuzz() {
                let mut mutated_children = self.children.clone(); // I believe Rc will make this shallow
                mutated_children[i] = Rc::from(mutated_child);
                result.push(Rc::new(Self {
                    base: self.base.clone(),
                    child_prototype: self.child_prototype.clone(),
                    children: mutated_children,
                    predicate: self.predicate.clone(),
                }));
            }
        }
        result
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
