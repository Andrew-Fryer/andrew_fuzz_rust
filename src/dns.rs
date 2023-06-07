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

    let mut opt_records = Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| {
        let current_len = ctx.vec().len() as i32;
        let data_length = ctx.parent().map()[&"data_length".to_string()].int();
        current_len == data_length
    }));
    opt_records.set_name("opt_records");
    let opt_records = Rc::new(opt_records);

    // let mut rr_A = todo!();
    // let mut rr_NS = todo!();
    // let mut rr_CNAME = todo!();
    // let mut rr_SOA = todo!();
    // let mut rr_PTR = todo!();
    // let mut rr_MX = todo!();
    // let mut rr_TXT = todo!();
    // let mut rr_AAAA = todo!();
    let mut rr_OPT = Sequence::new(ChildMap::from([
        ("udp_payload_size", uint16.clone()),
        ("extended_r_code", uint8.clone()),
        ("version", uint8.clone()),
        ("d0_z", uint16.clone()),
        ("data_length", uint16.clone()),
        ("opt_records", opt_records), // oh boy, now this is nested headers again!
    ]));
    rr_OPT.set_name("rr_OPT");
    let rr_OPT = Box::new(rr_OPT);
    // let mut rr_DS = todo!();
    // let mut rr_RRSIG = todo!();
    // let mut rr_KEY = todo!();
    // let mut rr_NSEC3 = todo!();
    
    // Should return an AST when the body is malformed, but we can still parse the rest of it?
        // I think so... because that means that we'll have a better approximation for code coverage...
    // That would mean that we'll recover from failing to parse this by just reading in `dataLength` bytes.
    // I could create a new non-terminal that fails it the child read too far and eats the rest if the child didn't read enough.
    let dummy = uint8.clone(); // this is a silly placeholder
    let mut resouce_record_body = Union::new(Rc::new(vec![
        rr_OPT,
    ]), dummy.clone());


    // TODO: add all resource types and make sure that makes my feature vectors longer!
    let mut resource_record = Sequence::new(ChildMap::from([
        ("name", domain.clone()),
        ("type", uint16.clone()),
        ("class", uint16.clone()),
        ("ttl", uint32.clone()),
        ("dataLength", uint16.clone()),
        ("data", Rc::new(rr_data_set)),
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