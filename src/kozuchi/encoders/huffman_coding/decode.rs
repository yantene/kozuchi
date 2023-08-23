use std::{iter::Peekable, marker::PhantomData};

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Decoded<'a, I: Iterator<Item = u8> + 'a> {
        input: Peekable<I>,
        queue: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Decoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.input.peek().is_none() {
                return None;
            }

            let index = self.input.next().unwrap();

            let byte = self.queue[index as usize];

            self.queue.remove(index as usize);
            self.queue.insert(0, byte);

            Some(byte as u8)
        }
    }

    Decoded {
        input: input.peekable(),
        queue: (0..=255).collect::<Vec<_>>(),
        _marker: PhantomData,
    }
}
