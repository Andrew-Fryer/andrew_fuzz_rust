use std::{collections::HashMap, rc::Rc};

use crate::{library::{set::Set, sequence::Sequence, u8::U8, u16::U16, button::Button}, core::DataModel};

pub fn dns() -> Box<dyn DataModel> {
    let resource_record = todo!();
    // Box::new(Sequence::new(HashMap::from([
    //     ("transactionId", U16::new()),
    //     ("flags", U16::new()),
    //     ("numQuestion", U16::new()),
    //     ("numAnswer", U16::new()),
    //     ("numAuthority", U16::new()),
    //     ("numAdditional", U16::new()),
    //     ("question", Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()["numQuestion"].int()))),
    //     ("answer", Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()["numAnswer"].int()))),
    //     ("authority", Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()["numAuthority"].int()))),
    //     ("additional", Set::new(resource_record, Vec::new(), Rc::new(|ctx| ctx.vec().len() as i32 == ctx.parent().map()["numAdditional"].int()))),
    //     ("end", Button::new()),
    // ])));
    todo!()
}