use std::{rc::{Weak, Rc}, collections::HashMap};

use super::DataModel;



pub struct Context<'a> {
    parent: Weak<Context<'a>>,
    children: Children<'a>,
}
pub enum Children<'a> {
    Zilch,
    Child(Rc<dyn DataModel>),
    ChildList(&'a Vec<Rc<dyn DataModel>>),
    ChildMap(&'a HashMap<String, Rc<dyn DataModel>>),
}
impl <'a> Context<'a> {
    pub fn new(parent: Weak<Context<'a>>, children: Children<'a>) -> Self {
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
            *child_list
        } else {
            panic!()
        }
    }
    pub fn map(&self) -> &HashMap<String, Rc<dyn DataModel>> {
        if let Children::ChildMap(child_map) = &self.children {
            *child_map
        } else {
            panic!()
        }
    }
}
