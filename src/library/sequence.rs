use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}};

use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, DataModelBase, Named, Contextual, context::Children, bolts::ChildMap, ParseError};

#[derive(Debug)]
pub struct Sequence {
    base: Rc<DataModelBase>, // todo: I should have a static DataModelBase for each thing in library. Then, we store a Rc<DataModelBase> in each DataModel...
    // bnt: BranchingNonTerminal,
    children: ChildMap,
    // children: HashMap<String, Rc<dyn DataModel>>,
}

impl Sequence {
    pub fn new(children: ChildMap) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Sequence".to_string())),
            children,
        }
    }
    // todo: this should probably be an interface or something...
    // I think this is meant for making this better, but it still sucks IMHO: https://docs.rs/delegate/latest/delegate/#
}

impl DataModel for Sequence {}

impl Contextual for Sequence {
    fn map(&self) -> &ChildMap {
        &&self.children
    }
}

impl Cloneable for Sequence {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self{
            base: self.base.clone(),
            children: self.children.clone(),
        })
    }
}

impl Breed for Sequence {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Sequence {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError> {
        let mut new_children = Rc::new(self.children.empty());
        for c in self.children.vals() {
            let child_ctx = Context::new(Rc::downgrade(ctx), Children::ChildMap(new_children.clone()));
            let new_child = c.parse(input, &Rc::new(child_ctx))?;
            // println!("parsed child: {:?}", new_child.serialize());
            // println!("{}: parsed child: {} : {:?}", self.name(), new_child.name(), new_child.serialize());
            Rc::make_mut(&mut new_children).push(Rc::from(new_child));
        }
        Ok(Box::new(Self {
            base: self.base.clone(),
            // TODO: get rid of this slow clone
            children: (*new_children).clone(), //Rc::make_mut(&mut new_children),
        }))
    }
}

impl Ast for Sequence {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Sequence {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        let mut result: Vec<Rc<dyn DataModel>> = Vec::new();
        for (i, c) in self.children.vals().iter().enumerate() {
            for mutated_child in c.fuzz() {
                let mut mutated_children = self.children.clone(); // I believe Rc will make this shallow
                mutated_children.set_ind(i, Rc::from(mutated_child));
                result.push(Rc::new(Self {
                    base: self.base.clone(),
                    children: mutated_children,
                }));
            }
        }
        result
    }
}

impl Named for Sequence {
    fn name(&self) -> &String {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

impl Vectorizer for Sequence {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
        for c in self.children.vals() {
            c.do_features(features);
        }
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally(self.name(), depth);
        for c in self.children.vals() {
            c.do_vectorization(fv, depth);
        }
    }
}

impl Serializer for Sequence {
    fn do_serialization(&self, ba: &mut BitArray) {
        for c in self.children.vals() {
            c.do_serialization(ba);
        }
    }
}
