use std::{collections::HashMap, rc::{Rc, Weak}, borrow::BorrowMut, process::Child};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint}, core::{DataModel, context::{Context, Children}, bolts::ChildMap}};
use crate::core::Named;

pub fn dns() -> Box<dyn DataModel> {
    let uint8: Rc<dyn DataModel> = Rc::new(U8::new());
    let uint16: Rc<dyn DataModel> = Rc::new(U16::new());
    let mut u32_sequence = Sequence::new(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
    ]));
    u32_sequence.set_name("u32_sequence");
    let uint32: Rc<dyn DataModel> = Rc::new(u32_sequence);
    let mut u48_sequence = Sequence::new(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
        ("b4", uint8.clone()),
        ("b5", uint8.clone()),
    ]));
    u48_sequence.set_name("u48_sequence");
    let uint48: Rc<dyn DataModel> = Rc::new(u48_sequence);
    let mut letter_set = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let len = ctx.vec().len() as i32;
        let len_field = ctx.parent().map()[&"length".to_string()].child().int();
        let result = len == len_field;
        if result {
            // println!("Parsed all {:} letters", len);
        }
        result
    }));
    letter_set.set_name("letter_set");
    let mut length_constraint = Constraint::new(uint8.clone(), Rc::new(|ctx| {
        let len_field = ctx.child().int();
        let result = len_field < 0xc0;
        if !result {
            // println!("Failed to parse label length");
        }
        result
    }));
    length_constraint.set_name("length_constraint");
    let mut length_sequence = Sequence::new(ChildMap::from([
        ("length", Rc::new(length_constraint) as Rc<dyn DataModel>),
        ("letters", Rc::new(letter_set) as Rc<dyn DataModel>),
    ]));
    length_sequence.set_name("length_sequence");
    let mut marker_constraint = Constraint::new(uint8.clone(), Rc::new(|ctx| {
        let marker_value = ctx.child().int();
        let result = marker_value >= 0xc0;
        if !result {
            // println!("Failed to parse marker");
        }
        result
    }));
    marker_constraint.set_name("marker_constraint");
    let mut marker_sequence = Sequence::new(ChildMap::from([
        ("marker", Rc::new(marker_constraint) as Rc<dyn DataModel>),
        ("ref", uint8.clone()),
    ]));
    marker_sequence.set_name("marker_sequence");
    let mut label = Union::new(Rc::new(vec![
        Box::new(length_sequence),
        Box::new(marker_sequence),
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
            last_element_len == 0 || last_element_len > 0xc0
        } else {
            true // this is hacky, and happens when there is a (marker,ref) instead of a (length,letters)
        }
    });
    let mut domain = Set::new(label.clone(), vec![label.clone()], domain_predicate);
    domain.set_name(&"domain");
    let domain: Rc<dyn DataModel> = Rc::new(domain);
    let mut query = Sequence::new(ChildMap::from([
        ("name", domain.clone()),
        ("type", uint16.clone()),
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

    // let mut rr_A = todo!();
    // let mut rr_NS = todo!();
    // let mut rr_CNAME = todo!();
    // let mut rr_SOA = todo!();
    // let mut rr_PTR = todo!();
    // let mut rr_MX = todo!();
    // let mut rr_TXT = todo!();
    // let mut rr_AAAA = todo!();
    // let mut rr_DS = todo!();
    // let mut rr_KEY = todo!();
    // let mut rr_NSEC3 = todo!();

    let mut rr_sig_signature = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let prev_fields_len: i32 = ctx.parent().map().vals().iter().map(|dm| dm.serialize().len() / 8).sum();
        let current_len = ctx.vec().len() as i32;
        // Constraint doesn't make a wrapper around the context, so we need 4 (not 6) hops up the tree
        // let data_length = ctx.parent().parent().parent().parent().parent().parent().map()[&"body_length"].int();
        let data_length = ctx.parent().parent().parent().parent().map()[&"body_length"].int();
        prev_fields_len + current_len == data_length
    }));
    rr_sig_signature.set_name("rr_sig_signature");
    let rr_sig_signature = Rc::new(rr_sig_signature);
    let mut rr_sig = Sequence::new(ChildMap::from([
        ("type_cov", uint16.clone()),
        ("alg", uint8.clone()),
        ("labels", uint8.clone()),
        ("origTimeToLive", uint32.clone()),
        ("sigExp", uint32.clone()),
        ("keyTag", uint16.clone()),
        ("signName", domain.clone()),
        ("signature", rr_sig_signature),
    ]));
    rr_sig.set_name("rr_sig");
    let rr_sig = Rc::new(rr_sig);
    let rr_sig = Constraint::new(rr_sig, Rc::new(|ctx| {
        ctx.parent().parent().parent().map()[&"type"].child().child().int() == 46
    }));
    let rr_sig = Box::new(rr_sig);

    // https://datatracker.ietf.org/doc/html/rfc2845
    let mut mac = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"mac_size"].int();
        current_len == data_length
    }));
    mac.set_name("mac");
    let mac = Rc::new(mac);
    let mut other_data = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"other_data_length"].int();
        current_len == data_length
    }));
    other_data.set_name("other_data");
    let other_data = Rc::new(other_data);
    let mut rr_tsig = Sequence::new(ChildMap::from([
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
    let rr_tsig = Constraint::new(rr_tsig, Rc::new(|ctx| {
        ctx.parent().parent().parent().map()[&"type"].child().child().int() == 250
    }));
    let rr_tsig = Box::new(rr_tsig);
    
    // Should return an AST when the body is malformed, but we can still parse the rest of it?
        // I think so... because that means that we'll have a better approximation for code coverage...
    // That would mean that we'll recover from failing to parse this by just reading in `dataLength` bytes.
    // I could create a new non-terminal that fails it the child read too far and eats the rest if the child didn't read enough.
    let dummy = uint8.clone(); // this is a silly placeholder that would be used if the grammar was used for generational fuzzing
    let mut rr_body_union = Union::new(Rc::new(vec![
        // rr_opt, // note that this parses the same as unknown
        rr_sig,
        rr_tsig,
    ]), dummy.clone());
    rr_body_union.set_name("rr_body_union");
    let rr_body_union = Rc::new(rr_body_union);

    // let mut rr_body_select = Select::new(Rc::new(|ctx| {
    //     match ctx.parent().map()[&"type".to_string()].int() {
    //         // 1 => 
    //         41 => rr_opt.clone(),
    //         46 => rr_sig.clone(),
    //         _ => button.clone(), // just fail
    //     }
    // }))

    // TODO: add all resource types and make sure that makes my feature vectors longer!

    let mut rr_body_constraint = Constraint::new(rr_body_union, Rc::new(|ctx| {
        let actual_data_len = ctx.child().serialize().len() / 8;
        let required_data_length = ctx.parent().parent().map()[&"body_length".to_string()].int();
        actual_data_len == required_data_length
    }));
    rr_body_constraint.set_name("rr_body_constraint");
    let rr_body_constraint = Box::new(rr_body_constraint);

    let mut rr_body_unknown = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().map()[&"body_length".to_string()].int();
        current_len == data_length
    }));
    rr_body_unknown.set_name("rr_body_unknown");
    let mut rr_body_unknown = Box::new(rr_body_unknown);

    let mut rr_body_or_unknown = Union::new(Rc::new(vec![
        rr_body_constraint,
        rr_body_unknown,
    ]), dummy.clone());
    rr_body_or_unknown.set_name("rr_body_or_unknown");
    let rr_body_or_unknown = Rc::new(rr_body_or_unknown);



    let mut rr_type_a = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 1
    }));
    rr_type_a.set_name("rr_type_a");
    let rr_type_a = Box::new(rr_type_a);

    let mut rr_type_ns = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 2
    }));
    rr_type_ns.set_name("rr_type_ns");
    let rr_type_ns = Box::new(rr_type_ns);

    let mut rr_type_cname = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 5
    }));
    rr_type_cname.set_name("rr_type_cname");
    let rr_type_cname = Box::new(rr_type_cname);

    let mut rr_type_soa = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 6
    }));
    rr_type_soa.set_name("rr_type_soa");
    let rr_type_soa = Box::new(rr_type_soa);

    let mut rr_type_ptr = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 12
    }));
    rr_type_ptr.set_name("rr_type_ptr");
    let rr_type_ptr = Box::new(rr_type_ptr);

    let mut rr_type_mx = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 15
    }));
    rr_type_mx.set_name("rr_type_mx");
    let rr_type_mx = Box::new(rr_type_mx);

    let mut rr_type_txt = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 16
    }));
    rr_type_txt.set_name("rr_type_txt");
    let rr_type_txt = Box::new(rr_type_txt);

    let mut rr_type_aaaa = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 28
    }));
    rr_type_aaaa.set_name("rr_type_aaaa");
    let rr_type_aaaa = Box::new(rr_type_aaaa);

    let mut rr_type_opt = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));
    rr_type_opt.set_name("rr_type_opt");
    let rr_type_opt = Box::new(rr_type_opt);

    let mut rr_type_ds = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 43
    }));
    rr_type_ds.set_name("rr_type_ds");
    let rr_type_ds = Box::new(rr_type_ds);

    let mut rr_type_sig = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));
    rr_type_sig.set_name("rr_type_sig");
    let rr_type_sig = Box::new(rr_type_sig);

    let mut rr_type_key = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 48
    }));
    rr_type_key.set_name("rr_type_key");
    let rr_type_key = Box::new(rr_type_key);

    let mut rr_type_nsec3 = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 53
    }));
    rr_type_nsec3.set_name("rr_type_nsec3");
    let rr_type_nsec3 = Box::new(rr_type_nsec3);


    let mut rr_type_tsig = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 250
    }));
    rr_type_tsig.set_name("rr_type_tsig");
    let rr_type_tsig = Box::new(rr_type_tsig);


    let mut rr_type_default = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        // no check; this is here as a shell so that traversing the tree is the same for all tt types
        true
    }));
    rr_type_default.set_name("rr_type_default");
    let rr_type_default = Box::new(rr_type_default);

    let mut rr_type_field = Union::new(Rc::new(vec![
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

    let mut resource_record = Sequence::new(ChildMap::from([
        ("name", domain.clone()),
        ("type", rr_type_field),
        ("class", uint16.clone()),
        ("ttl", uint32.clone()),
        ("body_length", uint16.clone()),
        ("rr_body", rr_body_or_unknown),
    ]));
    resource_record.set_name(&"rr");
    let resource_record = Rc::new(resource_record);
    let mut question_set = Set::new(query.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion"].int()));
    question_set.set_name("question_set");
    let mut answer_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer"].int()));
    answer_set.set_name("answer_set");
    let mut authority_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority"].int()));
    authority_set.set_name("authority_set");
    let mut additional_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional"].int()));
    additional_set.set_name("additional_set");
    let mut result = Box::new(Sequence::new(ChildMap::from([
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
