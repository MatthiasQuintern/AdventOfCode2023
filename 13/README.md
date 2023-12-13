# [Day 13](https://adventofcode.com/2023/day/13) 
:gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift::gift:

Today's language: **Rust**

Lines of code: **84**

Execution time: **0,002 s**

Once again, binary encoding is used to transform every line and every column of each pattern into one unsigned int.
The columns and lines are stored in one vector each. This means the same function can be used to find the reflection axis for both.

For task 2, we don't check whether two lines are equal, but whether they differ by 0 or 1 bit.

This approach is super fast since it has 0 heap allocations in loops and comparing two ints is way faster than comparing two strings.

```shell
rustc day13.rs
./day13
```

<!-- binary encoding ftw -->
