use std::{collections::HashMap, rc::Rc};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button}, core::DataModel};

pub fn dns() -> Box<dyn DataModel> {
    let u16: Rc<dyn DataModel> = Rc::new(U16::new());
    let resource_record = todo!();
    let result = Box::new(Sequence::new(HashMap::from([
        ("transactionId".to_string(), u16.clone()),
        ("flags".to_string(), u16.clone()),
        ("numQuestion".to_string(), u16.clone()),
        ("numAnswer".to_string(), u16.clone()),
        ("numAuthority".to_string(), u16.clone()),
        ("numAdditional".to_string(), u16.clone()),
        ("question".to_string(), Rc::new(Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numQuestion".to_string()].int())))),
        ("answer".to_string(), Rc::new(Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAnswer".to_string()].int())))),
        ("authority".to_string(), Rc::new(Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAuthority".to_string()].int())))),
        ("additional".to_string(), Rc::new(Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()[&"numAdditional".to_string()].int())))),
        ("end".to_string(), Rc::new(Button::new())),
    ])));
    result
}