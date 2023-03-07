use std::rc::{Weak, Rc};

use super::{DataModel, bolts::ChildMap};


#[derive(Debug)]
pub struct Context {
    parent: Weak<Context>,
    children: Children,
}
#[derive(Debug)]
pub enum Children {
    Zilch,
    Child(Rc<dyn DataModel>),
    ChildList(Rc<Vec<Rc<dyn DataModel>>>),
    ChildMap(Rc<ChildMap>),
}
impl Context {
    pub fn new(parent: Weak<Context>, children: Children) -> Self {
        Self {
            parent: parent,
            children,
        }
    }
    pub fn parent(&self) -> Rc<Context> {
        self.parent.upgrade().unwrap().clone()
    }
    pub fn child(&self) -> Rc<dyn DataModel> {
        if let Children::Child(child) = &self.children {
            child.clone()
        } else {
            panic!()
        }
    }
    pub fn vec(&self) -> &Vec<Rc<dyn DataModel>> {
        if let Children::ChildList(child_list) = &self.children {
            child_list
        } else {
            panic!()
        }
    }
    pub fn map(&self) -> &ChildMap {
        if let Children::ChildMap(child_map) = &self.children {
            child_map
        } else {
            panic!()
        }
    }
}
