use std::{rc::Rc, cell::RefCell, ops::{AddAssign, Add}, cmp::PartialEq, fmt::Debug};

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
    pub fn new(data: Vec<u8>, num_bits: Option<i32>) -> Self {
        // num_bits allows the caller to specify that they want non-byte-aligned data
        // we will truncate the number of bits in `data` to match num_bits
        if let Some(num_bits) = num_bits {
            todo!();
        }
        let len = data.len();
        Self {
            data: Rc::new(RefCell::new(data)),
            pos: 0,
            len: len as i32 * 8,
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            pos: self.pos,
            len: self.len,
        }
    }
    pub fn peek(&self, num_bits: u8) -> u8 {
        // please only use this if you know what you're doing
        // this shoud really only be visible to this file
        if num_bits > 8 {
            panic!();
        }
        if num_bits != 8 {
            todo!();
        }
        self.data.borrow()[(self.pos / 8) as usize]
    }
    pub fn clean_offset(&self) -> bool {
        self.pos % 8 == 0
    }
    pub fn len(&self) -> i32 {
        self.len
    }
    pub fn eat(&mut self, num_bits: i32) -> Option<Self> {
        // This is efficient because the only mutation we do to the underlying vec is extending it.
        // So, when we `eat`, we can share the underlying `Vec<u8>`.

        if num_bits > self.len {
            return None;
        }

        let result = if num_bits % 8 == 0 {
            if num_bits % 8 == 0 {
                Self {
                    // data: (&self.data[(self.pos)..(self.pos + num_bits)]).into(),
                    data: Rc::clone(&self.data),
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

        Some(result)
    }
    pub fn advance(&mut self, num_bits: i32) {
        self.pos += num_bits;
    }
    pub fn extend(&mut self, other: &BitArray) {
        // first check if someone else has already extended the underlying data beneath us
        if self.pos + self.len < self.data.borrow().len() as i32 {
            // if so, we are forced to do a deep clone
            // todo: don't copy the beginning elements if we don't need to
            let cloned_data = Rc::new(RefCell::new(self.data.borrow().clone()));
            self.data = cloned_data;
        }
        if !other.clean_offset() {
            todo!();
        }
        if !self.clean_offset() {
            todo!();
        }
        let mut other = other.clone();
        while other.len() > 0 {
            let b = other.peek(8);
            self.data.borrow_mut().push(b);
        }
        self.len += other.len();
    }
    // pub fn get(&self) -> 
}

impl AddAssign for BitArray {
    fn add_assign(&mut self, rhs: Self) {
        self.extend(&rhs);
    }
}

impl PartialEq for BitArray {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut a = self.clone();
        let mut b = other.clone();
        while a.len() > 0 {
            let a_val = a.eat(8).unwrap();
            let b_val = b.eat(8).unwrap();
            if a_val.peek(8) != b_val.peek(8) {
                return false;
            }
        }
        true
    }
}

// impl Debug for BitArray {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("BA ")

//         // let data = "".to_string();
//         let data = self.data.borrow().iter().map(|b| format!("{:#04X?}", b)).collect();

//         f.write_str(data);
//         std::fmt::Result ()
//     }
// }
