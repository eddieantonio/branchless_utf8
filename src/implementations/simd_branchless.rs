use std::mem::{self, MaybeUninit};

use wide::u32x4;

const LUT: [u32; 33] = [
    0, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
    0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
    0xE08080, 0xE08080, 0xE08080, 0xE08080, 0xE08080, 0xC080, 0xC080, 0xC080, 0xC080, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
const SHIFT_LUT: [u32; 33] = [
    0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0,
];

pub fn encode(chars: &[char]) -> Result<String, std::string::FromUtf8Error> {
    // Does this assumption always hold? I hope so!
    let chars = unsafe { mem::transmute::<&[char], &[u32]>(chars) };

    // Forward pass: encode to UTF-8, but leave it in 32 bits
    let mut final_size: usize = 0;
    let mut fat_utf8 = Vec::<u32>::with_capacity(chars.len());

    // Do 4 code points at a time
    let mut chunks = chars.chunks_exact(4);
    for chunk in &mut chunks {
        let mut values = [0u32; 4];
        values.clone_from_slice(chunk);

        let n_zeros = [
            values[0].leading_zeros() as usize,
            values[1].leading_zeros() as usize,
            values[2].leading_zeros() as usize,
            values[3].leading_zeros() as usize,
        ];
        let overlay = u32x4::new([
            LUT[n_zeros[0]],
            LUT[n_zeros[1]],
            LUT[n_zeros[2]],
            LUT[n_zeros[3]],
        ]);
        let shift_amount = u32x4::new([
            SHIFT_LUT[n_zeros[0]],
            SHIFT_LUT[n_zeros[1]],
            SHIFT_LUT[n_zeros[2]],
            SHIFT_LUT[n_zeros[3]],
        ]);

        let values = u32x4::new(values);

        let a = values & u32x4::splat(0b111_000000_000000_000000);
        let b = values & u32x4::splat(0b000_111111_000000_000000);
        let c = values & u32x4::splat(0b000_000000_111111_000000);
        let d = values & u32x4::splat(0b000_000000_000000_111111);

        let encoded: u32x4 = overlay | a << 6 | b << 4 | c << shift_amount | d;
        let [a1, a2, a3, a4] = encoded.utf8_size().to_array();
        final_size += (a1 + a2 + a3 + a4) as usize;
        let encoded = encoded.to_array();
        fat_utf8.extend(&encoded);
    }

    // Scalar tail (forward-pass):
    for scalar_value in chunks.remainder().iter().copied() {
        let n_zeros = scalar_value.leading_zeros() as usize;

        let a = scalar_value & 0b111_000000_000000_000000;
        let b = scalar_value & 0b000_111111_000000_000000;
        let c = scalar_value & 0b000_000000_111111_000000;
        let d = scalar_value & 0b000_000000_000000_111111;

        let encoded = LUT[n_zeros] | a << 6 | b << 4 | c << SHIFT_LUT[n_zeros] | d;
        let added_size = encoded.utf8_size();
        final_size += added_size;
        fat_utf8.push(encoded)
    }

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
        i = i.saturating_sub(encoded.utf8_size());
    }
    let last_written_position = i;

    // textbook the last few, this time forwards
    let mut i = 0;
    for encoded in fat_utf8.iter().copied() {
        if i >= last_written_position {
            break;
        }

        let byte_size = encoded.utf8_size();
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

trait Utf8Size: Copy {
    type Output;

    fn utf8_size(self) -> Self::Output;
}

impl Utf8Size for u32 {
    type Output = usize;

    #[inline]
    fn utf8_size(self) -> usize {
        std::cmp::max(1, (u32::BITS - self.leading_zeros()) as usize / 8)
    }
}

impl Utf8Size for u32x4 {
    type Output = u32x4;

    #[inline]
    fn utf8_size(self) -> Self {
        let values = self.to_array();
        let n_zeros = u32x4::new([
            values[0].leading_zeros(),
            values[1].leading_zeros(),
            values[2].leading_zeros(),
            values[3].leading_zeros(),
        ]);
        let possibly_zero = (u32x4::splat(u32::BITS) - n_zeros) >> 3;
        u32x4::splat(1).max(possibly_zero)
    }
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
