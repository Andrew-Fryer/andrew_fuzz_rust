use std::collections::HashMap;
use core::slice::Iter;

pub struct FeatureVector {
    fs: Vec<String>,
    d: HashMap<String, f64>,
}
impl FeatureVector {
    pub fn new(features: Vec<String>) -> Self {
        let mut d = HashMap::new();
        for f in &features {
            d.insert(f.to_string(), 0f64); // todo: avoid cloning String?
        }
        Self {
            fs: features,
            d,
        }
    }
    pub fn tally(&mut self, feature: &String, depth: i32) {
        *self.d.get_mut(feature).unwrap() += std::f64::consts::E.powi(depth);
    }
    pub fn values(&self) -> Vec<f64> {
        let mut result = Vec::new();
        for f in self.fs.iter() {
            result.push(*self.d.get(&f.to_string()).unwrap());
        }
        result
    }
    pub fn dist(&self, other: &FeatureVector) -> f64 {
        assert!(self.features().collect::<Vec<&String>>() == other.features().collect::<Vec<&String>>());
        let mut result = 0f64;
        for (self_val, other_val) in self.values().iter().zip(other.values().iter()){
            result += (self_val - other_val).powi(2);
        }
        result
    }
    pub fn features(&self) -> Iter<'_, String> {
        self.fs.iter()
    }
}
