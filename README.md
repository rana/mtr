# mtr

Metrics for Rust code.

How much does a function really cost?

Measurements in CPU cycles.

`mtr` = metrics

- [mtr](#mtr)
  - [Usage](#usage)
  - [Examples](#examples)
    - [allocation: array vs vector macro](#allocation-array-vs-vector-macro)
    - [sequential lookup: array vs match](#sequential-lookup-array-vs-match)
    - [random lookup: array vs match](#random-lookup-array-vs-match)
  - [Development notes](#development-notes)

## Usage

```sh
clear && cargo run -q --profile release -- --frm alc -d
```
```sh
clear && cargo run -q --profile release -- --frm alc --sel mdn --srt len --grp arr,rsz
```

* Run with optimizations on:
  * `cargo run --profile release`
  * `rustc -C opt-level=3`
  

```sh
> cargo run -q --profile release -- --help
Benchmark, query, and analyze functions

Usage: mtr [OPTIONS] --frm <lbl>...

Options:
  -f, --frm <lbl>...       Run benchmarks from one or more labels
  -g, --grp <lbl>...       Group benchmarks into one or more labels. Each label is a group
  -s, --srt <lbl[struct]>  Sort benchmarks by a struct label
  -x, --sel <lbl[stat]>    Select and apply a statisitcal function
  -t, --trn <lbl[struct]>  Transpose groups to series with the specified struct label
  -c, --cmp                Compare pairs of benchmarks as a ratio of max/min
  -i, --itr <u32>          Set the number of iterations to run a benchmark function [default: 16]
  -d, --dbg                Print debug information
  -h, --help               Print help
```
## Examples

### allocation: array vs vector macro
```sh
clear && cargo run -q --profile release -- --frm alc --sel mdn --srt len --grp arr,mcr --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┬────────┬────────┬────────┬─────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1,024 ┆ 2,048 ┆ 4,096 ┆ 8,192 ┆ 16,384 ┆ 32,768 ┆ 65,536 ┆ 131,072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪═════════╡
│ alc,arr           ┆ 2  ┆ 2  ┆ 4  ┆ 10  ┆ 18  ┆ 34  ┆ 128   ┆ 236   ┆ 432   ┆ 878   ┆ 2,730  ┆ 5,442  ┆ 11,028 ┆ 22,134  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ alc,mcr,vec       ┆ 40 ┆ 74 ┆ 56 ┆ 58  ┆ 104 ┆ 118 ┆ 124   ┆ 190   ┆ 318   ┆ 1,006 ┆ 2,612  ┆ 5,308  ┆ 11,192 ┆ 22,588  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 20 ┆ 37 ┆ 14 ┆ 5.8 ┆ 5.8 ┆ 3.5 ┆ 1     ┆ 1.2   ┆ 1.4   ┆ 1.1   ┆ 1      ┆ 1      ┆ 1      ┆ 1       │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┴────────┴────────┴────────┴─────────┘
```

### sequential lookup: array vs match
```sh
clear && cargo run -q --profile release -- --frm rd,seq --sel mdn --srt len --grp arr,mat --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬───────┬───────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1,024 ┆ 2,048 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪═══════╪═══════╡
│ arr,rd,seq        ┆ 2  ┆ 2  ┆ 2  ┆ 2   ┆ 0   ┆ 2   ┆ 2     ┆ 2     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ mat,rd,seq        ┆ 2  ┆ 16 ┆ 32 ┆ 70  ┆ 134 ┆ 260 ┆ 510   ┆ 1,012 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1  ┆ 8  ┆ 16 ┆ 35  ┆ 134 ┆ 130 ┆ 255   ┆ 506   │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴───────┴───────┘
```

### random lookup: array vs match

This benchmark has random number generation overhead.

```sh
clear && cargo run -q --profile release -- --frm rd,rnd --sel mdn --srt len --grp arr,mat --trn len --cmp
```
```sh
┌───────────────────┬─────┬─────┬───────┬───────┬───────┬────────┬────────┬────────┐
│ len               ┆ 16  ┆ 32  ┆ 64    ┆ 128   ┆ 256   ┆ 512    ┆ 1,024  ┆ 2,048  │
╞═══════════════════╪═════╪═════╪═══════╪═══════╪═══════╪════════╪════════╪════════╡
│ arr,rnd,rd        ┆ 394 ┆ 818 ┆ 1,512 ┆ 2,802 ┆ 5,362 ┆ 10,882 ┆ 21,638 ┆ 43,604 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ mat,rnd,rd        ┆ 338 ┆ 726 ┆ 1,304 ┆ 2,872 ┆ 5,502 ┆ 10,816 ┆ 21,650 ┆ 38,048 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1.2 ┆ 1.1 ┆ 1.2   ┆ 1     ┆ 1     ┆ 1      ┆ 1      ┆ 1.1    │
└───────────────────┴─────┴─────┴───────┴───────┴───────┴────────┴────────┴────────┘
```

## Development notes

[Why my Rust benchmarks were wrong, or how to correctly use std::hint::black_box?](https://gendignoux.com/blog/2022/01/31/rust-benchmarks.html)

[Counting exactly the number of distinct elements: sorted arrays vs. hash sets?](https://lemire.me/blog/2017/05/23/counting-exactly-the-number-of-distinct-elements-sorted-arrays-vs-hash-sets/)
     
[Counting CPU cycles](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/05/23/benchmark.h#L5) Daniel Lemire, C code: `RDTSC_START`, `RDTSC_STOP`

[RDTSC — Read Time-Stamp Counter](https://www.felixcloutier.com/x86/rdtsc)

Emit assembly code from Rust:
* `cargo rustc -- --emit asm`
* Read assembly file `target/debug/deps/mtr-44866ab166973511.s`.

