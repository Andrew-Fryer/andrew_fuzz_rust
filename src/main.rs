use std::rc::{Weak, Rc};


use andrew_fuzz::{core::{bit_array::BitArray, context::Context, context::Children}, dns};

fn main() {
    let mut grammar = dns::dns();
    // let mut input = BitArray::from_file("./input_data".to_string()).unwrap();
    // let mut input = BitArray::from_file("./.cur_input".to_string()).unwrap();
    // let mut input = BitArray::from_file("./small_dns_packet.binary".to_string()).unwrap();
    let mut input = BitArray::from_file("./small_dns_packet.binary.mutated".to_string()).unwrap();
    let ctx = Context::new(Weak::new(), Children::Zilch);
    let ast = grammar.parse(&mut input, &Rc::new(ctx)).unwrap();
    let fuzz = ast.fuzz();

    let base_fv = grammar.features();
    let mut fvs = Vec::new();
    for f in fuzz {
        let mut fv = base_fv.empty();
        f.do_vectorization(&mut fv, 0);
        fvs.push(fv);
        let fuzzy_data = f.serialize();
        fuzzy_data.to_file("./fuzzy_data".to_string()); // todo: should I be using str or String?
    }
    let mut distances = Vec::new();
    let mut total_distance = 0f64;
    let mut num_diff = 0;
    for fv1 in &fvs {
        for fv2 in &fvs {
            let dist = fv1.dist(fv2);
            distances.push(dist);
            total_distance += dist;
            if dist > 0.0 {
                num_diff += 1;
            }
        }
    }
    // distances.reduce()
    println!("total distance {:}", total_distance);
    println!("num diff {:}", num_diff);
    println!("{:?}", distances);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    // fn foo(x: DataModel) -> i32 {
    //     0
    // }
}