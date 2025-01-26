use branchless_utf8::*;

fn main() {
    print_encoded('A');
    print_encoded('ß');
    print_encoded('文');
    print_encoded('💩');

    print_encode_str("Hello");
    print_encode_str("ß 文 💩 A");
}
