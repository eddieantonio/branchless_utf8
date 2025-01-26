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
fn built_in_implementation(bencher: Bencher, t: TestCase) {
    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| example.into_iter().collect::<String>())
}

/// "Textbook" implementation -- match/case
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn textbook_implementation(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::textbook::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// First version of the branchless implementation:
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn naive_implementation(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::naive_branchless::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// A better scalar, branchless implementation
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn scalar_branchless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::scalar_branchless::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// Same as above, but removes a look-up table
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn scalar_branchless_no_shift_lut(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::scalar_branchless_no_shift_lut::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// A simd implementation, using wide -- turns out LLVM autovectorizes better than this
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn simd_branchless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::simd_branchless::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

/// A simd implementation, using wide -- turns out LLVM autovectorizes better than this
#[divan::bench(args = [TestCase::Utf8Sample, TestCase::Test1m])]
fn simd_branchless_lutless(bencher: Bencher, t: TestCase) {
    use branchless_utf8::implementations::simd_branchless_lutless::encode;

    bencher
        .with_inputs(|| t.load_utf32())
        .bench_values(|example| encode(&example))
}

// --- helper ---

impl TestCase {
    fn load_utf32(self) -> Vec<char> {
        match self {
            TestCase::Utf8Sample => include_str!("utf8-sample.txt").chars().collect(),
            TestCase::Test1m => include_str!("test_1m.txt").chars().collect(),
        }
    }
}
