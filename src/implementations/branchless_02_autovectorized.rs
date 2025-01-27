//! An updated version of branchless_01_naive that uses a rew more tricks.
use std::mem::{self, MaybeUninit};

const LUT: [u32; 33] = [
    0, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
    0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
    0xE08080, 0xE08080, 0xE08080, 0xE08080, 0xE08080, 0xC080, 0xC080, 0xC080, 0xC080, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

pub fn encode(chars: &[char]) -> Result<String, std::string::FromUtf8Error> {
    // Forward pass: encode to UTF-8, but leave it in 32 bits
    let mut final_size = 0;

    let fat_utf8: Vec<u32> = chars
        .iter()
        .copied()
        .map(|orig| {
            let scalar_value: u32 = orig.into();
            let n_zeros = scalar_value.leading_zeros() as usize;

            let a = scalar_value & 0b111_000000_000000_000000;
            let b = scalar_value & 0b000_111111_000000_000000;
            let c = scalar_value & 0b000_000000_111111_000000;
            let d = scalar_value & 0b000_000000_000000_111111;

            // Only ASCII values with the 7th bit set need this exception:
            let shift = if n_zeros == 25 { 0 } else { 2 };

            let encoded = LUT[n_zeros] | a << 6 | b << 4 | c << shift | d;
            let added_size = utf8_size(encoded);
            final_size += added_size;
            encoded
        })
        .collect();

    // The buffer: starts off uninitialized:
    let mut buffer = vec![MaybeUninit::uninit(); final_size];

    // We must early return here when empty or else we'll subtract from 0usize which will overflow and panic
    if buffer.is_empty() {
        return Ok(String::new());
    }

    // Backwards path:
    let mut i = buffer.len();
    let mut reversed_fat_utf8 = fat_utf8.iter().copied().rev();
    while i > 4 {
        let encoded = reversed_fat_utf8.next().unwrap();
        let [a, b, c, d] = encoded.to_be_bytes();
        buffer[i - 1].write(d);
        buffer[i - 2].write(c);
        buffer[i - 3].write(b);
        buffer[i - 4].write(a);
        i = i.saturating_sub(utf8_size(encoded));
    }
    let last_written_position = i;

    // textbook the last few, this time forwards
    let mut i = 0;
    for encoded in fat_utf8.iter().copied() {
        if i >= last_written_position {
            break;
        }

        let byte_size = utf8_size(encoded);
        let [a, b, c, d] = encoded.to_be_bytes();

        assert!(i + byte_size <= buffer.len());
        match byte_size {
            1 => {
                buffer[i].write(d);
            }
            2 => {
                buffer[i].write(c);
                buffer[i + 1].write(d);
            }
            3 => {
                buffer[i].write(b);
                buffer[i + 1].write(c);
                buffer[i + 2].write(d);
            }
            4 => {
                buffer[i].write(a);
                buffer[i + 1].write(b);
                buffer[i + 2].write(c);
                buffer[i + 3].write(d);
            }
            _ => unreachable!(),
        }

        i += byte_size;
    }

    let buffer = unsafe { mem::transmute::<Vec<MaybeUninit<u8>>, Vec<u8>>(buffer) };
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
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
    fn encode_128() {
        roundtrip(crate::common_test::RANDOM_128);
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
