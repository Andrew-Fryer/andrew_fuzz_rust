use std::collections::HashMap;
use core::slice::Iter;

// pub trait FeatureVector {
//     fn tally
// }

// pub struct FeatureVector {
//     fs: Vec<String>,
//     d: HashMap<String, f64>,
// }
// impl FeatureVector {
//     pub fn new(features: Vec<String>) -> Self {
//         let mut d = HashMap::new();
//         for f in &features {
//             d.insert(f.to_string(), 0f64); // todo: avoid cloning String?
//         }
//         Self {
//             fs: features,
//             d,
//         }
//     }
//     pub fn tally(&mut self, feature: &String, depth: i32) {
//         *self.d.get_mut(feature).unwrap() += std::f64::consts::E.powi(depth);
//     }
//     pub fn values(&self) -> Vec<f64> {
//         let mut result = Vec::new();
//         for f in self.fs.iter() {
//             result.push(*self.d.get(&f.to_string()).unwrap());
//         }
//         result
//     }
//     pub fn dist(&self, other: &FeatureVector) -> f64 {
//         assert!(self.features().collect::<Vec<&String>>() == other.features().collect::<Vec<&String>>());
//         let mut result = 0f64;
//         for (self_val, other_val) in self.values().iter().zip(other.values().iter()){
//             result += (self_val - other_val).powi(2);
//         }
//         result
//     }
//     pub fn features(&self) -> Iter<'_, String> {
//         self.fs.iter()
//     }
//     pub fn empty(&self) -> FeatureVector {
//         let mut d = HashMap::new();
//         for f in self.features() {
//             d.insert(f.to_string(), 0f64);
//         }
//         Self {
//             fs: self.features().map(|s| s.to_string()).collect(),
//             d,
//         }
//     }
// }

pub struct FeatureVector {
    fs: Vec<(String, String)>,
    d: HashMap<(String, String), f64>,
    last_seen: Option<String>,
}
impl FeatureVector {
    pub fn new(features: Vec<String>) -> Self {
        let mut d = HashMap::new();
        let mut fs = Vec::new();
        for f_start in &features {
            for f_end in &features {
                fs.push((f_start.to_string(), f_end.to_string()));
            }
        }
        for f in &fs {
            d.insert(f.clone(), 0f64); // todo: avoid cloning String?
        }
        Self {
            fs,
            d,
            last_seen: None,
        }
    }
    pub fn tally(&mut self, feature: &String, depth: i32) {
        if let Some(f_start) = &self.last_seen {
            *self.d.get_mut(&(f_start.clone(), feature.clone())).unwrap() += std::f64::consts::E.powi(depth);
        }
        self.last_seen = Some(feature.clone());
    }
    pub fn values(&self) -> Vec<f64> {
        let mut result = Vec::new();
        for f in self.fs.iter() {
            result.push(*self.d.get(f).unwrap());
        }
        result
    }
    pub fn dist(&self, other: &FeatureVector) -> f64 {
        assert!(self.features().collect::<Vec<&(String, String)>>() == other.features().collect::<Vec<&(String, String)>>());
        let mut result = 0f64;
        for (self_val, other_val) in self.values().iter().zip(other.values().iter()){
            result += (self_val - other_val).powi(2);
        }
        result
    }
    pub fn features(&self) -> Iter<'_, (String, String)> {
        self.fs.iter()
    }
    pub fn empty(&self) -> FeatureVector {
        let mut d = HashMap::new();
        for f in self.features() {
            d.insert(f.clone(), 0f64);
        }
        Self {
            fs: self.features().map(|f| f.clone()).collect(),
            d,
            last_seen: None,
        }
    }
}
