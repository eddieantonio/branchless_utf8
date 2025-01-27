use divan::Bencher;

fn main() {
    divan::main();
}

#[derive(Clone, Copy, Debug)]
enum TestCase {
    Utf8Sample,
    Test1m,
}

/// Rust built-in implementation
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn impl01_rust_collect_string(bencher: Bencher, t: TestCase) {
    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| example.into_iter().collect::<String>())
}

/// "Textbook" implementation -- match/case
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn impl02_textbook_implementation(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::textbook::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// First version of the branchless implementation:
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn impl03_naive_branchless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::branchless_01_naive::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// A better scalar, branchless implementation
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn impl04_scalar_branchless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::branchless_02_autovectorized::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// A simd implementation, using wide -- turns out LLVM autovectorizes better than this
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn impl05_simd_branchless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::branchless_03_explicit_simd::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

// --- Helper ---

impl TestCase {
    fn load_utf32(self) -> Vec<char> {
        match self {
            TestCase::Utf8Sample => include_str!("utf8-sample.txt").chars().collect(),
            TestCase::Test1m => include_str!("test_1m.txt").chars().collect(),
        }
    }
}
