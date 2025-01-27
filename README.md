# UTF-8 encoding

Various algorithms for encoding UTF-8 from a sequence of UTF-32 code
units. Some algorithms are better than others.

**tl;dr**: What I've learned from this is that LLVM is much better at
vectorizing code than I am. Also, the "textbook" algorithm is faster
than `vec.iter().collect::<String>()` for some reason.

# Results

These results were obtained on my 2021 M1 MacBook Pro (macOS 14.7).

On [The UTF-8 Sample page](./benches/utf8-sample.txt):

| Implementation                 | Fastest  | Slowest  | Median   | Mean     |
|--------------------------------|---------:|---------:|---------:|---------:|
| `.collect::<String>()`         | 23.74 µs | 54.49 µs | 24.66 µs | 25.13 µs |
| Textbook implementation        | 19.62 µs | 61.16 µs | 20.37 µs | 21.19 µs |
| Naïve branchless               | 25.24 µs | 93.49 µs | 26.27 µs | 27.07 µs |
| Autovectorized branchless      | 19.62 µs | 82.45 µs | 20.39 µs |    21 µs |
| Explicit SIMD with `wide`      | 33.45 µs | 74.58 µs |  33.6 µs |  34.6 µs |

On about 1 million random code points ([Do not open this file](./benches/test_1m.txt)):

| Implementation                 | Fastest  | Slowest  | Median   | Mean     |
|--------------------------------|---------:|---------:|---------:|---------:|
| `.collect::<String>()`         | 6.697 ms | 7.168 ms | 6.737 ms | 6.767 ms |
| Textbook implementation        | 4.967 ms | 5.751 ms | 5.108 ms | 5.136 ms |
| Naïve branchless               | 8.295 ms | 8.941 ms |  8.45 ms | 8.464 ms |
| Autovectorized branchless      | 3.352 ms | 3.891 ms | 3.479 ms | 3.503 ms |
| Explicit SIMD with `wide`      | 5.354 ms | 6.101 ms | 5.499 ms | 5.508 ms |

# Copying

© 2025 Eddie Antonio Santos. AGPL-3.0 licensed.
