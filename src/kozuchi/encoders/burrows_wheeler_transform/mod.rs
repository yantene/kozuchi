// Chunk size:  2^(8 * BYTE_WIDTH)
pub const BYTE_WIDTH: usize = 2;

mod encode;

pub fn encode<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    encode::run(input)
}

mod decode;

pub fn decode<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    decode::run(input)
}
