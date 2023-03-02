// use 

use std::{collections::HashMap, rc::Rc};

use super::DataModel;

pub struct ChildMap<'a> {
    k_to_i: HashMap<&'a str, usize>,
    arr: Vec<Rc<dyn DataModel>>,
}

impl<'a, const N: usize> From<[(&'a str, Rc<dyn DataModel>); N]> for ChildMap<'a> {
    fn from(kv_arr: [(&'a str, Rc<dyn DataModel>); N]) -> Self {
        let mut k_to_i = HashMap::new();
        let mut arr = Vec::new();
        for (i, (k, child)) in kv_arr.iter().enumerate() {
            let result = k_to_i.insert(*k, i);
            assert!(result != None);
            arr.push(child.clone());
        }
        Self {
            k_to_i,
            arr,
        }
    }
}
