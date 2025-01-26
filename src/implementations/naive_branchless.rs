pub fn encode_code_points_to_str(chars: &[char]) -> Result<String, std::string::FromUtf8Error> {
    let fat_utf8 = encode_code_points(chars);
    let mut buffer = vec![0u8; 4 * fat_utf8.len()];
    if buffer.is_empty() {
        return Ok(String::new());
    }

    // fill in the codes from the end:
    let mut i = buffer.len() - 1;
    // jank:
    let mut start = buffer.len();
    for &encoded in fat_utf8.iter().rev() {
        let advance = utf8_size(encoded);
        start -= advance;
        buffer[i] = (encoded & 0xFF) as u8;
        buffer[i - 1] = ((encoded & 0xFF00) >> 8) as u8;
        buffer[i - 2] = ((encoded & 0xFF0000) >> 16) as u8;
        buffer[i - 3] = ((encoded & 0xFF000000) >> 24) as u8;
        i = i.saturating_sub(advance);
    }

    String::from_utf8(buffer[start..].to_vec())
}

fn utf8_size(encoded: u32) -> usize {
    std::cmp::max(1, (u32::BITS - encoded.leading_zeros()) as usize / 8)
}

pub fn encode_code_points(chars: &[char]) -> Vec<u32> {
    chars.iter().copied().map(encode_one).collect()
}

pub fn encode_one(c: char) -> u32 {
    const LUT: [u32; 22] = [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xC080, 0xC080, 0xC080, 0xC080, 0xE08080, 0xE08080,
        0xE08080, 0xE08080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080, 0xF0808080,
    ];

    const SHIFT_LUT: [u32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2,
    ];

    let scalar_value: u32 = c.into();
    // TODO: invert:
    let n_bits = (u32::BITS - scalar_value.leading_zeros()) as usize;

    let a = scalar_value & 0b1111_000000_000000_000000;
    let b = scalar_value & 0b0000_111111_000000_000000;
    let c = scalar_value & 0b0000_000000_111111_000000;
    let d = scalar_value & 0b0000_000000_000000_111111;

    LUT[n_bits] | a << 6 | b << 4 | c << SHIFT_LUT[n_bits] | d
}
