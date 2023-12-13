use std::rc::{Weak, Rc};


use combinator_fuzzer::{core::{bit_array::BitArray, context::Context, context::Children}, dns, simple_example_grammar};

fn main() {
    // let mut grammar = dns::dns();
    let mut grammar = dns::dns();
    // let mut input = BitArray::from_file("./input_data".to_string()).unwrap();
    // let mut input = BitArray::from_file("./.cur_input".to_string()).unwrap();
    // let mut input = BitArray::from_file("./small_dns_packet.binary".to_string()).unwrap();
    let mut input = BitArray::from_file("./.cur_input".to_string()).unwrap();
    let ctx = Context::new(Weak::new(), Children::Zilch);
    let ast = grammar.parse(&mut input, &Rc::new(ctx)).unwrap();
    return;
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
    use std::rc::{Weak, Rc};

    use combinator_fuzzer::{simple_example_grammar, core::{bit_array::BitArray, context::{Context, Children}}};

    fn try_parse(input: Vec<u8>) -> bool {
        let grammar = simple_example_grammar::simple();
        let mut input = BitArray::new(input, None);
        let ctx = Context::new(Weak::new(), Children::Zilch);
        let parse_result = grammar.parse(&mut input, &Rc::new(ctx));
        let has_ast = parse_result.is_ok();
        has_ast
    }
    #[test]
    fn five_bytes() {
        assert!(try_parse(vec![0x00, 0x01, 0x02, 0x03, 0x04]))
    }
    #[test]
    fn four_bytes_good() {
        assert!(try_parse(vec![0x00, 0x01, 0x02, 0x07]))
    }
    #[test]
    fn four_bytes_bad() {
        assert!(!try_parse(vec![0x00, 0x01, 0x02, 0x03]))
    }
    #[test]
    fn three_bytes() {
        assert!(!try_parse(vec![0x00, 0x01, 0x02]))
    }
}
