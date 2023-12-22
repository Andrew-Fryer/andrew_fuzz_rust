use std::rc::Rc;
use crate::{library::{sequence::Sequence, u8::U8, u16::U16, button::Button, union::Union, constraint::Constraint}, core::RcDataModel};
pub fn simple() -> RcDataModel {
    let uint8 = U8::new();
    let uint16 = U16::new();
    return Sequence::new("root_node", vec![
        ("first_field", uint8.clone()),
        ("second_field", uint16.clone()),
        ("field_foo", Union::new("foo_node", vec![
            Constraint::new("foo_constraint_node", uint8.clone(), Rc::new(|ctx| {
                ctx.child().int() % 7 == 0
            })),
            uint16.clone(),
        ])),
        ("end", Button::new()),
    ]);
}
