# UTF-8 encoding

Various algorithms for encoding UTF-8. Some of them are better than others.

```
     Running benches/basic.rs (target/release/deps/basic-d3a63add28fe644d)
Timer precision: 41 ns
basic                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ built_in_implementation                │               │               │               │         │
│  ├─ Test1m                6.708 ms      │ 7.344 ms      │ 6.773 ms      │ 6.795 ms      │ 100     │ 100
│  ╰─ Utf8Sample            23.66 µs      │ 54.66 µs      │ 23.78 µs      │ 24.22 µs      │ 100     │ 100
├─ naive_implementation                   │               │               │               │         │
│  ├─ Test1m                8.486 ms      │ 9.009 ms      │ 8.644 ms      │ 8.656 ms      │ 100     │ 100
│  ╰─ Utf8Sample            27.58 µs      │ 86.33 µs      │ 27.87 µs      │ 31.06 µs      │ 100     │ 100
├─ scalar_branchless                      │               │               │               │         │
│  ├─ Test1m                3.158 ms      │ 3.455 ms      │ 3.257 ms      │ 3.254 ms      │ 100     │ 100
│  ╰─ Utf8Sample            18.91 µs      │ 30.74 µs      │ 18.99 µs      │ 19.15 µs      │ 100     │ 100
├─ simd_branchless                        │               │               │               │         │
│  ├─ Test1m                5.318 ms      │ 5.867 ms      │ 5.412 ms      │ 5.415 ms      │ 100     │ 100
│  ╰─ Utf8Sample            33.45 µs      │ 71.08 µs      │ 33.58 µs      │ 34.34 µs      │ 100     │ 100
╰─ textbook_implementation                │               │               │               │         │
   ├─ Test1m                5.004 ms      │ 5.27 ms       │ 5.077 ms      │ 5.077 ms      │ 100     │ 100
   ╰─ Utf8Sample            24.99 µs      │ 75.79 µs      │ 25.12 µs      │ 25.8 µs       │ 100     │ 100
```


# Copying

© 2025 Eddie Antonio Santos. AGPL-3.0 licensed.
