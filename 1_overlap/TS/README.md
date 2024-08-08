Run test with

```
bun test
```

example

```
overlap.test.ts:
✓ overlap_6 [0.23ms]
✓ merged_6 [0.14ms]
✓ merged_on_threshold [0.05ms]
✓ merged_none [0.04ms]
✓ overlap_all [0.03ms]
✓ merged_exact [0.02ms]
✓ merged_all_a [0.03ms]
✓ merged_all_b [0.02ms]
✓ repeating_pattern [0.03ms]
✓ long_array_iter [662.92ms]

 10 pass
 0 fail
 10 expect() calls
Ran 10 tests across 1 files. [678.00ms]
```

note: the `long_array_iter` uses arrays of _10_000_ elements
