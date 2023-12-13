use std::{collections::HashMap, rc::{Rc, Weak}, borrow::BorrowMut, process::Child};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint, switch::Switch}, core::{DataModel, context::{Context, Children}, bolts::ChildMap}};
use crate::core::Named;

pub fn dns() -> Rc<dyn DataModel> {
    let uint8 = U8::new();
    let uint16 = U16::new();

    let u32_sequence = Sequence::new("u32_sequence", vec![
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
    ]);

    let u48_sequence = Sequence::new("u48_sequence", vec![
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
        ("b4", uint8.clone()),
        ("b5", uint8.clone()),
    ]);

    let u128_sequence = Sequence::new("u128_sequence", vec![
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
    ]);

    let u256_sequence = Sequence::new("u256_sequence", vec![
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
    ]);

    let letter_set = Set::new("letter_set", uint8.clone(), Rc::new(|ctx| {
        let len = ctx.vec().len() as i32;
        let len_field = ctx.parent().map()[&"length".to_string()].child().int();
        let result = len == len_field;
        if result {
            // println!("Parsed all {:} letters", len);
        }
        result
    }));
    let length_constraint = Constraint::new("length_constraint", uint8.clone(), Rc::new(|ctx| {
        let len_field = ctx.child().int();
        let result = len_field < 0xc0;
        if !result {
            // println!("Failed to parse label length");
        }
        result
    }));
    let length_sequence = Sequence::new("length_sequence", vec![
        ("length", length_constraint),
        ("letters", letter_set),
    ]);
    let marker_constraint = Constraint::new("marker_constraint", uint8.clone(), Rc::new(|ctx| {
        let marker_value = ctx.child().int();
        let result = marker_value >= 0xc0;
        if !result {
            // println!("Failed to parse marker");
        }
        result
    }));
    let marker_sequence = Sequence::new("marker_sequence", vec![
        ("marker", marker_constraint),
        ("ref", uint8.clone()),
    ]);
    let label = Union::new("label", vec![
        length_sequence,
        marker_sequence,
    ], uint8.clone());
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
    let domain = Set::new_with_children("domain", label.clone(), vec![label.clone()], domain_predicate);


    let rr_type_a = Constraint::new("rr_type_a", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 1
    }));

    let rr_type_ns = Constraint::new("rr_type_ns", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 2
    }));

    let rr_type_cname = Constraint::new("rr_type_cname", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 5
    }));

    let rr_type_soa = Constraint::new("rr_type_soa", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 6
    }));

    let rr_type_ptr = Constraint::new("rr_type_ptr", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 12
    }));

    let rr_type_mx = Constraint::new("rr_type_mx", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 15
    }));

    let rr_type_txt = Constraint::new("rr_type_txt", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 16
    }));

    let rr_type_aaaa = Constraint::new("rr_type_aaaa", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 28
    }));

    let rr_type_opt = Constraint::new("rr_type_opt", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));

    let rr_type_ds = Constraint::new("rr_type_ds", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 43
    }));

    let rr_type_sig = Constraint::new("rr_type_sig", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));

    let rr_type_key = Constraint::new("rr_type_key", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 48
    }));

    let rr_type_nsec3 = Constraint::new("rr_type_nsec3", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 50
    }));


    let rr_type_tsig = Constraint::new("rr_type_tsig", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 250
    }));


    let query_type_axfr = Constraint::new("query_type_axfr", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 252
    }));

    let query_type_mailb = Constraint::new("query_type_mailb", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 253
    }));


    let query_type_maila = Constraint::new("query_type_maila", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 254
    }));

    let query_type_all = Constraint::new("query_type_all", uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 255
    }));


    let rr_type_default = Constraint::new("rr_type_default", uint16.clone(), Rc::new(|ctx| {
        // no check; this is here as a shell so that traversing the tree is the same for all tt types
        true
    }));


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
    let query = Sequence::new("query", vec![
        ("name", domain.clone()),
        ("type", query_type),
        ("class", uint16.clone()),
    ]);
    // let mut rr_data_set = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
    //     let current_len = ctx.vec().len() as i32;
    //     let data_length = ctx.parent().map()[&"dataLength".to_string()].int();
    //     current_len == data_length
    // }));
    // rr_data_set.set_name("rr_data_set");


    let rr_body_blob = Set::new("rr_body_blob", uint8.clone(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        current_len == data_length
    }));


