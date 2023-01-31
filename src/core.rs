use std::{collections::HashMap, slice::Iter};

mod feature_vector;
use feature_vector::FeatureVector;

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

