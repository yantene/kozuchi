use std::{iter::Peekable, marker::PhantomData};

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Encoded<'a, I: Iterator<Item = u8> + 'a> {
        input: Peekable<I>,
        queue: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Encoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.input.peek().is_none() {
                return None;
            }

            let byte = self.input.next().unwrap();

            let index = self.queue.iter().position(|&x| x == byte).unwrap();

            self.queue.remove(index);
            self.queue.insert(0, byte);

            Some(index as u8)
        }
    }

    Encoded {
        input: input.peekable(),
        queue: (0..=255).collect::<Vec<_>>(),
        _marker: PhantomData,
    }
}
