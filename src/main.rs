use std::rc::{Weak, Rc};

use crate::core::{bit_array::BitArray, context::Context, context::Children};

mod core;
mod library;
mod dns;

fn main() {
    let mut grammar = dns::dns();
    // let mut input = BitArray::from_file("./input_data".to_string()).unwrap();
    let mut input = BitArray::from_file("./.cur_input".to_string()).unwrap();
    let ctx = Context::new(Weak::new(), Children::Zilch);
    let ast = grammar.parse(&mut input, &Rc::new(ctx)).unwrap();
    let fuzz = ast.fuzz();
    let mut fvs = Vec::new();
    for f in fuzz {
        fvs.push(f.vectorize());
        let fuzzy_data = f.serialize();
        fuzzy_data.to_file("./fuzzy_data".to_string()); // todo: should I be using str or String?
    }
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    // fn foo(x: DataModel) -> i32 {
    //     0
    // }
}