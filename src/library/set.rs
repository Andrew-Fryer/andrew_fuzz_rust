use std::{rc::{Rc, Weak}, collections::HashSet, fmt::Formatter};

use crate::core::{DataModel, context::Context, Parser, Vectorizer, Serializer, Ast, Fuzzer, Cloneable, Breed, bit_array::BitArray, Named, DataModelBase, Contextual, context::Children, feature_vector::FeatureVector, ParseError};


pub struct Set {
    base: Rc<DataModelBase>, // todo: I think DataModels should share DataModelBases
    child_prototype: Rc<dyn DataModel>,
    children: Vec<Rc<dyn DataModel>>,
    predicate: Rc<dyn Fn(Rc<Context>) -> bool>,
}

impl Set {
    // Should I just make children an empty vec?
    pub fn new(child_prototype: Rc<dyn DataModel>, children: Vec<Rc<dyn DataModel>>, predicate: Rc<dyn Fn(Rc<Context>) -> bool>) -> Self {
        Self {
            base: Rc::new(DataModelBase::new("Set".to_string())),
            child_prototype,
            children,
            predicate,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

impl DataModel for Set {}

impl Contextual for Set {

}

impl Cloneable for Set {
    fn clone(&self) -> Box<dyn DataModel> {
        Box::new(Self {
            base: self.base.clone(),
            child_prototype: self.child_prototype.clone(),
            children: self.children.clone(), // todo: make sure this isn't a deep (recursive) clone
            predicate: self.predicate.clone(),
        })
    }
}

impl Breed for Set {
    fn breed(&self, other: Box<dyn DataModel>) -> Box<dyn DataModel> {
        todo!();
    }
}

impl Parser for Set {
    fn parse(&self, input: &mut BitArray, ctx: &Rc<Context>) -> Result<Box<dyn DataModel>, ParseError> {
        let mut new_children = Rc::new(Vec::new());
        loop {
            let child_ctx = Rc::new(Context::new(Rc::downgrade(ctx), Children::ChildList(new_children.clone())));
            if (self.predicate)(child_ctx.clone()) {
                break;
            }
            let new_child = self.child_prototype.parse(input, &child_ctx)?;
            Rc::make_mut(&mut new_children).push(Rc::from(new_child));
        }
        Ok(Box::new(Self {
            base: self.base.clone(),
            child_prototype: self.child_prototype.clone(),
            children: Rc::try_unwrap(new_children).unwrap(),
            predicate: self.predicate.clone(),
        }))
    }
}

impl Ast for Set {
    fn debug(&self) -> String {
        "".to_string()
    }
}

impl Fuzzer for Set {
    // todo: this is kind of a silly way to fuzz in all honesty...
    fn fuzz(&self) -> Vec<Rc<dyn DataModel>> {
        let mut result: Vec<Rc<dyn DataModel>> = Vec::new();
        for (i, c) in self.children.iter().enumerate() {
            for mutated_child in c.fuzz() {
                let mut mutated_children = self.children.clone(); // I believe Rc will make this shallow
                mutated_children[i] = Rc::from(mutated_child);
                result.push(Rc::new(Self {
                    base: self.base.clone(),
                    child_prototype: self.child_prototype.clone(),
                    children: mutated_children,
                    predicate: self.predicate.clone(),
                }));
            }
        }
        result
    }
}

impl Named for Set {
    fn name(&self) -> &String {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base = Rc::new(DataModelBase::new(name.to_string()));
    }
}

// TODO: I could improve this with a Traverable trait or a Nonterminal trait or something
impl Vectorizer for Set {
    fn do_features(&self, features: &mut HashSet<String>) {
        features.insert(self.name().to_string());
        self.child_prototype.do_features(features);
    }
    fn do_vectorization(&self, fv: &mut FeatureVector, depth: i32) {
        fv.tally(self.name(), depth);
        for c in &self.children {
            c.do_vectorization(fv, depth);
        }
    }
}

impl Serializer for Set {
    // todo: avoid duplicate code between here and Sequence by introducing a `BranchingNonTerminal` trait
    fn do_serialization(&self, ba: &mut BitArray) {
        for c in &self.children {
            c.do_serialization(ba);
        }
    }
}

impl std::fmt::Debug for Set {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("Set");
        Ok(())
    }
}
