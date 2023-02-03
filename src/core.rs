use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, slice::SliceIndex, ops::Index};
use std::fmt::Write;

pub mod bit_array;
use bit_array::BitArray;
pub mod feature_vector;
use feature_vector::FeatureVector;

pub trait DataModel: Breed + Cloneable + Contextual + Parser + Ast + Fuzzer + Named + Vectorizer + Serializer {}

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

pub trait Contextual {
    fn parent(&self) -> Rc<dyn Contextual> {
        panic!()
    }
    fn child(&self) -> Rc<dyn Contextual> {
        panic!()
    }
    fn vec(&self) -> &Vec<Rc<dyn Contextual>> {
        panic!()
    }
    fn map(&self) -> &HashMap<String, Rc<dyn Contextual>> {
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

pub trait Parser {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Option<Box<dyn DataModel>>;
}
pub trait Ast: Named {
    fn debug(&self) -> String {
        let mut result = String::new();
        write!(result, "{}", self.name()).expect("err writing");
        result
    }
}
pub trait Fuzzer {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>>; // todo: should this return an Iter instead for performance reasons?
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
    fn do_serialization(&self, ba: &mut BitArray);
    fn serialize(&self) -> BitArray {
        let mut ba = BitArray::fresh();
        self.do_serialization(&mut ba);
        ba
    }
}

// pub struct Input {}
pub struct Context<'a> {
    parent: Weak<Context<'a>>,
    children: Children<'a>,
}
pub enum Children<'a> {
    Zilch,
    Child(Box<dyn DataModel>),
    ChildList(&'a Vec<Rc<dyn DataModel>>),
    ChildMap(&'a HashMap<String, Rc<dyn DataModel>>),
}
impl Context<'_> {
    pub fn new(parent: Weak<Context>, children: Children) -> Self {
        Self {
            parent,
            children,
        }
    }
    pub fn parent(&self) -> Rc<Context> {
        self.parent.upgrade().unwrap()
    }
    pub fn child(&self) -> Rc<dyn DataModel> {
        if let Children::Child(child) = self.children {
            Rc::from(child)
        } else {
            panic!()
        }
    }
}
// impl Index<usize> for Context {
//     type Output = Rc<dyn DataModel>;

//     fn index(&self, index: usize) -> &Self::Output {
//         if let Children::ChildList(children_vec) = self.children {
//             &children_vec[index]
//         } else {
//             panic!()
//         }
//     }
// }
// impl Index<&String> for Context {
//     type Output = Rc<dyn DataModel>;

//     fn index(&self, index: &String) -> &Self::Output {
//         if let Children::ChildMap(children_map) = self.children {
//             &children_map[index]
//         } else {
//             panic!()
//         }
//     }
// }
