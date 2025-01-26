use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench]
fn naive_implementation(bencher: Bencher) {
    use branchless_utf8::implementations::naive_branchless::encode;

    bencher
        .with_inputs(|| load_example())
        .bench_values(|example| encode(&example))
}

#[divan::bench]
fn built_in_implementation(bencher: Bencher) {
    bencher
        .with_inputs(|| load_example())
        .bench_values(|example| example.into_iter().collect::<String>())
}

fn load_example() -> Vec<char> {
    let example = include_str!("utf8-sample.txt");
    example.chars().collect()
}
