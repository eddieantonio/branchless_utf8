pub mod implementations;

pub fn print_encoded(c: char) {
    use implementations::branchless_01_naive::encode_one;

    let encoded = encode_one(c);
    let decoded = {
        let code_units = encoded.to_be_bytes();
        let offset = std::cmp::min(3, encoded.leading_zeros() as usize / 8);
        let s = std::str::from_utf8(&code_units[offset..]);
        s.map(String::from)
    };
    println!("{encoded:08X} {encoded:032b} {decoded:?}");
}

pub fn print_encode_str(s: &str) {
    use implementations::branchless_01_naive::encode;

    let chars: Vec<_> = s.chars().collect();
    let encoded = encode(&chars);
    println!("{encoded:?}");
}

#[cfg(test)]
pub mod common_test {
    pub const RANDOM_128: &str = include_str!("random_128.txt");
}
