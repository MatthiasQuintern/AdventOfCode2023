# [Day 14](https://adventofcode.com/2023/day/14) 
:gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift:

Today's language: **Rust**

Lines of code: **84**

Execution time: **0,171 s**

This solution uses a single function for the tilting procedure.
Different tilting directions are achieved by "remapping" `x` and `y` through captures.

For task 2, a cycle is searched by comparing the rocks after each nwse-tilt with all previous results. 
After finding two equal states, only `(1.000.000.000 - cycle_start) % cycle_length` cycles have to be performed to get the correct result.
This approach requires making copies of the 2D array in a loop, which involves expensive heap allocations.
It is still pretty fast.

```shell
rustc day14.rs
./day14
```
<!-- no bruteforce in < 90 lines -->
