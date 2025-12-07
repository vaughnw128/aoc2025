# aoc2025

My advent of code 2025 solutions :steamhappy:

Please see the [advent of code website](https://adventofcode.com/2025) if you're interested in fun puzzles.

## running

Navigate to the day you want to run and just run it! Simple as :D

```bash
$ cd d1 && cargo run

➜  d1 git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/main`
Solution 1: 1076
Solution 2: 6379
```

## benchmarks

Solutions are benchmarked with divan 

```bash
$ cd d1 && cargo bench

...

     Running benches/bench.rs (target/release/deps/bench-3ecf50ff07f5f598)
Timer precision: 10 ns
bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part_1  107.2 µs      │ 596 µs        │ 109.1 µs      │ 124.3 µs      │ 100     │ 100
╰─ part_2  106.2 µs      │ 288.8 µs      │ 107.4 µs      │ 114 µs        │ 100     │ 100
```