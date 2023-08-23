use std::{iter::Peekable, marker::PhantomData};

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Encoded<'a, I: Iterator<Item = u8> + 'a> {
        input: Peekable<I>,
        bits: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Encoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if self.bits.len() >= 8 {
                    let mut byte = 0;
                    for _ in 0..8 {
                        byte = (byte << 1) + self.bits.remove(0);
                    }

                    return Some(byte);
                }

                if self.input.peek().is_none() {
                    if self.bits.is_empty() {
                        return None;
                    }

                    let mut byte = 0;
                    for _ in 0..8 {
                        if self.bits.is_empty() {
                            byte <<= 1;
                        } else {
                            byte = (byte << 1) + self.bits.remove(0);
                        }
                    }
                    return Some(byte);
                }

                let byte = self.input.next().unwrap();

                self.bits.append(&mut vec![1; byte as usize + 1]);
                self.bits.push(0);
            }
        }
    }

    Encoded {
        input: input.peekable(),
        bits: vec![],
        _marker: PhantomData,
    }
}
