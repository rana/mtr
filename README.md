# mtr

Metrics for Rust code.

How much does a function really cost?

Measurements in CPU cycles.

`mtr` = metrics

- [mtr](#mtr)
  - [Usage](#usage)
  - [Examples](#examples)
    - [allocation: array vs vector](#allocation-array-vs-vector)
    - [match vs array lookup](#match-vs-array-lookup)
  - [Development notes](#development-notes)

## Usage

```sh
clear && cargo run -q --profile release -- --frm alc -d
```
```sh
clear && cargo run -q --profile release -- --frm alc --sel mdn --srt len --grp arr,rsz
```
```sh
clear && cargo run -q --profile release -- --frm alc --sel mdn --srt len --grp arr,rsz --trn len
```

* Run with optimizations on:
  * `cargo run --profile release`
  * `rustc -C opt-level=3`
  

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
## Examples

### allocation: array vs vector
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┬────────┬────────┬────────┬─────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1,024 ┆ 2,048 ┆ 4,096 ┆ 8,192 ┆ 16,384 ┆ 32,768 ┆ 65,536 ┆ 131,072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪═════════╡
│ alc arr mdn       ┆ 0  ┆ 2  ┆ 6  ┆ 14  ┆ 26  ┆ 48  ┆ 114   ┆ 192   ┆ 344   ┆ 984   ┆ 3,032  ┆ 6,206  ┆ 12,900 ┆ 28,908  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ alc mdn rsz vec   ┆ 52 ┆ 54 ┆ 58 ┆ 68  ┆ 72  ┆ 164 ┆ 266   ┆ 396   ┆ 662   ┆ 1,594 ┆ 3,662  ┆ 7,440  ┆ 15,456 ┆ 34,106  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 52 ┆ 27 ┆ 9  ┆ 4   ┆ 2   ┆ 3   ┆ 2     ┆ 2     ┆ 1     ┆ 1     ┆ 1      ┆ 1      ┆ 1      ┆ 1       │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┴────────┴────────┴────────┴─────────┘
```

### match vs array lookup
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

[Why my Rust benchmarks were wrong, or how to correctly use std::hint::black_box?](https://gendignoux.com/blog/2022/01/31/rust-benchmarks.html)

[Counting exactly the number of distinct elements: sorted arrays vs. hash sets?](https://lemire.me/blog/2017/05/23/counting-exactly-the-number-of-distinct-elements-sorted-arrays-vs-hash-sets/)
     
[Counting CPU cycles](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/05/23/benchmark.h#L5) Daniel Lemire, C code: `RDTSC_START`, `RDTSC_STOP`

[RDTSC — Read Time-Stamp Counter](https://www.felixcloutier.com/x86/rdtsc)

Emit assembly code from Rust:
* `cargo rustc -- --emit asm`
* Read assembly file `target/debug/deps/mtr-44866ab166973511.s`.

