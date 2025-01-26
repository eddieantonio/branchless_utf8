use std::mem::{self, MaybeUninit};

const LUT: [u32; 22] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xC080, 0xC080, 0xC080, 0xC080, 0xE08080, 0xE08080,
    0xE08080, 0xE08080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
];

const SHIFT_LUT: [u32; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
];

pub fn encode(chars: &[char]) -> Result<String, std::string::FromUtf8Error> {
    // Forward pass: encode to UTF-8, but leave it in 32 bits
    let fat_utf8 = to_code_points(chars);

    // The buffer: starts off uninitialized:
    let mut buffer = vec![MaybeUninit::uninit(); 4 * fat_utf8.len()];
    // We must early return here when empty or else we'll subtract from 0usize which will overflow and panic
    if buffer.is_empty() {
        return Ok(String::new());
    }

    // Backwards path:
    let mut i = buffer.len() - 1;
    // There's a better way to do this:
    let mut start = buffer.len();
    for &encoded in fat_utf8.iter().rev() {
        let advance = utf8_size(encoded);
        start -= advance;
        buffer[i].write((encoded & 0xFF) as u8);
        buffer[i - 1].write(((encoded & 0xFF00) >> 8) as u8);
        buffer[i - 2].write(((encoded & 0xFF0000) >> 16) as u8);
        buffer[i - 3].write(((encoded & 0xFF000000) >> 24) as u8);
        i = i.saturating_sub(advance);
    }

    let buffer = unsafe { mem::transmute::<Vec<MaybeUninit<u8>>, Vec<u8>>(buffer) };
    // Uh: did I not just defeat the point of uniniting a lot of memory?
    // TODO: use String::from_unchecked() which is another transmute:
    String::from_utf8(buffer[start..].to_vec())
}

pub fn encode_one(c: char) -> u32 {
    let scalar_value: u32 = c.into();
    // TODO: invert:
    let n_bits = (u32::BITS - scalar_value.leading_zeros()) as usize;

    let a = scalar_value & 0b1111_000000_000000_000000;
    let b = scalar_value & 0b0000_111111_000000_000000;
    let c = scalar_value & 0b0000_000000_111111_000000;
    let d = scalar_value & 0b0000_000000_000000_111111;

    LUT[n_bits] | a << 6 | b << 4 | c << SHIFT_LUT[n_bits] | d
}

fn to_code_points(chars: &[char]) -> Vec<u32> {
    chars.iter().copied().map(encode_one).collect()
}

fn utf8_size(encoded: u32) -> usize {
    std::cmp::max(1, (u32::BITS - encoded.leading_zeros()) as usize / 8)
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

    fn roundtrip(s: &str) {
        let utf32: Vec<_> = s.chars().collect();
        assert_eq!(Ok(s), encode(&utf32).as_deref());
    }
}
