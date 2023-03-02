use std::{collections::HashMap, rc::Rc, borrow::BorrowMut};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint}, core::{DataModel, context::Context, bolts::ChildMap}};
use crate::core::Named;

pub fn dns() -> Box<dyn DataModel> {
    let uint8: Rc<dyn DataModel> = Rc::new(U8::new());
    let uint16: Rc<dyn DataModel> = Rc::new(U16::new());
    let uint32: Rc<dyn DataModel> = Rc::new(Sequence::new(ChildMap::from([
        ("b0", uint8.clone()),
        ("b1", uint8.clone()),
        ("b2", uint8.clone()),
        ("b3", uint8.clone()),
    ])));
    let mut label = Union::new(Rc::new(vec![
        Box::new(Sequence::new(ChildMap::from([
            ("length", Rc::new(Constraint::new(uint8.clone(), Rc::new(|ctx| ctx.child().int() < 0xc0))) as Rc<dyn DataModel>),
            ("letters", Rc::new(Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"length".to_string()].child().int()))) as Rc<dyn DataModel>),
        ]))),
        Box::new(Sequence::new(ChildMap::from([
            ("marker", Rc::new(Constraint::new(uint8.clone(), Rc::new(|ctx| ctx.child().int() >= 0xc0))) as Rc<dyn DataModel>),
            ("ref", uint8.clone()),
        ]))),
    ]), uint8.clone());
    label.set_name(&"label");
    let label: Rc<dyn DataModel> = Rc::new(label);
    let domain_predicate: Rc<dyn Fn(Rc<Context>) -> bool> = Rc::new(|ctx| {
        let v = ctx.vec();
        let v_len = v.len();
        if v_len <= 0 {
            return false;
        }
        let last_element = v[v_len - 1].clone();
        let last_element_len = last_element.child().map()[&"length".to_string()].child().int();
        last_element_len == 0 || last_element_len > 0xc0
    });
    let mut domain = Set::new(label.clone(), vec![label.clone()], domain_predicate);
    domain.set_name(&"domain");
    let domain: Rc<dyn DataModel> = Rc::new(domain);
    let rr_a: Box<dyn DataModel> = Box::new(Sequence::new(ChildMap::from([
        ("name", domain.clone()),
        ("type", uint16.clone()),
        ("class", uint16.clone()),
        ("ttl", uint32.clone()),
        ("dataLength", uint16.clone()),
        ("data", Rc::new(Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"dataLength".to_string()].int())))),
    ])));
    // let rr_aaaa = todo!();
    let mut resource_record = Union::new(Rc::new(vec![rr_a.clone()]), Rc::from(rr_a)); //, rr_aaaa));
    resource_record.set_name(&"rr");
    let resource_record = Rc::new(resource_record);
    let mut result = Box::new(Sequence::new(ChildMap::from([
        ("transactionId", uint16.clone()),
        ("flags", uint16.clone()),
        ("numQuestion", uint16.clone()),
        ("numAnswer", uint16.clone()),
        ("numAuthority", uint16.clone()),
        ("numAdditional", uint16.clone()),
        ("question", Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion"].int())))),
        ("answer", Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer".to_string()].int())))),
        ("authority", Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority".to_string()].int())))),
        ("additional", Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional".to_string()].int())))),
        ("end", Rc::new(Button::new())),
    ])));
    result.set_name(&"dns");
    result
}