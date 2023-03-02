use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, Named, DataModelBase, Contextual};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;

pub struct Constraint {
    base: Rc<DataModelBase>,
}

impl Constraint {
    pub fn new(child: Rc<dyn DataModel>, constraint_fn: Rc<dyn Fn(Context) -> bool>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Constraint".to_string())),
        }
    }
}

impl DataModel for Constraint {}

impl Contextual for Constraint {}

impl Cloneable for Constraint {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
        })
    }
}

impl Breed for Constraint {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        self.clone()
    }
}

impl Parser for Constraint {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context<'_>>) -> Option<Box<dyn DataModel>> {
        if let None = input.eat(1) {
            Some(self.clone())
        } else {
            None
        }
    }
}

impl Ast for Constraint {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "Button");
        result
    }
}

impl Fuzzer for Constraint {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        Vec::new()
    }
}

impl Named for Constraint {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for Constraint {}

impl Serializer for Constraint {
    fn do_serialization(&self, ba: &mut BitArray) {
        // don't write out anything
    }
}
