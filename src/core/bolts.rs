// use 

use std::{collections::HashMap, rc::Rc, process::Child, ops::Index};

use super::DataModel;

pub struct ChildMap {
    k_to_i: Rc<HashMap<&'static str, usize>>,
    arr: Vec<Rc<dyn DataModel>>,
}

impl ChildMap {
    pub fn empty(&self) -> Self {
        Self {
            k_to_i: self.k_to_i.clone(),
            arr: Vec::new(),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            k_to_i: self.k_to_i.clone(),
            arr: self.arr.clone(),
        }
    }
    pub fn vals(&self) -> &Vec<Rc<dyn DataModel>> {
        &self.arr
    }
    pub fn push(&mut self, child: Rc<dyn DataModel>) {
        if self.arr.len() >= self.k_to_i.len() {
            panic!();
        }
        self.arr.push(child);
    }
}

impl<const N: usize> From<[(&'static str, Rc<dyn DataModel>); N]> for ChildMap {
    fn from(kv_arr: [(&'static str, Rc<dyn DataModel>); N]) -> Self {
        let mut k_to_i = HashMap::new();
        let mut arr = Vec::new();
        for (i, (k, child)) in kv_arr.iter().enumerate() {
            let result = k_to_i.insert(*k, i);
            assert!(result == None);
            arr.push(child.clone());
        }
        Self {
            k_to_i: Rc::new(k_to_i),
            arr,
        }
    }
}

// impl Index for ChildMap {
//     type Output: 
// }

impl Index<&str> for ChildMap {
    type Output = Rc<dyn DataModel>;

    fn index(&self, key: &str) -> &Rc<dyn DataModel> {
        &self.arr[self.k_to_i[key]]
    }
}
