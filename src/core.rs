use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, slice::SliceIndex, ops::Index, backtrace::Backtrace};
use std::fmt::Write;
use std::result::Result;

pub mod bit_array;
use bit_array::BitArray;
pub mod feature_vector;
use feature_vector::FeatureVector;
pub mod context;
use context::Context;

use self::bolts::ChildMap;
pub mod bolts;

pub trait DataModel: Breed + Cloneable + Contextual + Parser + Ast + Fuzzer + Named + Vectorizer + Serializer + std::fmt::Debug {}

// I can't just use an alias, because I want to implement traits for Rc in my own way :)
// ... or maybe not, that gets a bit silly...
pub type RcDataModel = Rc<dyn DataModel>;

// use std::ops::{Deref, DerefMut};

// pub struct RcDataModel(pub Rc<dyn DataModel>);

// impl RcDataModel {
//     pub fn new<T: Into<Rc<dyn DataModel>>>(dm: T) -> Self {
//         Self(dm.into())
//     }
// }

// impl Deref for RcDataModel {
//     type Target = Rc<dyn DataModel>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for RcDataModel {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl From<&RcDataModel> for RcDataModel {
//     fn from(rcdm: &RcDataModel) -> Self {
//         RcDataModel(rcdm.0.clone())
//     }
// }
// impl Into<RcDataModel> for &RcDataModel {
//     fn into(self) -> RcDataModel {
//         self.clone()
//     }
// }

#[macro_export]
macro_rules! impl_into_RcDataModel {
    ($t:ty) => {
        impl Into<RcDataModel> for $t {
            fn into(self) -> RcDataModel {
                Rc::new(self)
            }
        }
    };
}

#[derive(Debug)]
pub struct DataModelBase {
    name: String,
}

impl DataModelBase {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

pub trait Cloneable {
    fn clone(&self) -> Box<dyn DataModel>;
}

pub trait Breed {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel>;
}

// This allows the grammar to look around the nodes that have already been parsed.
pub trait Contextual {
    fn child(&self) -> Rc<dyn DataModel> {
        panic!()
    }
    fn vec(&self) -> &Vec<Rc<dyn DataModel>> {
        panic!()
    }
    fn map(&self) -> &ChildMap {
        panic!()
    }
    fn data(&self) -> &BitArray {
        panic!()
    }
    fn int(&self) -> i32 {
        panic!()
    }
    fn str(&self) -> &String {
        panic!()
    }
}

#[derive(Debug)]
pub enum ParseError {
    Err(Rc<Context>, BitArray, Backtrace),
    Children(Vec<ParseError>), // used when a Union fails to parse
}
pub trait Parser {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError>;
}
pub trait Ast: Named {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "{}", self.name()).expect("err writing");
        result
    }
}
pub trait Fuzzer {
    // todo: I think this should take a context object so that we can do fix ups (and fuzzing mutliple dependent data peices)!
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>>; // todo: should this return an Iter instead for performance reasons?
}

pub trait Named {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: &str);
}

// pub trait Traversable

pub trait Vectorizer: Named {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
    }
    // fn do_features(&self, features: HashSet<String>); // todo: change to str?
    fn features(&self) -> FeatureVector {
        let mut features = HashSet::new();
        self.do_features(&mut features);
        let mut features: Vec<String> = features.into_iter().collect();
        features.sort();
        FeatureVector::new(features)
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally(self.name(), depth);
    }
    // fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32);
    fn vectorize(&self) -> FeatureVector {
        let mut fv = self.features();
        self.do_vectorization(&mut fv, 0);
        fv
    }
}
pub trait Serializer {
    fn do_serialization(&self, ba: &mut BitArray);
    fn serialize(&self) -> BitArray {
        let mut ba = BitArray::fresh();
        self.do_serialization(&mut ba);
        ba
    }
}
