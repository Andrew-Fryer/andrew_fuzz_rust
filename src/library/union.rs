use std::{collections::{HashMap, HashSet}, rc::Rc, borrow::Borrow};

use crate::core::{DataModel, Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, feature_vector::FeatureVector, DataModelBase, Named};

pub struct Union {
    base: Rc<DataModelBase>, // todo: I should have a static DataModelBase for each thing in library. Then, we store a Rc<DataModelBase> in each DataModel...
    // bnt: BranchingNonTerminal,
    potential_children: Rc<Vec<Box<dyn DataModel>>>,
    child: Rc<dyn DataModel>,
}

impl Union {
    pub fn new(potential_children: Rc<Vec<Box<dyn DataModel>>>, child: Rc<dyn DataModel>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Union".to_string())),
            potential_children,
            child,
        }
    }
    // todo: this should probably be an interface or something...
    // I think this is meant for making this better, but it still sucks IMHO: https://docs.rs/delegate/latest/delegate/#
    pub fn name(&self) -> &String {
        self.base.name()
    }
}

impl DataModel for Union {}

impl Cloneable for Union {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self{
            base: self.base.clone(),
            potential_children: self.potential_children.clone(),
            child: self.child.clone(),
        })
    }
}

impl Breed for Union {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Union {
    fn parse(&self, input: &mut BitArray, ctx: &Context) -> Option<Box<dyn DataModel>> {
        let mut successful_children = Vec::new();
        for c in &*self.potential_children {
            let mut input_for_child = input.clone();
            let child_ctx = Context::new(); // todo: do this properly
            if let Some(new_child) = c.parse(&mut input_for_child, ctx) {
                successful_children.push(new_child);
            }
        }
        if successful_children.len() > 1 {
            // println!("Warning: found ambiguity! {:?}", successful_children.iter().map(|c| c.debug()));
            println!("Warning: found ambiguity!");
        }
        while successful_children.len() > 1 {
            successful_children.pop();
        }
        if let Some(child) = successful_children.pop() {
            Some(Box::new(Self {
                base: self.base.clone(),
                potential_children: self.potential_children.clone(),
                child: Rc::from(child),
            }))
        } else {
            None
        }
    }
}

impl Ast for Union {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Union {
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        todo!();
        // let mut result: Vec<Rc<dyn DataModel>> = Vec::new();
        // for (child_name, c) in &self.children {
        //     for mutated_child in c.fuzz() {
        //         let mut mutated_children = self.children.clone(); // I believe Rc will make this shallow
        //         mutated_children.insert(child_name.to_string(), Rc::from(mutated_child));
        //         result.push(Rc::new(Self {
        //             base: self.base.clone(),
        //             children: mutated_children,
        //         }));
        //     }
        // }
        // result
    }
}

impl Named for Union {
    fn name(&self) -> &String {
        self.base.name()
    }
}

impl Vectorizer for Union {}

impl Serializer for Union {
    fn do_serialization(&self, ba: &mut BitArray) {
        self.child.do_serialization(ba);
    }
}