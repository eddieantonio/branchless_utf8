//! What would happen if you spend 2 minutes reading the Wikipedia article on UTF-8 and writing a
//! converter:
pub fn encode(chars: &[char]) -> Result<String, std::string::FromUtf8Error> {
    let mut buffer = Vec::with_capacity(chars.len());

    for c in chars.iter().copied() {
        let c = c as u32;
        match c {
            // ASCII
            0..0x80 => buffer.push(c as u8),
            0x80..0x800 => {
                // Two byte-sequences
                buffer.push(0b1100_0000 | ((c >> 6) & 0x3F) as u8);
                buffer.push(0b1000_0000 | (c & 0x3F) as u8);
            }
            0x800..=0x10000 => {
                // Three byte-sequences
                buffer.push(0b1110_0000 | ((c >> 12) & 0x3F) as u8);
                buffer.push(0b1000_0000 | ((c >> 6) & 0x3F) as u8);
                buffer.push(0b1000_0000 | (c & 0x3F) as u8);
            }
            _ => {
                // Four-byte sequences
                buffer.push(0b1111_0000 | ((c >> 18) & 0x3F) as u8);
                buffer.push(0b1000_0000 | ((c >> 12) & 0x3F) as u8);
                buffer.push(0b1000_0000 | ((c >> 6) & 0x3F) as u8);
                buffer.push(0b1000_0000 | (c & 0x3F) as u8);
            }
        }
    }

    String::from_utf8(buffer)
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn encode_ascii() {
        roundtrip("A");
        roundtrip("Hello");
    }

    #[test]
    fn encode_latin1() {
        roundtrip("ß");
    }

    #[test]
    fn encode_bmp() {
        roundtrip("文");
    }

    #[test]
    fn encode_astral() {
        roundtrip("💩");
    }

    #[test]
    fn encode_anything() {
        roundtrip("ß 文 💩 A");
    }

    #[test]
    fn encode_128() {
        roundtrip(crate::common_test::RANDOM_128);
    }

    fn roundtrip(s: &str) {
        let utf32: Vec<_> = s.chars().collect();
        assert_eq!(Ok(s), encode(&utf32).as_deref());
    }
}
