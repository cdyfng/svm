use std::{
    io::{Cursor, Read},
    iter::Iterator,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Nibble(pub u8);

impl Nibble {
    pub fn is_msb_on(&self) -> bool {
        let msb = self.0 & 0b_0000_1000;
        msb == 0b_0000_1000
    }

    #[inline]
    pub fn is_msb_off(&self) -> bool {
        !self.is_msb_on()
    }

    pub fn bits(&self) -> [bool; 4] {
        let msb_0 = self.0 & 0b_0000_1000 == 0b0000_1000;
        let msb_1 = self.0 & 0b_0000_0100 == 0b0000_0100;
        let msb_2 = self.0 & 0b_0000_0010 == 0b0000_0010;
        let msb_3 = self.0 & 0b_0000_0001 == 0b0000_0001;

        [msb_0, msb_1, msb_2, msb_3]
    }
}

pub struct NibbleIter<'a, 'b: 'a> {
    buf: [u8; 1],
    has_more: bool,
    first_read: bool,
    last_byte: Option<u8>,
    cursor: &'a mut Cursor<&'b [u8]>,
}

impl<'a, 'b> NibbleIter<'a, 'b> {
    pub fn new(cursor: &'a mut Cursor<&'b [u8]>) -> Self {
        Self {
            cursor,
            buf: [0; 1],
            last_byte: None,
            has_more: true,
            first_read: true,
        }
    }
}

impl<'a, 'b> Iterator for NibbleIter<'a, 'b> {
    type Item = Nibble;

    fn next(&mut self) -> Option<Nibble> {
        if self.has_more == false {
            return None;
        }

        let nibble = {
            match self.last_byte {
                None => {
                    if let Err(..) = self.cursor.read_exact(&mut self.buf) {
                        if self.first_read {
                            self.has_more = false;
                            return None;
                        } else {
                            panic!("Not enough bytes")
                        }
                    }

                    let byte = self.buf[0];
                    self.last_byte = Some(byte);

                    Nibble((byte & 0xF0) >> 4)
                }
                Some(byte) => {
                    self.last_byte = None;
                    Nibble(byte & 0x0F)
                }
            }
        };

        if nibble.is_msb_off() {
            self.has_more = false;
        }

        self.first_read = false;

        Some(nibble)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_nibble(iter: &mut NibbleIter) -> u8 {
        iter.next().unwrap().0
    }

    fn try_read_nibble(iter: &mut NibbleIter) -> Option<u8> {
        iter.next().map(|nibble| nibble.0)
    }

    #[test]
    fn nibble_iter_reads_empty_seq() {
        let vec = vec![];
        let mut cursor = Cursor::new(&vec[..]);

        let mut iter = NibbleIter::new(&mut cursor);
        assert_eq!(None, try_read_nibble(&mut iter));
    }

    #[test]
    fn nibble_iter_reads_nibbles() {
        let vec = vec![0b_1001_1111, 0b_0011_0000];
        let mut cursor = Cursor::new(&vec[..]);

        let mut iter = NibbleIter::new(&mut cursor);

        assert_eq!(0b_0000_1001, read_nibble(&mut iter));
        assert_eq!(0b_0000_1111, read_nibble(&mut iter));
        assert_eq!(0b_0000_0011, read_nibble(&mut iter));
        assert_eq!(0b_0000_0000, read_nibble(&mut iter));
        assert_eq!(None, try_read_nibble(&mut iter));
    }
}