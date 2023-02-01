use std::{rc::Rc, cell::RefCell};

// I have cool ideas about how to optimize this!
// we could make it an enum that could be normal or could be a Vec<Box<BitArray>> which is sort of lazy...
// pub enum BitArray<'a> {
//     Ref {
//         data: &'a Vec<u8>,
//         offset: i32,
//         len: i32,
//     },
// }

// BitArray does bit stuff for us (on the underlying bytes).
pub struct BitArray {
    data: Rc<RefCell<Vec<u8>>>,
    pos: i32,
    len: i32,
}
impl BitArray {
    pub fn new(num_bits: i32) -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::with_capacity((num_bits / 8) as usize))),
            pos: 0,
            len: num_bits,
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            pos: self.pos,
            len: self.len,
        }
    }
    pub fn chunks(&mut self) -> &Vec<u8> {
        &self.data.borrow() // todo: make this start at pos and go until len
    }
    pub fn len(&self) -> i32 {
        self.len
    }
    pub fn eat(&mut self, num_bits: i32) -> Self {
        // This is efficient because the only mutation we do to a BitArray is extending it.
        // So, to `eat`, we can share the underlying `Vec<u8>`.
        let next_pos = self.pos + num_bits;

        let result = if num_bits % 8 == 0 {
            if num_bits % 8 == 0 {
                Self {
                    // data: (&self.data[(self.pos)..(self.pos + num_bits)]).into(),
                    data: self.data,
                    pos: self.pos,
                    len: num_bits,
                }
            } else {
                todo!();
            }
        } else {
            todo!();
        };

        self.pos += num_bits;

        result
    }
    pub fn advance(&mut self, num_bits: i32) {
        self.pos += num_bits;
    }
    pub fn extend(&mut self, other: &BitArray) {
        self.data.borrow_mut().extend_from_slice(other.chunks());
        self.len += other.len();
    }
    // pub fn get(&self) -> 
}