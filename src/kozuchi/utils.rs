use std::{fs::File, io::Bytes, io::Read};

pub fn iterate_bytes(input: File) -> impl Iterator<Item = u8> {
    struct IteratedBytes {
        input: Bytes<File>,
    }

    impl Iterator for IteratedBytes {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self.input.next() {
                Some(Ok(byte)) => Some(byte),
                _ => None,
            }
        }
    }

    IteratedBytes {
        input: input.bytes(),
    }
}
