use std::{collections::{HashMap, HashSet}, rc::Rc, borrow::Borrow, fmt::{format, Formatter}};
use std::fmt::Debug;

use crate::{core::{DataModel, RcDataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, DataModelBase, Named, Contextual, context::Children, ParseError}, impl_into_RcDataModel};


pub struct Switch {
    base: Rc<DataModelBase>, // todo: I should have a static DataModelBase for each thing in library. Then, we store a Rc<DataModelBase> in each DataModel...
    // bnt: BranchingNonTerminal,
    potential_children: Rc<Vec<Rc<dyn DataModel>>>,
    child: Rc<dyn DataModel>,
    select_fn: Rc<dyn Fn(Rc<Context>) -> Rc<dyn DataModel>>,
}

impl Switch {
    pub fn new_no_name(potential_children: Rc<Vec<Rc<dyn DataModel>>>, child: Rc<dyn DataModel>, select_fn: Rc<dyn Fn(Rc<Context>) -> Rc<dyn DataModel>>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Switch".to_string())),
            potential_children,
            child,
            select_fn,
        }
    }
    pub fn new(name: &str, potential_children: Vec<RcDataModel>, select_fn: Rc<dyn Fn(Rc<Context>) -> Rc<dyn DataModel>>) -> RcDataModel {
        let child = potential_children[0].clone();
        let mut result = Self::new_no_name(Rc::new(potential_children), child, select_fn);
        result.set_name(name);
        Rc::new(result)
    }
    // todo: this should probably be an interface or something...
    // I think this is meant for making this better, but it still sucks IMHO: https://docs.rs/delegate/latest/delegate/#
    pub fn name(&self) -> &String {
        self.base.name()
    }
}

impl Debug for Switch {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("Switch: {}", self.name());
        Ok(())
    }
}

impl DataModel for Switch {}

impl Contextual for Switch {
    fn child(&self) -> Rc<dyn DataModel> {
        self.child.clone()
    }
}

impl Cloneable for Switch {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self{
            base: self.base.clone(),
            potential_children: self.potential_children.clone(),
            child: self.child.clone(),
            select_fn: self.select_fn.clone(),
        })
    }
}

impl Breed for Switch {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Switch {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError> {
        let child = (self.select_fn)(ctx.clone());
        let child_ctx = Rc::new(Context::new(Rc::downgrade(ctx), Children::Zilch));
        child.parse(input, &child_ctx)
    }
}

impl Ast for Switch {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Switch {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        self.child.fuzz().iter().map(|mutated_child| {
            let mutated_self: Rc<dyn DataModel> = Rc::new(Self {
                base: self.base.clone(),
                potential_children: self.potential_children.clone(),
                child: mutated_child.clone(),
                select_fn: self.select_fn.clone(),
            });
            mutated_self
        }).collect()
    }
}

impl Named for Switch {
    fn name(&self) -> &String {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

// TODO: don't have duplicate code between here and Constraint
impl Vectorizer for Switch {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
        for c in self.potential_children.iter() {
            c.do_features(features);
        }
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally(self.name(), depth);
        self.child.do_vectorization(fv, depth);
    }
}

impl Serializer for Switch {
    fn do_serialization(&self, ba: &mut BitArray) {
        self.child.do_serialization(ba);
    }
}

// impl From<Switch> for Rc<dyn DataModel> {
//     fn from(dm: Switch) -> Rc<dyn DataModel> {
//         Rc::new(dm)
//     }
// }

impl_into_RcDataModel!(Switch);