// todo: move the constraint to a dummy (consumes 0 bits) terminal at start of the rr body Sequence to speed up parsing?
    let rr_a = Sequence::new("rr_a", vec![
        ("ipv4_address", uint32.clone()),
    ]);


    let rr_ns = Sequence::new("rr_ns", vec![
        ("nameServer", domain.clone()),
    ]);


    let rr_cname = Sequence::new("rr_cname", vec![
        ("cname", domain.clone()),
    ]);


    let rr_soa = Sequence::new("rr_soa", vec![
        ("primaryNameServer", domain.clone()),
        ("reponsibleAuthority", domain.clone()),
        ("serialNumber", uint32.clone()),
        ("refreshInterval", uint32.clone()),
        ("retryInterval", uint32.clone()),
        ("expireLimit", uint32.clone()),
        ("minimumTTL", uint32.clone()),
    ]);


    let rr_ptr = Sequence::new("rr_ptr", vec![
        ("domainName", domain.clone()),
    ]);


    let rr_mx = Sequence::new("rr_mx", vec![
        ("preference", uint16.clone()),
        ("mailExchange", domain.clone()),
    ]);


    let rr_txt = Sequence::new("rr_txt", vec![
        ("text", rr_body_blob.clone()),
    ]);


    let rr_aaaa = Sequence::new("rr_aaaa", vec![
        ("ipv6_address", uint128.clone()),
    ]);


    let rr_opt = Sequence::new("rr_opt", vec![
        ("opt_records", rr_body_blob),
    ]);


    let rr_ds = Sequence::new("rr_ds", vec![
        ("keyid", uint16.clone()),
        ("alg", uint8.clone()),
        ("digestType", uint8.clone()),
        ("digest", uint256.clone()),
    ]);

    let rr_sig_signature = Set::new("rr_sig_signature", uint8.clone(), Rc::new(|ctx| {
        let prev_fields_len: i32 = ctx.parent().map().vals().iter().map(|dm| dm.serialize().len() / 8).sum();
        let current_len = ctx.vec().len() as i32;
        // Constraint doesn't make a wrapper around the context, so we need 4 (not 6) hops up the tree
        // let data_length = ctx.parent().parent().parent().parent().parent().parent().map()[&"body_length"].int();
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        prev_fields_len + current_len == data_length
    }));
    let rr_sig = Sequence::new("rr_sig", vec![
        ("type_cov", uint16.clone()),
        ("alg", uint8.clone()),
        ("labels", uint8.clone()),
        ("origTimeToLive", uint32.clone()),
        ("sigExp", uint32.clone()),
        ("sigInception", uint32.clone()),
        ("keyTag", uint16.clone()),
        ("signName", domain.clone()),
        ("signature", rr_sig_signature),
    ]);

    let mut rr_key_blob = Set::new_no_name(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        current_len == data_length - 4
    }));
    rr_key_blob.set_name("rr_key_blob");
    let rr_key_blob: Rc<dyn DataModel> = Rc::new(rr_key_blob);
    let rr_key = Sequence::new("rr_key", vec![
        ("flags", uint16.clone()),
        ("protocol", uint8.clone()),
        ("algorithm", uint8.clone()),
        ("key", rr_key_blob), // todo: we could simplify this by just having one rr_body_blob field here
    ]);

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
    let rr_nsec3_type_map = Sequence::new("rr_nsec3_type_map", vec![
        ("map_num", uint8.clone()),
        ("length", uint8.clone()),
        ("map_bits", rr_nsec3_type_map_blob),
    ]);
    let rr_nsec3 = Sequence::new("rr_nsec3", vec![
        ("alg", uint8.clone()),
        ("flags", uint8.clone()),
        ("iterations", uint16.clone()),
        ("salt_length", uint8.clone()),
        ("salt", rr_nsec3_salt_blob),
        ("hash_length", uint8.clone()),
        ("next_hash", rr_nsec3_hash_blob),
        ("type_map", rr_nsec3_type_map),
    ]);

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
    let rr_tsig = Sequence::new("rr_tsig", vec![
        ("algorithm", domain.clone()),
        ("time_signed", uint48.clone()),
        ("fudge", uint16.clone()),
        ("mac_size", uint16.clone()),
        ("mac", mac),
        ("original_id", uint16.clone()),
        ("error", uint16.clone()),
        ("other_data_length", uint16.clone()),
        ("other_data", other_data),
    ]);

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
    let rr_body_constraint = Rc::new(rr_body_constraint);

    let rr_body_unknown = Rc::new(Set::new("rr_body_unknown", uint8.clone(), Rc::new(|ctx| {
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
    let rr_type_a = Rc::new(rr_type_a);

    let mut rr_type_ns = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 2
    }));
    rr_type_ns.set_name("rr_type_ns");
    let rr_type_ns = Rc::new(rr_type_ns);

    let mut rr_type_cname = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 5
    }));
    rr_type_cname.set_name("rr_type_cname");
    let rr_type_cname = Rc::new(rr_type_cname);

    let mut rr_type_soa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 6
    }));
    rr_type_soa.set_name("rr_type_soa");
    let rr_type_soa = Rc::new(rr_type_soa);

    let mut rr_type_ptr = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 12
    }));
    rr_type_ptr.set_name("rr_type_ptr");
    let rr_type_ptr = Rc::new(rr_type_ptr);

    let mut rr_type_mx = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 15
    }));
    rr_type_mx.set_name("rr_type_mx");
    let rr_type_mx = Rc::new(rr_type_mx);

    let mut rr_type_txt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 16
    }));
    rr_type_txt.set_name("rr_type_txt");
    let rr_type_txt = Rc::new(rr_type_txt);

    let mut rr_type_aaaa = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 28
    }));
    rr_type_aaaa.set_name("rr_type_aaaa");
    let rr_type_aaaa = Rc::new(rr_type_aaaa);

    let mut rr_type_opt = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));
    rr_type_opt.set_name("rr_type_opt");
    let rr_type_opt = Rc::new(rr_type_opt);

    let mut rr_type_ds = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 43
    }));
    rr_type_ds.set_name("rr_type_ds");
    let rr_type_ds = Rc::new(rr_type_ds);

    let mut rr_type_sig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));
    rr_type_sig.set_name("rr_type_sig");
    let rr_type_sig = Rc::new(rr_type_sig);

    let mut rr_type_key = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 48
    }));
    rr_type_key.set_name("rr_type_key");
    let rr_type_key = Rc::new(rr_type_key);

    let mut rr_type_nsec3 = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 50
    }));
    rr_type_nsec3.set_name("rr_type_nsec3");
    let rr_type_nsec3 = Rc::new(rr_type_nsec3);


    let mut rr_type_tsig = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 250
    }));
    rr_type_tsig.set_name("rr_type_tsig");
    let rr_type_tsig = Rc::new(rr_type_tsig);


    let mut rr_type_default = Constraint::new_no_name(uint16.clone(), Rc::new(|ctx| {
        // no check; this is here as a shell so that traversing the tree is the same for all tt types
        true
    }));
    rr_type_default.set_name("rr_type_default");
    let rr_type_default = Rc::new(rr_type_default);

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

    let resource_record = Sequence::new("resource_record", vec![
        ("name", domain.clone()),
        ("type", rr_type_field),
        ("class", uint16.clone()),
        ("ttl", uint32.clone()),
        ("body_length", uint16.clone()),
        ("rr_body", rr_body_or_unknown),
    ]);
    let mut question_set = Set::new_no_name(query.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion"].int()));
    question_set.set_name("question_set");
    let mut answer_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer"].int()));
    answer_set.set_name("answer_set");
    let mut authority_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority"].int()));
    authority_set.set_name("authority_set");
    let mut additional_set = Set::new_no_name(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional"].int()));
    additional_set.set_name("additional_set");
    let result = Rc::new("result", vec!ChildMap::from([
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
    ]));
}
