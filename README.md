# UTF-8 encoding

Various algorithms for encoding UTF-8. Some of them are better than others.

```
     Running benches/basic.rs (target/release/deps/basic-d3a63add28fe644d)
Timer precision: 41 ns
basic                              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ built_in_implementation                       │               │               │               │         │
│  ├─ Test1m                       6.628 ms      │ 6.925 ms      │ 6.757 ms      │ 6.768 ms      │ 100     │ 100
│  ╰─ Utf8Sample                   21.16 µs      │ 47.49 µs      │ 21.22 µs      │ 21.56 µs      │ 100     │ 100
├─ naive_implementation                          │               │               │               │         │
│  ├─ Test1m                       8.271 ms      │ 10.36 ms      │ 8.407 ms      │ 8.441 ms      │ 100     │ 100
│  ╰─ Utf8Sample                   26.08 µs      │ 86.24 µs      │ 26.24 µs      │ 26.99 µs      │ 100     │ 100
├─ scalar_branchless                             │               │               │               │         │
│  ├─ Test1m                       3.157 ms      │ 4.052 ms      │ 3.222 ms      │ 3.246 ms      │ 100     │ 100
│  ╰─ Utf8Sample                   18.91 µs      │ 30.24 µs      │ 19.08 µs      │ 19.2 µs       │ 100     │ 100
├─ scalar_branchless_no_shift_lut                │               │               │               │         │
│  ├─ Test1m                       2.419 ms      │ 4.96 ms       │ 2.49 ms       │ 2.529 ms      │ 100     │ 100
│  ╰─ Utf8Sample                   13.91 µs      │ 48.08 µs      │ 14.04 µs      │ 14.54 µs      │ 100     │ 100
├─ simd_branchless                               │               │               │               │         │
│  ├─ Test1m                       5.34 ms       │ 7.37 ms       │ 5.409 ms      │ 5.436 ms      │ 100     │ 100
│  ╰─ Utf8Sample                   33.45 µs      │ 70.49 µs      │ 33.54 µs      │ 34.02 µs      │ 100     │ 100
╰─ textbook_implementation                       │               │               │               │         │
   ├─ Test1m                       4.95 ms       │ 5.212 ms      │ 5.006 ms      │ 5.013 ms      │ 100     │ 100
   ╰─ Utf8Sample                   20.12 µs      │ 72.04 µs      │ 20.24 µs      │ 21.23 µs      │ 100     │ 100
```


# Copying

© 2025 Eddie Antonio Santos. AGPL-3.0 licensed.
