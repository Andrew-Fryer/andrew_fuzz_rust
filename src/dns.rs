use std::{collections::HashMap, rc::Rc};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint}, core::{DataModel, context::Context}};

pub fn dns() -> Box<dyn DataModel> {
    let uint8: Rc<dyn DataModel> = Rc::new(U8::new());
    let uint16: Rc<dyn DataModel> = Rc::new(U16::new());
    let uint32: Rc<dyn DataModel> = Rc::new(Sequence::new(HashMap::from([
        ("b0".to_string(), uint8.clone()),
        ("b1".to_string(), uint8.clone()),
        ("b2".to_string(), uint8.clone()),
        ("b3".to_string(), uint8.clone()),
    ])));
    let label: Rc<dyn DataModel> = Rc::new(Union::new(Rc::new(vec![
        Box::new(Sequence::new(HashMap::from([
            ("length".to_string(), Rc::new(Constraint::new(uint8.clone(), Rc::new(|ctx| ctx.child().int() >= 0xc0))) as Rc<dyn DataModel>),
            ("letters".to_string(), Rc::new(Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"length".to_string()].int()))) as Rc<dyn DataModel>),
        ]))),
        Box::new(Sequence::new(HashMap::from([
            ("marker".to_string(), Rc::new(Constraint::new(uint8.clone(), Rc::new(|ctx| ctx.child().int() >= 0xc0))) as Rc<dyn DataModel>),
            ("ref".to_string(), uint8.clone()),
        ]))),
    ]), uint8.clone()));
    let domain_predicate: Rc<dyn Fn(Rc<Context>) -> bool> = Rc::new(|ctx| {
        let v = ctx.vec();
        let v_len = v.len();
        if v_len <= 0 {
            return false;
        }
        let last_element = v[v_len - 1].clone();
        let last_element_len = last_element.map()[&"length".to_string()].int();
        last_element_len == 0 || last_element_len > 0xc0
    });
    let domain: Rc<dyn DataModel> = Rc::new(Set::new(label.clone(), vec![label.clone()], domain_predicate));
    let rr_a: Box<dyn DataModel> = Box::new(Sequence::new(HashMap::from([
        ("name".to_string(), domain.clone()),
        ("type".to_string(), uint16.clone()),
        ("class".to_string(), uint16.clone()),
        ("ttl".to_string(), uint32.clone()),
        ("dataLength".to_string(), uint16.clone()),
        ("data".to_string(), Rc::new(Set::new(uint8.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"dataLength".to_string()].int())))),
    ])));
    // let rr_aaaa = todo!();
    let resource_record = Rc::new(Union::new(Rc::new(vec![rr_a.clone()]), Rc::from(rr_a))); //, rr_aaaa));
    let result = Box::new(Sequence::new(HashMap::from([
        ("transactionId".to_string(), uint16.clone()),
        ("flags".to_string(), uint16.clone()),
        ("numQuestion".to_string(), uint16.clone()),
        ("numAnswer".to_string(), uint16.clone()),
        ("numAuthority".to_string(), uint16.clone()),
        ("numAdditional".to_string(), uint16.clone()),
        ("question".to_string(), Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion".to_string()].int())))),
        ("answer".to_string(), Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer".to_string()].int())))),
        ("authority".to_string(), Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority".to_string()].int())))),
        ("additional".to_string(), Rc::new(Set::new(resource_record.clone(), Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional".to_string()].int())))),
        ("end".to_string(), Rc::new(Button::new())),
    ])));
    result
}