# AoC_2022
Solutions to the Advent of Code 2022, written in Rust.

If you want to run these, you first need to [install Rust](https://www.rust-lang.org/tools/install). Then, from the root of the project, just run:
```
cargo run
```
To select a different day, or to run multiple at once, edit [`src/main.rs`](https://github.com/lilakerl/AoC_2022/blob/trunk/src/main.rs), 
and swap out the day in the imports and body of the `main()` function. E.g. the below main.rs runs days 2 and 5:
```rust
use crate::day::Day;
// bring the requisite days into scope
use crate::day2::Day2;
use crate::day5::Day5;

fn main() {
    // run the days' solutions and print the answers
    println!("Day 2:\n{}", Day2.run());
    println!("Day 5:\n{}", Day5.run());
}
```
Note that each day runs on the input in `src/day<day-number>/input.txt`. By default, these are populated with the test input for each respective day.
