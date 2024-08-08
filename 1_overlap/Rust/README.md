Run test with

```
cargo test
```

exaple

```
running 10 tests
test test::merged_none ... ok
test test::merged_exact ... ok
test test::merged_all_a ... ok
test test::merged_6 ... ok
test test::merged_all_b ... ok
test test::merged_on_threshold ... ok
test test::overlap_6 ... ok
test test::overlap_all ... ok
test test::repeating_pattern ... ok
test test::long_array_iter ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

note: the `long_array_iter` uses arrays of _200_000_ elements
