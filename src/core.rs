use core::num;
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
    fn serialize(&self) -> BitArray;
}

// I have cool ideas about how to optimize this!
// we could make it an enum that could be normal or could be a Vec<Box<BitArray>> which is sort of lazy...
pub struct BitArray<'a> {
    data: &'a Vec<u8>,
    offset: i32,
    len: i32,
}
impl BitArray<'_> {
    pub fn new(num_bits: i32) -> Self {
        Self {
            data: Vec::with_capacity((num_bits / 8) as usize),
            offset: 0,
            len: num_bits,
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            data: self.data,
            offset: self.offset,
            len: self.len,
        }
    }
    pub fn advance(&mut self, num_bits: i32) {
        self.offset += num_bits;
    }
    // pub fn extend(&mut self, other: &BitArray) {
    //     self.data.extend_from_slice(other.asdf());
    //     self.len += other.len; //
    // }
    // pub fn get(&self) -> 
}

pub struct BinaryStream {
    data: Vec<u8>,
    bit_pos: i32,
}

impl BinaryStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            bit_pos: 0,
        }
    }
    // pub fn eat_bits(&mut self, num_bits) -> BitArray {
    //     let ind = self.bit_pos + num_bits;
    //     let b = self.data[self.pos];
    //     self.pos += 1;
    //     b
    // }
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
