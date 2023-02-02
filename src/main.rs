use crate::core::{bit_array::BitArray, Context, Children};

mod core;
mod library;
mod dns;

fn main() {
    let mut grammar = dns::dns();
    let mut input = BitArray::from_file("./input_data".to_string()).unwrap();
    let ctx = Context {
        children: Children::Zilch,
    };
    let parse_obj = grammar.parse(&mut input, &ctx).unwrap();
    let ast = parse_obj.data_model();
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