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
            false // this is hacky, and happens when there is a (marker,ref) instead of a (length,letters)
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
        let prev_fields_len: i32 = ctx.parent().map().vals().iter().map(|dm| dm.serialize().len()).sum();
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().parent().map()[&"body_length".to_string()].int();
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
    let rr_sig = Box::new(rr_sig);
    
    // Should return an AST when the body is malformed, but we can still parse the rest of it?
        // I think so... because that means that we'll have a better approximation for code coverage...
    // That would mean that we'll recover from failing to parse this by just reading in `dataLength` bytes.
    // I could create a new non-terminal that fails it the child read too far and eats the rest if the child didn't read enough.
    let dummy = uint8.clone(); // this is a silly placeholder that would be used if the grammar was used for generational fuzzing
    let mut rr_body_union = Union::new(Rc::new(vec![
        // rr_opt, // note that this parses the same as unknown
        rr_sig
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
        let actual_data_len = ctx.child().serialize().len();
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

    let mut rr_type_opt = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 41
    }));
    rr_type_opt.set_name("rr_type_opt");
    let rr_type_opt = Box::new(rr_type_opt);

    let mut rr_type_sig = Constraint::new(uint16.clone(), Rc::new(|ctx| {
        ctx.child().int() == 46
    }));
    rr_type_sig.set_name("rr_type_sig");
    let rr_type_sig = Box::new(rr_type_sig);

    let mut rr_type_field = Union::new(Rc::new(vec![
        // rr_type_a,
        rr_type_opt,
        rr_type_sig,
        Box::new(U16::new()), // default (which will cause ambiguity, but whatever); I could do this a bit better by adding an OrderedUnion non-terminal
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
    let mut answer_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer".to_string()].int()));
    answer_set.set_name("answer_set");
    let mut authority_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority".to_string()].int()));
    authority_set.set_name("authority_set");
    let mut additional_set = Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional".to_string()].int()));
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