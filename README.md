# mtr

Rust metrics for common functions.

`mtr` = metrics

Measurements in CPU cycles.

- [mtr](#mtr)
  - [Usage](#usage)
  - [array vs vector](#array-vs-vector)
  - [match vs array](#match-vs-array)
  - [Development notes](#development-notes)

## Usage

* Run benchmarks with optimizations.
```sh
clear && cargo run -q --profile release -- mat-arr
```

```sh
> cargo run -q --profile release -- --help
Usage: mtr [COMMAND]

Commands:
  arr-vec  Array-vector comparison
  mat-arr  Match-Array comparison
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## array vs vector
```sh
┌────────────────────────┬────┬────┬─────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┬────────┬────────┬────────┬─────────┐
│ len                    ┆ 16 ┆ 32 ┆ 64  ┆ 128 ┆ 256 ┆ 512 ┆ 1,024 ┆ 2,048 ┆ 4,096 ┆ 8,192 ┆ 16,384 ┆ 32,768 ┆ 65,536 ┆ 131,072 │
╞════════════════════════╪════╪════╪═════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪═════════╡
│ arr alc prm(0) mdn     ┆ 0  ┆ 0  ┆ 2   ┆ 18  ┆ 14  ┆ 40  ┆ 88    ┆ 160   ┆ 298   ┆ 580   ┆ 2,558  ┆ 5,698  ┆ 11,394 ┆ 22,844  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ vec alc prm(0) mdn rsz ┆ 28 ┆ 28 ┆ 32  ┆ 36  ┆ 40  ┆ 84  ┆ 180   ┆ 270   ┆ 524   ┆ 1,454 ┆ 2,870  ┆ 5,668  ┆ 11,432 ┆ 22,868  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ vec alc prm(0) mdn mcr ┆ 60 ┆ 96 ┆ 104 ┆ 64  ┆ 76  ┆ 120 ┆ 178   ┆ 296   ┆ 564   ┆ 1,490 ┆ 2,938  ┆ 5,698  ┆ 10,576 ┆ 22,854  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ times                  ┆ 28 ┆ 28 ┆ 16  ┆ 2   ┆ 2   ┆ 2   ┆ 2     ┆ 1     ┆ 1     ┆ 2     ┆ 1      ┆ 1      ┆ 1      ┆ 1       │
└────────────────────────┴────┴────┴─────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┴────────┴────────┴────────┴─────────┘
```

## match vs array
```sh
┌─────────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┐
│ len     ┆ 4   ┆ 8   ┆ 16  ┆ 32  ┆ 64  ┆ 128 ┆ 256 ┆ 512 ┆ 1,024 ┆ 2,048 ┆ 4,096 ┆ 8,192 │
╞═════════╪═════╪═════╪═════╪═════╪═════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╡
│ mdn mat ┆ 486 ┆ 412 ┆ 486 ┆ 412 ┆ 360 ┆ 420 ┆ 492 ┆ 566 ┆ 566   ┆ 420   ┆ 494   ┆ 566   │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ arr mdn ┆ 356 ┆ 360 ┆ 356 ┆ 358 ┆ 422 ┆ 500 ┆ 386 ┆ 432 ┆ 516   ┆ 658   ┆ 1,428 ┆ 2,716 │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ times   ┆ 1   ┆ 1   ┆ 1   ┆ 1   ┆ 1   ┆ 1   ┆ 1   ┆ 1   ┆ 1     ┆ 1     ┆ 2     ┆ 4     │
└─────────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┘
```

## Development notes

Cargo profiles for compiler optimization.

<https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level>
<https://doc.rust-lang.org/cargo/reference/profiles.html#dev>

`test` & `dev` profiles don't optimize code, `opt-level=0`.

To emit assembly code from Rust:
* `cargo rustc -- --emit asm`
* Read assembly file `target/debug/deps/mtr-44866ab166973511.s`.

To make assembly code more readable:
* Mark crate type as `#![crate_type = "staticlib"]`.
* Marks fn with attribute `#[no_mangle]`.
```rust
#![crate_type = "staticlib"]
#[no_mangle]
pub fn something(){}
```
```sh
cargo rustc --crate-type=staticlib
```

[Why my Rust benchmarks were wrong, or how to correctly use std::hint::black_box?](https://gendignoux.com/blog/2022/01/31/rust-benchmarks.html)

[Counting exactly the number of distinct elements: sorted arrays vs. hash sets?](https://lemire.me/blog/2017/05/23/counting-exactly-the-number-of-distinct-elements-sorted-arrays-vs-hash-sets/)
     
[Counting CPU cycles](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/05/23/benchmark.h#L5) Daniel Lemire, C code: `RDTSC_START`, `RDTSC_STOP`

[RDTSC — Read Time-Stamp Counter](https://www.felixcloutier.com/x86/rdtsc)
