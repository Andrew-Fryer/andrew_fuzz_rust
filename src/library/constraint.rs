use std::backtrace::Backtrace;
use std::collections::HashSet;
use std::fmt::{Write, Formatter};
use std::rc::Rc;

use crate::core::{ParseError, RcDataModel};
use crate::core::context::Children;
use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, Named, DataModelBase, Contextual};
use crate::core::bit_array::BitArray;
use crate::core::feature_vector::FeatureVector;
use crate::impl_into_RcDataModel;


pub struct Constraint {
    base: Rc<DataModelBase>,
    child: Rc<dyn DataModel>,
    constraint_fn: Rc<dyn Fn(Rc<Context>) -> bool>,
}

impl Constraint {
    pub fn new_no_name(child: Rc<dyn DataModel>, constraint_fn: Rc<dyn Fn(Rc<Context>) -> bool>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Constraint".to_string())),
            child,
            constraint_fn,
        }
    }
    pub fn new<T: Into<RcDataModel>>(name: &str, child: T, constraint_fn: Rc<dyn Fn(Rc<Context>) -> bool>) -> Self {
        // let child = Rc::<dyn DataModel>::from(child);
        let child = child.into();
        let mut result = Self::new_no_name(child.0, constraint_fn);
        result.set_name(name);
        result
    }
}

pub fn constraint<T: Into<RcDataModel>>(name: &str, child: T, constraint_fn: Rc<dyn Fn(Rc<Context>) -> bool>) -> RcDataModel {
    RcDataModel::new(Constraint::new(name, child, constraint_fn))
}

impl DataModel for Constraint {}

impl Contextual for Constraint {
    fn child(&self) -> Rc<dyn DataModel> {
        self.child.clone()
    }
}

impl Cloneable for Constraint {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            child: self.child.clone(),
            constraint_fn: self.constraint_fn.clone(),
        })
    }
}

impl Breed for Constraint {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        self.clone()
    }
}

impl Parser for Constraint {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError> {
        let parsed_child = self.child.parse(input, ctx)?;
        let parsed_child: Rc<dyn DataModel> = Rc::from(parsed_child);
        let child_ctx = Rc::new(Context::new(Rc::downgrade(ctx), Children::Child(parsed_child.clone())));
        if (self.constraint_fn)(child_ctx.clone()) {
            Ok(Box::new(Self {
                base: self.base.clone(),
                child: parsed_child,
                constraint_fn: self.constraint_fn.clone(),
            }))
        } else {
            Err(ParseError::Err(child_ctx.clone(), input.clone(), Backtrace::capture()))
        }
    }
}

impl Ast for Constraint {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "Constraint").unwrap();
        result
    }
}

impl Fuzzer for Constraint {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        self.child.fuzz()
    }
}

impl Named for Constraint {
    fn name(&self) -> &String {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

// impl Vectorizer for Constraint {
//     fn do_features(&self, features: &mut HashSet<String>) {
//         self.child.do_features(features);
//     }
//     fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
//         self.child.do_vectorization(fv, depth);
//     }
// }

impl Vectorizer for Constraint {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
        self.child.do_features(features);
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally(self.name(), depth);
        self.child.do_vectorization(fv, depth);
    }
}

impl Serializer for Constraint {
    fn do_serialization(&self, ba: &mut BitArray) {
        self.child.do_serialization(ba);
    }
}

impl std::fmt::Debug for Constraint {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("Constraint");
        Ok(())
    }
}

// impl From<Constraint> for Rc<dyn DataModel> {
//     fn from(dm: Constraint) -> Rc<dyn DataModel> {
//         Rc::new(dm)
//     }
// }

impl_into_RcDataModel!(Constraint);
