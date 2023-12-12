use std::{collections::HashMap, rc::{Rc, Weak}, borrow::BorrowMut, process::Child};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint, switch::Switch}, core::{DataModel, context::{Context, Children}, bolts::ChildMap}};
use crate::core::Named;

pub fn simple() -> Box<dyn DataModel> {
    let uint8: Rc<dyn DataModel> = Rc::new(U8::new());
    let uint16: Rc<dyn DataModel> = Rc::new(U16::new());
    let grammar = Sequence::new("simple_root", ChildMap::from([
        ("first_field", uint8.clone()),
        ("second_field", uint16.clone()),
        ("third_field", Rc::new(Union::new("", Rc::new(vec![
            Rc::new(Constraint::new("divisibility_constraint", uint8.clone(), Rc::new(|ctx| {
                ctx.child().int() % 8 == 0
            }))),
            Rc::new(Sequence::new("uint16_wrapper", ChildMap::from([
                ("field", uint16.clone()),
            ]))),
        ]), uint16.clone()))),
    ]));
    Box::new(grammar)
}

pub fn dns() -> Box<dyn DataModel> {
    let uint8: Rc<dyn DataModel> = Rc::new(U8::new());
    let uint16: Rc<dyn DataModel> = Rc::new(U16::new());

    let mut u32_sequence = Sequence::new_no_name(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
    ]));
    u32_sequence.set_name("u32_sequence");
    let uint32: Rc<dyn DataModel> = Rc::new(u32_sequence);

    let mut u48_sequence = Sequence::new_no_name(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
        ("b4", uint8.clone()),
        ("b5", uint8.clone()),
    ]));
    u48_sequence.set_name("u48_sequence");
    let uint48: Rc<dyn DataModel> = Rc::new(u48_sequence);

    let mut u128_sequence = Sequence::new_no_name(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
        ("b4", uint8.clone()),
        ("b5", uint8.clone()),
        ("b6", uint8.clone()),
        ("b7", uint8.clone()),
        ("b8", uint8.clone()),
        ("b9", uint8.clone()),
        ("b10", uint8.clone()),
        ("b11", uint8.clone()),
        ("b12", uint8.clone()),
        ("b13", uint8.clone()),
        ("b14", uint8.clone()),
        ("b15", uint8.clone()),
    ]));
    u128_sequence.set_name("u128_sequence");
    let uint128: Rc<dyn DataModel> = Rc::new(u128_sequence);

    let mut u256_sequence = Sequence::new_no_name(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
        ("b4", uint8.clone()),
        ("b5", uint8.clone()),
        ("b6", uint8.clone()),
        ("b7", uint8.clone()),
        ("b8", uint8.clone()),
        ("b9", uint8.clone()),
        ("b10", uint8.clone()),
        ("b11", uint8.clone()),
        ("b12", uint8.clone()),
        ("b13", uint8.clone()),
        ("b14", uint8.clone()),
        ("b15", uint8.clone()),
        ("b16", uint8.clone()),
        ("b17", uint8.clone()),
        ("b18", uint8.clone()),
        ("b19", uint8.clone()),
        ("b20", uint8.clone()),
        ("b21", uint8.clone()),
        ("b22", uint8.clone()),
        ("b23", uint8.clone()),
        ("b24", uint8.clone()),
        ("b25", uint8.clone()),
        ("b26", uint8.clone()),
        ("b27", uint8.clone()),
        ("b28", uint8.clone()),
        ("b29", uint8.clone()),
        ("b30", uint8.clone()),
        ("b31", uint8.clone()),
    ]));
    u256_sequence.set_name("u256_sequence");
    let uint256: Rc<dyn DataModel> = Rc::new(u256_sequence);

    let mut letter_set = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let len = ctx.vec().len() as i32;
        let len_field = ctx.parent().map()[&"length".to_string()].child().int();
        let result = len == len_field;
        if result {
            // println!("Parsed all {:} letters", len);
        }
        result
    }));
    letter_set.set_name("letter_set");
    let mut length_constraint = Constraint::new_no_name(uint8.clone(), Rc::new(|ctx| {
        let len_field = ctx.child().int();
        let result = len_field < 0xc0;
        if !result {
            // println!("Failed to parse label length");
        }
        result
    }));
    length_constraint.set_name("length_constraint");
    let mut length_sequence = Sequence::new_no_name(ChildMap::from([
        ("length", Rc::new(length_constraint) as Rc<dyn DataModel>),
        ("letters", Rc::new(letter_set) as Rc<dyn DataModel>),
    ]));
    length_sequence.set_name("length_sequence");
    let mut marker_constraint = Constraint::new_no_name(uint8.clone(), Rc::new(|ctx| {
        let marker_value = ctx.child().int();
        let result = marker_value >= 0xc0;
        if !result {
            // println!("Failed to parse marker");
        }
        result
    }));
    marker_constraint.set_name("marker_constraint");
    let mut marker_sequence = Sequence::new_no_name(ChildMap::from([
        ("marker", Rc::new(marker_constraint) as Rc<dyn DataModel>),
        ("ref", uint8.clone()),
    ]));
    marker_sequence.set_name("marker_sequence");
    let mut label = Union::new_no_name(Rc::new(vec![
        Rc::new(length_sequence),
        Rc::new(marker_sequence),
    ]), uint8.clone());
    label.set_name(&"label");
    let label: Rc<dyn DataModel> = Rc::new(label);
    let domain_predicate: Rc<dyn Fn(Rc<Context>) -> bool> = Rc::new(|ctx: Rc<Context>| {
        // let c = Context::new(Weak::new(), Children::Zilch);
        let v = ctx.vec();
        let v_len = v.len();
        if v_len <= 0 {
            return false;
        }
        let last_element = v[v_len - 1].clone();
        if let Some(constraint_obj) = last_element.child().map().get(&"length".to_string()) {
            let last_element_len = constraint_obj.child().int();
            last_element_len == 0
        } else {
            true // this is hacky, and happens when there is a (marker,ref) instead of a (length,letters)
        }
    });
    let mut domain = Set::new_no_name(label.clone(), vec![label.clone()], domain_predicate);
    domain.set_name(&"domain");
    let domain: Rc<dyn DataModel> = Rc::new(domain);


    let rr_type_a = Box::new(Constraint::new("rr_type_a", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 1
    })));

    let rr_type_ns = Box::new(Constraint::new("rr_type_ns", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 2
    })));

    let mut rr_type_cname = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 5
    }));
    rr_type_cname.set_name("rr_type_cname");
    let rr_type_cname = Box::new(rr_type_cname);

    let mut rr_type_soa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 6
    }));
    rr_type_soa.set_name("rr_type_soa");
    let rr_type_soa = Box::new(rr_type_soa);

    let mut rr_type_ptr = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 12
    }));
    rr_type_ptr.set_name("rr_type_ptr");
    let rr_type_ptr = Box::new(rr_type_ptr);

    let mut rr_type_mx = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 15
    }));
    rr_type_mx.set_name("rr_type_mx");
    let rr_type_mx = Box::new(rr_type_mx);

    let mut rr_type_txt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 16
    }));
    rr_type_txt.set_name("rr_type_txt");
    let rr_type_txt = Box::new(rr_type_txt);

    let mut rr_type_aaaa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 28
    }));
    rr_type_aaaa.set_name("rr_type_aaaa");
    let rr_type_aaaa = Box::new(rr_type_aaaa);

    let mut rr_type_opt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));
    rr_type_opt.set_name("rr_type_opt");
    let rr_type_opt = Box::new(rr_type_opt);

    let mut rr_type_ds = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 43
    }));
    rr_type_ds.set_name("rr_type_ds");
    let rr_type_ds = Box::new(rr_type_ds);

    let mut rr_type_sig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));
    rr_type_sig.set_name("rr_type_sig");
    let rr_type_sig = Box::new(rr_type_sig);

    let mut rr_type_key = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 48
    }));
    rr_type_key.set_name("rr_type_key");
    let rr_type_key = Box::new(rr_type_key);

    let mut rr_type_nsec3 = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 50
    }));
    rr_type_nsec3.set_name("rr_type_nsec3");
    let rr_type_nsec3 = Box::new(rr_type_nsec3);


    let mut rr_type_tsig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 250
    }));
    rr_type_tsig.set_name("rr_type_tsig");
    let rr_type_tsig = Box::new(rr_type_tsig);


    let mut query_type_axfr = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 252
    }));
    query_type_axfr.set_name("query_type_axfr");
    let query_type_axfr = Box::new(query_type_axfr);

    let mut query_type_mailb = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 253
    }));
    query_type_mailb.set_name("query_type_mailb");
    let query_type_mailb = Box::new(query_type_mailb);


    let mut query_type_maila = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 254
    }));
    query_type_maila.set_name("query_type_maila");
    let query_type_maila = Box::new(query_type_maila);

    let mut query_type_all = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 255
    }));
    query_type_all.set_name("query_type_all");
    let query_type_all = Box::new(query_type_all);


    let mut rr_type_default = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        // no check; this is here as a shell so that traversing the tree is the same for all tt types
        true
    }));
    rr_type_default.set_name("rr_type_default");
    let rr_type_default = Box::new(rr_type_default);


    let dummy = uint8.clone(); // this is a silly placeholder that would be used if the grammar was used for generational fuzzing


    let mut query_type = Union::new_no_name(Rc::new(vec![
        rr_type_a,
        rr_type_ns,
        rr_type_cname,
        rr_type_soa,
        rr_type_ptr,
        rr_type_mx,
        rr_type_txt,
        rr_type_aaaa,
        rr_type_opt,
        rr_type_ds,
        rr_type_sig,
        rr_type_key,
        rr_type_nsec3,

        rr_type_tsig,

        query_type_axfr,
        query_type_mailb,
        query_type_maila,
        query_type_all,

        rr_type_default, // default (which will cause ambiguity, but whatever); I could do this a bit better by adding an OrderedUnion non-terminal
    ]), dummy.clone());
    query_type.set_name("query_type");
    let query_type = Rc::new(query_type);
    let mut query = Sequence::new_no_name(ChildMap::from([
        ("name", domain.clone()),
        ("type", query_type),
        ("class", uint16.clone()),
    ]));
    query.set_name(&"query");
    let query = Rc::new(query);
    // let mut rr_data_set = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
    //     let current_len = ctx.vec().len() as i32;
    //     let data_length = ctx.parent().map()[&"dataLength".to_string()].int();
    //     current_len == data_length
    // }));
    // rr_data_set.set_name("rr_data_set");


    let mut rr_body_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        current_len == data_length
    }));
    rr_body_blob.set_name("rr_body_blob");
    let rr_body_blob: Rc<dyn DataModel> = Rc::new(rr_body_blob);


