use std::{collections::HashMap, slice::Iter};

pub trait DataModel: Breed + Cloneable + Parser + Ast + Fuzzer + Vectorizer + Serializer {}

pub trait Cloneable {
    fn clone(&self) -> dyn DataModel;
}

pub trait Breed {
    fn breed(&self, other: dyn DataModel) -> dyn DataModel;
}

pub trait Parser {
    fn parse(&self, input: BinaryStream, ctx: Context) -> dyn DataModel; // why don't I need a box around `dyn DataModel`?
}
pub trait Ast {
    fn debug(&self) -> String;
}
pub trait Fuzzer {
    fn fuzz(&self) -> Vec<Box<dyn DataModel>>;
}

pub struct FeatureVector {
    fs: Vec<String>,
    d: HashMap<String, f64>,
}
impl FeatureVector {
    pub fn new(features: Vec<String>) -> Self {
        let mut d = HashMap::new();
        for f in &features {
            d.insert(f.to_string(), 0f64); // todo: avoid cloning String?
        }
        Self {
            fs: features,
            d,
        }
    }
    pub fn tally(&mut self, feature: String, depth: i32) {
        *self.d.get_mut(&feature).unwrap() += std::f64::consts::E.powi(depth);
    }
    pub fn values(&self) -> Vec<f64> {
        let mut result = Vec::new();
        for f in self.fs.iter() {
            result.push(*self.d.get(&f.to_string()).unwrap());
        }
        result
    }
    pub fn dist(&self, other: &FeatureVector) -> f64 {
        assert!(self.features().collect::<Vec<&String>>() == other.features().collect::<Vec<&String>>());
        let mut result = 0f64;
        for (self_val, other_val) in self.values().iter().zip(other.values().iter()){
            result += (self_val - other_val).powi(2);
        }
        result
    }
    pub fn features(&self) -> Iter<'_, String> {
        self.fs.iter()
    }
}
// impl Iterator for FeatureVector
pub trait Vectorizer {
    fn features(&self) -> FeatureVector;
    fn vectorize(&self) -> FeatureVector;
}
pub trait Serializer {
    fn serialize(&self) -> BinaryStream;
}

pub struct BinaryStream {
    data: Vec<u8>,
    pos: usize,
    bit_pos: u8,
}

impl BinaryStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: 0,
            bit_pos: 0,
        }
    }
    pub fn eat_byte(&mut self) -> u8 {
        assert!(self.bit_pos == 0); // todo
        let b = self.data[self.pos];
        self.pos += 1;
        b
    }
}

// pub struct Input {}
pub struct Context {
    // parent: Option<dyn DataModel>,
    children: Children,
}
pub enum Children {
    None,
    Child(Box<dyn DataModel>),
    ChildList(Vec<Box<dyn DataModel>>),
    ChildMap(HashMap<String, Box<dyn DataModel>>),
}

pub struct ParsingProgress {
    pub data_model: Box<dyn DataModel>,
    pub stream: BinaryStream,
}

////////////////////////////////////
// asdf

