use std::{collections::{HashMap, HashSet}};
use std::fmt::Write;

pub mod bit_array;
use bit_array::BitArray;
pub mod feature_vector;
use feature_vector::FeatureVector;

pub trait DataModel: Breed + Cloneable + Parser + Ast + Fuzzer + Named + Vectorizer + Serializer {}

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

pub trait Parser {
    fn parse(&self, input: BitArray, ctx: Context) -> Option<ParsingProgress>;
}
pub trait Ast: Named {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "{}", self.name()).expect("err writing");
        result
    }
}
pub trait Fuzzer {
    fn fuzz(&self) -> Vec<Box<dyn DataModel>>;
}

pub trait Named {
    fn name(&self) -> &String;
}

pub trait Vectorizer: Named {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
    }
    // fn do_features(&self, features: HashSet<String>); // todo: change to str?
    fn features(&self) -> FeatureVector {
        let mut features = HashSet::new();
        self.do_features(&mut features);
        FeatureVector::new(features.into_iter().collect())
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally("U8".to_string(), depth);
    }
    // fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32);
    fn vectorize(&self) -> FeatureVector {
        let mut fv = self.features();
        self.do_vectorization(&mut fv, 0);
        fv
    }
}
pub trait Serializer {
    fn serialize(&self) -> BitArray;
}

// pub struct Input {}
pub struct Context {
    // parent: Option<dyn DataModel>,
    pub children: Children,
}
pub enum Children {
    Zilch,
    Child(Box<dyn DataModel>),
    ChildList(Vec<Box<dyn DataModel>>),
    ChildMap(HashMap<String, Box<dyn DataModel>>),
}

pub struct ParsingProgress {
    data_model: Box<dyn DataModel>,
    stream: BitArray,
}

impl ParsingProgress {
    pub fn new(data_model: Box<dyn DataModel>, stream: BitArray) -> Self {
        Self {
            data_model,
            stream,
        }
    }
    pub fn data_model(&self) -> &Box<dyn DataModel> {
        &self.data_model
    }
    pub fn stream(&self) -> &BitArray {
        &self.stream
    }
}
