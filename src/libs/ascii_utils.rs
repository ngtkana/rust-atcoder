use ascii::ToAsciiChar;

pub fn lowercase_to_index(ch: ascii::AsciiChar) -> usize {
    usize::from(ch.as_byte() - b'a')
}

pub fn index_to_lowercase(index: usize) -> ascii::AsciiChar {
    (index as u8 + b'a').to_ascii_char().unwrap()
}
