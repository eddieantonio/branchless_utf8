pub mod implementations;

pub fn print_encoded(c: char) {
    use implementations::naive_branchless;

    let encoded = naive_branchless::encode_one(c);
    let decoded = {
        let code_units = encoded.to_be_bytes();
        let offset = std::cmp::min(3, encoded.leading_zeros() as usize / 8);
        let s = std::str::from_utf8(&code_units[offset..]);
        s.map(String::from)
    };
    println!("{encoded:08X} {encoded:032b} {decoded:?}");
}

pub fn print_encode_str(s: &str) {
    use implementations::naive_branchless;

    let chars: Vec<_> = s.chars().collect();
    let encoded = naive_branchless::encode(&chars);
    println!("{encoded:?}");
}