// todo: move the constraint to a dummy (consumes 0 bits) terminal at start of the rr body Sequence to speed up parsing?
    let mut rr_a = Sequence::new_no_name(ChildMap::from([
        ("ipv4_address", uint32.clone()),
    ]));
    rr_a.set_name("rr_a");
    let rr_a = Rc::new(rr_a);


    let mut rr_ns = Sequence::new_no_name(ChildMap::from([
        ("nameServer", domain.clone()),
    ]));
    rr_ns.set_name("rr_ns");
    let rr_ns = Rc::new(rr_ns);


    let mut rr_cname = Sequence::new_no_name(ChildMap::from([
        ("cname", domain.clone()),
    ]));
    rr_cname.set_name("rr_cname");
    let rr_cname = Rc::new(rr_cname);


    let mut rr_soa = Sequence::new_no_name(ChildMap::from([
        ("primaryNameServer", domain.clone()),
        ("reponsibleAuthority", domain.clone()),
        ("serialNumber", uint32.clone()),
        ("refreshInterval", uint32.clone()),
        ("retryInterval", uint32.clone()),
        ("expireLimit", uint32.clone()),
        ("minimumTTL", uint32.clone()),
    ]));
    rr_soa.set_name("rr_soa");
    let rr_soa = Rc::new(rr_soa);


    let mut rr_ptr = Sequence::new_no_name(ChildMap::from([
        ("domainName", domain.clone()),
    ]));
    rr_ptr.set_name("rr_ptr");
    let rr_ptr = Rc::new(rr_ptr);


    let mut rr_mx = Sequence::new_no_name(ChildMap::from([
        ("preference", uint16.clone()),
        ("mailExchange", domain.clone()),
    ]));
    rr_mx.set_name("rr_mx");
    let rr_mx = Rc::new(rr_mx);


    let mut rr_txt = Sequence::new_no_name(ChildMap::from([
        ("text", rr_body_blob.clone()),
    ]));
    rr_txt.set_name("rr_txt");
    let rr_txt = Rc::new(rr_txt);


    let mut rr_aaaa = Sequence::new_no_name(ChildMap::from([
        ("ipv6_address", uint128.clone()),
    ]));
    rr_aaaa.set_name("rr_aaaa");
    let rr_aaaa = Rc::new(rr_aaaa);


    let mut rr_opt = Sequence::new_no_name(ChildMap::from([
        ("opt_records", rr_body_blob),
    ]));
    rr_opt.set_name("rr_opt");
    let rr_opt = Rc::new(rr_opt);


    let mut rr_ds = Sequence::new_no_name(ChildMap::from([
        ("keyid", uint16.clone()),
        ("alg", uint8.clone()),
        ("digestType", uint8.clone()),
        ("digest", uint256.clone()),
    ]));
    rr_ds.set_name("rr_ds");
    let rr_ds = Rc::new(rr_ds);

    let mut rr_sig_signature = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let prev_fields_len: i32 = ctx.parent().map().vals().iter().map(|dm| dm.serialize().len() / 8).sum();
        let current_len = ctx.vec().len() as i32;
        // Constraint doesn't make a wrapper around the context, so we need 4 (not 6) hops up the tree
        // let data_length = ctx.parent().parent().parent().parent().parent().parent().map()[&"body_length"].int();
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        prev_fields_len + current_len == data_length
    }));
    rr_sig_signature.set_name("rr_sig_signature");
    let rr_sig_signature = Rc::new(rr_sig_signature);
    let mut rr_sig = Sequence::new_no_name(ChildMap::from([
        ("type_cov", uint16.clone()),
        ("alg", uint8.clone()),
        ("labels", uint8.clone()),
        ("origTimeToLive", uint32.clone()),
        ("sigExp", uint32.clone()),
        ("sigInception", uint32.clone()),
        ("keyTag", uint16.clone()),
        ("signName", domain.clone()),
        ("signature", rr_sig_signature),
    ]));
    rr_sig.set_name("rr_sig");
    let rr_sig = Rc::new(rr_sig);

    let mut rr_key_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        current_len == data_length - 4
    }));
    rr_key_blob.set_name("rr_key_blob");
    let rr_key_blob: Rc<dyn DataModel> = Rc::new(rr_key_blob);
    let mut rr_key = Sequence::new_no_name(ChildMap::from([
        ("flags", uint16.clone()),
        ("protocol", uint8.clone()),
        ("algorithm", uint8.clone()),
        ("key", rr_key_blob), // todo: we could simplify this by just having one rr_body_blob field here
    ]));
    rr_key.set_name("rr_key");
    let rr_key = Rc::new(rr_key);

    let mut rr_nsec3_salt_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"salt_length"].int();
        current_len == data_length
    }));
    rr_nsec3_salt_blob.set_name("rr_nsec3_salt_blob");
    let rr_nsec3_salt_blob: Rc<dyn DataModel> = Rc::new(rr_nsec3_salt_blob);
    let mut rr_nsec3_hash_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"hash_length"].int();
        current_len == data_length
    }));
    rr_nsec3_hash_blob.set_name("rr_nsec3_hash_blob");
    let rr_nsec3_hash_blob: Rc<dyn DataModel> = Rc::new(rr_nsec3_hash_blob);
    let mut rr_nsec3_type_map_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"length"].int();
        current_len == data_length
    }));
    rr_nsec3_type_map_blob.set_name("rr_nsec3_type_map_blob");
    let rr_nsec3_type_map_blob: Rc<dyn DataModel> = Rc::new(rr_nsec3_type_map_blob);
    let mut rr_nsec3_type_map = Sequence::new_no_name(ChildMap::from([
        ("map_num", uint8.clone()),
        ("length", uint8.clone()),
        ("map_bits", rr_nsec3_type_map_blob),
    ]));
    rr_nsec3_type_map.set_name("rr_nsec3_type_map");
    let rr_nsec3_type_map = Rc::new(rr_nsec3_type_map);
    let mut rr_nsec3 = Sequence::new_no_name(ChildMap::from([
        ("alg", uint8.clone()),
        ("flags", uint8.clone()),
        ("iterations", uint16.clone()),
        ("salt_length", uint8.clone()),
        ("salt", rr_nsec3_salt_blob),
        ("hash_length", uint8.clone()),
        ("next_hash", rr_nsec3_hash_blob),
        ("type_map", rr_nsec3_type_map),
    ]));
    rr_nsec3.set_name("rr_nsec3");
    let rr_nsec3 = Rc::new(rr_nsec3);

    // https://datatracker.ietf.org/doc/html/rfc2845
    let mut mac = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"mac_size"].int();
        current_len == data_length
    }));
    mac.set_name("mac");
    let mac = Rc::new(mac);
    let mut other_data = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"other_data_length"].int();
        current_len == data_length
    }));
    other_data.set_name("other_data");
    let other_data = Rc::new(other_data);
    let mut rr_tsig = Sequence::new_no_name(ChildMap::from([
        ("algorithm", domain.clone()),
        ("time_signed", uint48.clone()),
        ("fudge", uint16.clone()),
        ("mac_size", uint16.clone()),
        ("mac", mac),
        ("original_id", uint16.clone()),
        ("error", uint16.clone()),
        ("other_data_length", uint16.clone()),
        ("other_data", other_data),
    ]));
    rr_tsig.set_name("rr_tsig");
    let rr_tsig = Rc::new(rr_tsig);

    let fail = Rc::new(Constraint::new("fail", dummy.clone(), Rc::new(|_| false)));
    let mut rr_body_switch = Switch::new(Rc::new(vec![
        rr_a.clone(),
        rr_ns.clone(),
        rr_cname.clone(),
        rr_soa.clone(),
        rr_ptr.clone(),
        rr_mx.clone(),
        rr_txt.clone(),
        rr_aaaa.clone(),
        rr_opt.clone(),
        rr_ds.clone(),
        rr_sig.clone(),
        rr_key.clone(),
        rr_nsec3.clone(),

        rr_tsig.clone(),

    ]), dummy.clone(), Rc::new(move |ctx| {
        match ctx.parent().map()[&"type"].child().child().int() {
            1 => rr_a.clone(),
            2 => rr_ns.clone(),
            5 => rr_cname.clone(),
            6 => rr_soa.clone(),
            12 => rr_ptr.clone(),
            15 => rr_mx.clone(),
            16 => rr_txt.clone(),
            28 => rr_aaaa.clone(),
            41 => rr_opt.clone(),
            43 => rr_ds.clone(),
            46 => rr_sig.clone(),
            48 => rr_key.clone(),
            50 => rr_nsec3.clone(),

            250 => rr_tsig.clone(),

            _ => fail.clone(), // just fail
        }
    }));
    rr_body_switch.set_name("rr_body_switch");
    let rr_body_switch = Rc::new(rr_body_switch);

    // TODO: I could make this more lenient by adding a Set that eats any extra bytes...
    let mut rr_body_constraint = Constraint::new_no_name(rr_body_switch, Rc::new(|ctx| {
        let actual_data_len = ctx.child().serialize().len() / 8;
        let required_data_length = ctx.parent().parent().map()[&"body_length".to_string()].int();
        actual_data_len == required_data_length
    }));
    rr_body_constraint.set_name("rr_body_constraint");
    let rr_body_constraint = Box::new(rr_body_constraint);

    let rr_body_unknown = Box::new(Set::new("rr_body_unknown", uint8.clone(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().map()[&"body_length".to_string()].int();
        current_len == data_length
    })));

    let mut rr_body_or_unknown = Union::new_no_name(Rc::new(vec![
        rr_body_constraint,
        rr_body_unknown, // this gives some robustness to the parser
    ]), dummy.clone());
    rr_body_or_unknown.set_name("rr_body_or_unknown");
    let rr_body_or_unknown = Rc::new(rr_body_or_unknown);



    let mut rr_type_a = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 1
    }));
    rr_type_a.set_name("rr_type_a");
    let rr_type_a = Box::new(rr_type_a);

    let mut rr_type_ns = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 2
    }));
    rr_type_ns.set_name("rr_type_ns");
    let rr_type_ns = Box::new(rr_type_ns);

    let mut rr_type_cname = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 5
    }));
    rr_type_cname.set_name("rr_type_cname");
    let rr_type_cname = Box::new(rr_type_cname);

    let mut rr_type_soa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 6
    }));
    rr_type_soa.set_name("rr_type_soa");
    let rr_type_soa = Box::new(rr_type_soa);

    let mut rr_type_ptr = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 12
    }));
    rr_type_ptr.set_name("rr_type_ptr");
    let rr_type_ptr = Box::new(rr_type_ptr);

    let mut rr_type_mx = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 15
    }));
    rr_type_mx.set_name("rr_type_mx");
    let rr_type_mx = Box::new(rr_type_mx);

    let mut rr_type_txt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 16
    }));
    rr_type_txt.set_name("rr_type_txt");
    let rr_type_txt = Box::new(rr_type_txt);

    let mut rr_type_aaaa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 28
    }));
    rr_type_aaaa.set_name("rr_type_aaaa");
    let rr_type_aaaa = Box::new(rr_type_aaaa);

    let mut rr_type_opt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));
    rr_type_opt.set_name("rr_type_opt");
    let rr_type_opt = Box::new(rr_type_opt);

    let mut rr_type_ds = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 43
    }));
    rr_type_ds.set_name("rr_type_ds");
    let rr_type_ds = Box::new(rr_type_ds);

    let mut rr_type_sig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));
    rr_type_sig.set_name("rr_type_sig");
    let rr_type_sig = Box::new(rr_type_sig);

    let mut rr_type_key = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 48
    }));
    rr_type_key.set_name("rr_type_key");
    let rr_type_key = Box::new(rr_type_key);

    let mut rr_type_nsec3 = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 50
    }));
    rr_type_nsec3.set_name("rr_type_nsec3");
    let rr_type_nsec3 = Box::new(rr_type_nsec3);


    let mut rr_type_tsig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 250
    }));
    rr_type_tsig.set_name("rr_type_tsig");
    let rr_type_tsig = Box::new(rr_type_tsig);


    let mut rr_type_default = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        // no check; this is here as a shell so that traversing the tree is the same for all tt types
        true
    }));
    rr_type_default.set_name("rr_type_default");
    let rr_type_default = Box::new(rr_type_default);

    let mut rr_type_field = Union::new_no_name(Rc::new(vec![
        rr_type_a,
        rr_type_ns,
        rr_type_cname,
        rr_type_soa,
        rr_type_ptr,
        rr_type_mx,
        rr_type_txt,
        rr_type_aaaa,
        rr_type_opt,
        rr_type_ds,
        rr_type_sig,
        rr_type_key,
        rr_type_nsec3,

        rr_type_tsig,
        rr_type_default, // default (which will cause ambiguity, but whatever); I could do this a bit better by adding an OrderedUnion non-terminal
    ]), dummy.clone());
    rr_type_field.set_name("rr_type_field");
    let rr_type_field = Rc::new(rr_type_field);

    let mut resource_record = Sequence::new_no_name(ChildMap::from([
        ("name", domain.clone()),
        ("type", rr_type_field),
        ("class", uint16.clone()),
        ("ttl", uint32.clone()),
        ("body_length", uint16.clone()),
        ("rr_body", rr_body_or_unknown),
    ]));
    resource_record.set_name(&"rr");
    let resource_record = Rc::new(resource_record);
    let mut question_set = Set::new_no_name(query.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion"].int()));
    question_set.set_name("question_set");
    let mut answer_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer"].int()));
    answer_set.set_name("answer_set");
    let mut authority_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority"].int()));
    authority_set.set_name("authority_set");
    let mut additional_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional"].int()));
    additional_set.set_name("additional_set");
    let mut result = Box::new(Sequence::new_no_name(ChildMap::from([
        ("transactionId", uint16.clone()),
        ("flags", uint16.clone()),
        ("numQuestion", uint16.clone()),
        ("numAnswer", uint16.clone()),
        ("numAuthority", uint16.clone()),
        ("numAdditional", uint16.clone()),
        ("question", Rc::new(question_set)),
        ("answer", Rc::new(answer_set)),
        ("authority", Rc::new(authority_set)),
        ("additional", Rc::new(additional_set)),
        ("end", Rc::new(Button::new())),
    ])));
    result.set_name(&"dns");
    result
}
