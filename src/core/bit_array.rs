use core::num;
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
    pub fn new(mut data: Vec<u8>, num_bits: Option<i32>) -> Self {
        // num_bits allows the caller to specify that they want non-byte-aligned data
        // we will truncate the number of bits in `data` to match num_bits
        let len = if let Some(num_bits) = num_bits {
            let data_len = data.len();
            let num_bits_to_zero = (8 - (num_bits % 8)) % 8;
            data[data_len - 1] >>= num_bits_to_zero;
            data[data_len - 1] <<= num_bits_to_zero;
            // data[data_len - 1].checked_shl(num_bits_to_zero).unwrap_or(0);
            // data[data_len - 1].checked_shr(num_bits_to_zero).unwrap_or(0);
            if num_bits > data.len() as i32 * 8 {
                panic!();
            }
            num_bits
        } else {
            data.len() as i32 * 8
        };
        Self {
            data: Rc::new(RefCell::new(data)),
            pos: 0,
            len,
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
        if num_bits == 0 {
            return 0;
        }
        if num_bits as i32 > self.len {
            panic!();
        }
        if self.pos % 8 == 0 && num_bits == 8 {
            self.data.borrow()[(self.pos / 8) as usize]
        } else {
            let head_start = self.pos % 8;
            let mut head_bits = self.data.borrow()[(self.pos / 8) as usize];
            head_bits <<= head_start;
            let mut head_end = head_start + num_bits as i32;
            let tail_start = 0;
            let mut tail_end = 0;
            if head_end > 8 {
                tail_end = head_end - 8;
                head_end = 8;

                // let mut tail_bits = self.data.borrow()[((self.pos + num_bits as i32) / 8) as usize];
                let mut tail_bits = self.data.borrow()[((self.pos as i32) / 8 + 1) as usize];
                let tail_bits_to_zero = 8 - tail_end;
                tail_bits >>= tail_bits_to_zero;
                tail_bits <<= tail_bits_to_zero;
                tail_bits >>= head_end - head_start;
                head_bits | tail_bits
            } else {
                let head_bits_to_zero = 8 - head_end;
                head_bits >>= head_start + head_bits_to_zero;
                head_bits <<= head_start + head_bits_to_zero;
                head_bits
            }
        }
    }
    pub fn clean_offset(&self) -> bool {
        self.pos % 8 == 0
    }
    pub fn clean_end_offset(&self) -> bool {
        (self.pos + self.len) % 8 == 0
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

        let result = Self {
            data: Rc::clone(&self.data),
            pos: self.pos,
            len: num_bits,
        };

        self.pos += num_bits;
        self.len -= num_bits;

        Some(result)
    }
    pub fn advance(&mut self, num_bits: i32) {
        self.pos += num_bits;
    }
    // todo: make this lazy
    pub fn extend(&mut self, other: &BitArray) {
        // first check if someone else has already extended the underlying data beneath us
        if self.pos + self.len < self.data.borrow().len() as i32 {
            // if so, we are forced to do a deep clone
            // todo: don't copy the beginning elements if we don't need to
            let cloned_data = Rc::new(RefCell::new(self.data.borrow().clone()));
            self.data = cloned_data;
        }
        // We don't want to mutate `other`, so we clone it.
        let mut other = other.clone();
        let other_len = other.len();
        let mut data = self.data.borrow_mut();
        if !self.clean_end_offset() {
            let num_bits_free_in_self = (8 - ((self.pos + self.len) % 8) % 8);
            let num_bits = if num_bits_free_in_self < other.len() {
                num_bits_free_in_self
            } else {
                other.len()
            };
            let data_len = data.len();
            data[data_len - 1] |= other.eat(num_bits).unwrap().peek(num_bits as u8) >> (8 - num_bits_free_in_self);
        }
        while other.len() > 0 {
            let num_bits = if other.len() > 8 {
                8
            } else {
                other.len()
            };
            let b = other.eat(num_bits).unwrap().peek(num_bits as u8);
            data.push(b); // for some reason, this isn't mutating self.data <- I think it just isn't showing up in the debugger...
        }
        self.len += other_len;
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
            let a_len = a.len();
            let num_bits = if a_len > 8 {
                8
            } else {
                a_len
            };
            let a_val = a.eat(num_bits).unwrap();
            let b_val = b.eat(num_bits).unwrap();
            let t1 = a_val.peek(num_bits as u8);
            let t2 = b_val.peek(num_bits as u8);
            if t1 != t2 {
                return false;
            }
        }
        true
    }
}

impl Debug for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("BA ")

        // // let data = "".to_string();
        // let data = self.data.borrow().iter().map(|b| format!("{:#04X?}", b)).collect();

        // f.write_str(data);
        // std::fmt::Result ()
        println!("{:02X?}", self.data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bit_array::BitArray;

    #[test]
    fn simple() {
        let data = vec![0x00, 0xA0, 0x66, 0b1111_1011, 65];
        let mut ba = BitArray::new(data, None);
        assert_eq!(ba.eat(8), Some(BitArray::new(vec![0x00], None)));
        assert_eq!(ba.eat(8), Some(BitArray::new(vec![0xA0], None)));
        assert_eq!(ba, BitArray::new(vec![0x66, 0b1111_1011, 65], None));
    }

    #[test]
    fn extend() {
        let data = vec![0x00, 0xA0, 0x66, 0b1111_1011, 65];
        let mut ba = BitArray::new(data, None);
        ba.extend(&BitArray::new(vec![0xFF], None));
        while ba.len() > 8 {
            ba.eat(8);
        }
        assert_eq!(ba, BitArray::new(vec![0xFF], None));
    }

    #[test]
    fn bit_wise() {
        let mut ba = BitArray::new(vec![0xA9], Some(7));
        assert_eq!(ba.eat(4), Some(BitArray::new(vec![0xA0], Some(4))));
        assert_eq!(ba.len(), 3);
        ba.extend(&BitArray::new(vec![0x7F], Some(2)));
        assert_eq!(ba.len(), 5);
        ba.extend(&BitArray::new(vec![0x0F], Some(8)));
        assert_eq!(ba.len(), 13);
        let x = 11;
        assert_eq!(ba.eat(x), Some(BitArray::new(vec![0x88, 0x60], Some(x))));
        assert_eq!(ba.len(), 2);
        assert_eq!(ba, BitArray::new(vec![0b1100_0000], Some(2)));
        assert_eq!(ba.eat(3), None);
    }

    #[test]
    fn perf() {
        let iterations = 1000 * 1000;
        let mut ba = BitArray::new(vec![0xA9], None);
        for i in 0..iterations {
            ba.extend(&BitArray::new(vec![0x7B], None));
        }
        assert_eq!(ba.eat(8), Some(BitArray::new(vec![0xA9], None)));
        for i in 0..iterations {
            assert_eq!(ba.eat(8), Some(BitArray::new(vec![0x7B], None)));
        }
        assert_eq!(ba.eat(1), None);
        assert_eq!(ba, BitArray::new(vec![], None));
    }

    #[test]
    fn perf_bit_wise() {
        let iterations = 1000 * 1000;
        let mut ba = BitArray::new(vec![0xA9], Some(7));
        for i in 0..iterations {
            ba.extend(&BitArray::new(vec![0x7F], Some(2)));
        }
        assert_eq!(ba.eat(7), Some(BitArray::new(vec![0xA8], Some(7))));
        for i in 0..iterations {
            assert_eq!(ba.eat(2), Some(BitArray::new(vec![0b0100_0000], Some(2))));
        }
        assert_eq!(ba.eat(1), None);
        assert_eq!(ba, BitArray::new(vec![], None));
    }
}
