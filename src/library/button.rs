use std::backtrace::Backtrace;
use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use crate::core::ParseError;
use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, Named, DataModelBase, Contextual};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;

#[derive(Debug)]
pub struct Button {
    base: Rc<DataModelBase>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Button".to_string())),
        }
    }
}

impl DataModel for Button {}

impl Contextual for Button {}

impl Cloneable for Button {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
        })
    }
}

impl Breed for Button {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        self.clone()
    }
}

impl Parser for Button {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError> {
        if let None = input.eat(1) {
            Ok(self.clone())
        } else {
            Err(ParseError::Err(ctx.clone(), input.clone(), Backtrace::capture()))
        }
    }
}

impl Ast for Button {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "Button");
        result
    }
}

impl Fuzzer for Button {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        Vec::new()
    }
}

impl Named for Button {
    fn name(&self) -> &String {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

impl Vectorizer for Button {}

impl Serializer for Button {
    fn do_serialization(&self, ba: &mut BitArray) {
        // don't write out anything
    }
}
