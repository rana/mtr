# mtr

Metrics for Rust code.

How much does a function cost?

Measurements in CPU cycles.

`mtr` = metrics

- [mtr](#mtr)
  - [Usage](#usage)
  - [Examples](#examples)
    - [Allocation: array vs vector macro](#allocation-array-vs-vector-macro)
    - [Allocation: array vs vector capacity and resize](#allocation-array-vs-vector-capacity-and-resize)
    - [Allocation: vector macro vs vector capacity and resize](#allocation-vector-macro-vs-vector-capacity-and-resize)
    - [Lookup: Sequential: array vs match](#lookup-sequential-array-vs-match)
    - [Lookup: Random: array vs match](#lookup-random-array-vs-match)
    - [Iteration: range index (bounds checked) vs iterator](#iteration-range-index-bounds-checked-vs-iterator)
    - [Iteration: range index bounds checked vs range index unchecked](#iteration-range-index-bounds-checked-vs-range-index-unchecked)
    - [Iteration: iterator vs into iterator](#iteration-iterator-vs-into-iterator)
    - [Cast: u8 vs usize](#cast-u8-vs-usize)
    - [Accumulate: read pointer vs read de-referenced value](#accumulate-read-pointer-vs-read-de-referenced-value)
    - [Accumulate: total count vs multiple add one](#accumulate-total-count-vs-multiple-add-one)
    - [Accumulate: Unroll: Single accumulator: no unrolling vs unroll 8](#accumulate-unroll-single-accumulator-no-unrolling-vs-unroll-8)
    - [Accumulate: Unroll: 1 accumulator vs 8 accumulators](#accumulate-unroll-1-accumulator-vs-8-accumulators)
    - [Accumulate: Unroll: 8 accumulators vs 16 accumulators](#accumulate-unroll-8-accumulators-vs-16-accumulators)
  - [Development notes](#development-notes)

## Usage

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

* Run with optimizations on. Either:
  * `cargo run --profile release`
  * `rustc -C opt-level=3`

## Examples

### Allocation: array vs vector macro

Prefer array.

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

### Allocation: array vs vector capacity and resize

Prefer array.

```sh
clear && cargo run -q --profile release -- --frm alc --sel mdn --srt len --grp arr,rsz --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬────────┬────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536  ┆ 131072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪════════╪════════╡
│ alc,arr           ┆ 4  ┆ 4  ┆ 4  ┆ 12  ┆ 20  ┆ 36  ┆ 132  ┆ 242  ┆ 432  ┆ 886  ┆ 2,770 ┆ 5,738 ┆ 11,556 ┆ 23,002 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ alc,rsz,vec       ┆ 24 ┆ 28 ┆ 32 ┆ 32  ┆ 38  ┆ 98  ┆ 124  ┆ 192  ┆ 326  ┆ 668  ┆ 2,770 ┆ 5,886 ┆ 10,698 ┆ 21,476 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 6  ┆ 7  ┆ 8  ┆ 2.7 ┆ 1.9 ┆ 2.7 ┆ 1.1  ┆ 1.3  ┆ 1.3  ┆ 1.3  ┆ 1     ┆ 1     ┆ 1.1    ┆ 1.1    │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴────────┴────────┘
```

### Allocation: vector macro vs vector capacity and resize

Prever vector capacity and resize.

```sh
clear && cargo run -q --profile release -- --frm alc,vec --sel mdn --srt len --grp mcr,rsz --trn len --cmp
```
```sh
┌───────────────────┬────┬─────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┬────────┐
│ len               ┆ 16 ┆ 32  ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192  ┆ 16384 ┆ 32768 ┆ 65536  ┆ 131072 │
╞═══════════════════╪════╪═════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╪════════╡
│ alc,mcr,vec       ┆ 72 ┆ 72  ┆ 90 ┆ 56  ┆ 98  ┆ 120 ┆ 174  ┆ 196  ┆ 510  ┆ 1,072 ┆ 2,850 ┆ 5,126 ┆ 11,082 ┆ 22,140 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ alc,rsz,vec       ┆ 24 ┆ 26  ┆ 30 ┆ 36  ┆ 38  ┆ 88  ┆ 148  ┆ 258  ┆ 484  ┆ 1,020 ┆ 2,798 ┆ 5,092 ┆ 11,062 ┆ 20,412 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 3  ┆ 2.8 ┆ 3  ┆ 1.6 ┆ 2.6 ┆ 1.4 ┆ 1.2  ┆ 1.3  ┆ 1.1  ┆ 1.1   ┆ 1     ┆ 1     ┆ 1      ┆ 1.1    │
└───────────────────┴────┴─────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┴────────┘
```

### Lookup: Sequential: array vs match

Prefer array.

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

### Lookup: Random: array vs match

Prefer match.

```sh
clear && cargo run -q --profile release -- --frm rd,rnd --sel mdn --srt len --grp arr,mat --trn len --cmp
```
```sh
┌───────────────────┬─────┬─────┬─────┬─────┬─────┬─────┬───────┬───────┐
│ len               ┆ 16  ┆ 32  ┆ 64  ┆ 128 ┆ 256 ┆ 512 ┆ 1024  ┆ 2048  │
╞═══════════════════╪═════╪═════╪═════╪═════╪═════╪═════╪═══════╪═══════╡
│ arr,rnd,rd        ┆ 38  ┆ 58  ┆ 86  ┆ 208 ┆ 430 ┆ 836 ┆ 1,766 ┆ 3,344 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ mat,rnd,rd        ┆ 42  ┆ 54  ┆ 130 ┆ 190 ┆ 400 ┆ 724 ┆ 1,418 ┆ 2,406 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1.1 ┆ 1.1 ┆ 1.5 ┆ 1.1 ┆ 1.1 ┆ 1.2 ┆ 1.2   ┆ 1.4   │
└───────────────────┴─────┴─────┴─────┴─────┴─────┴─────┴───────┴───────┘
```

### Iteration: range index (bounds checked) vs iterator

Prefer iterator.

```sh
clear && cargo run -q --profile release -- --frm lop --sel mdn --srt len --grp idx,itr --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ idx,lop           ┆ 2  ┆ 2  ┆ 2  ┆ 2   ┆ 2   ┆ 2   ┆ 6    ┆ 2    ┆ 18   ┆ 10   ┆ 10    ┆ 12    ┆ 10    ┆ 24     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ itr,lop           ┆ 2  ┆ 2  ┆ 2  ┆ 2   ┆ 2   ┆ 2   ┆ 2    ┆ 2    ┆ 2    ┆ 10   ┆ 2     ┆ 12    ┆ 10    ┆ 22     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1  ┆ 1  ┆ 1  ┆ 1   ┆ 1   ┆ 1   ┆ 3    ┆ 1    ┆ 9    ┆ 1    ┆ 5     ┆ 1     ┆ 1     ┆ 1.1    │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

### Iteration: range index bounds checked vs range index unchecked

Tie.

```sh
clear && cargo run -q --profile release -- --frm lop,idx --sel mdn --srt len --grp chk,unchk --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ chk,idx,lop       ┆ 0  ┆ 0  ┆ 0  ┆ 0   ┆ 0   ┆ 0   ┆ 0    ┆ 0    ┆ 0    ┆ 0    ┆ 0     ┆ 24    ┆ 14    ┆ 14     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ idx,lop,unchk     ┆ 0  ┆ 0  ┆ 0  ┆ 0   ┆ 0   ┆ 0   ┆ 4    ┆ 2    ┆ 2    ┆ 0    ┆ 0     ┆ 20    ┆ 14    ┆ 22     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 0  ┆ 0  ┆ 0  ┆ 0   ┆ 0   ┆ 0   ┆ 4    ┆ 2    ┆ 2    ┆ 0    ┆ 0     ┆ 1.2   ┆ 1     ┆ 1.6    │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

### Iteration: iterator vs into iterator

Prefer iterator.

```sh
clear && cargo run -q --profile release -- --frm lop --sel mdn --srt len --grp itr,into_itr --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬──────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512  ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪══════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ itr,lop           ┆ 4  ┆ 4  ┆ 4  ┆ 4   ┆ 4   ┆ 4    ┆ 6    ┆ 4    ┆ 4    ┆ 4    ┆ 4     ┆ 4     ┆ 20    ┆ 18     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ into_itr,lop      ┆ 16 ┆ 16 ┆ 16 ┆ 16  ┆ 18  ┆ 54   ┆ 72   ┆ 52   ┆ 48   ┆ 62   ┆ 96    ┆ 100   ┆ 130   ┆ 254    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 4  ┆ 4  ┆ 4  ┆ 4   ┆ 4.5 ┆ 13.5 ┆ 12   ┆ 13   ┆ 12   ┆ 15.5 ┆ 24    ┆ 25    ┆ 6.5   ┆ 14.1   │
└───────────────────┴────┴────┴────┴─────┴─────┴──────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

### Cast: u8 vs usize

Prefer usize.

```sh
clear && cargo run -q --profile release -- --frm cst --sel mdn --srt len --grp u8,usize --trn len --cmp
```
```sh
┌───────────────────┬─────┬────┬─────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┬────────┬────────┬────────┬─────────┐
│ len               ┆ 16  ┆ 32 ┆ 64  ┆ 128 ┆ 256 ┆ 512 ┆ 1024  ┆ 2048  ┆ 4096  ┆ 8192  ┆ 16384  ┆ 32768  ┆ 65536  ┆ 131072  │
╞═══════════════════╪═════╪════╪═════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪═════════╡
│ cst,u8            ┆ 18  ┆ 8  ┆ 78  ┆ 198 ┆ 290 ┆ 578 ┆ 1,272 ┆ 2,336 ┆ 4,636 ┆ 9,294 ┆ 18,576 ┆ 37,070 ┆ 74,938 ┆ 149,532 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ cst,usize         ┆ 14  ┆ 2  ┆ 68  ┆ 144 ┆ 290 ┆ 580 ┆ 1,168 ┆ 2,320 ┆ 4,622 ┆ 9,284 ┆ 18,552 ┆ 44,356 ┆ 74,406 ┆ 152,376 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1.3 ┆ 4  ┆ 1.1 ┆ 1.4 ┆ 1   ┆ 1   ┆ 1.1   ┆ 1     ┆ 1     ┆ 1     ┆ 1      ┆ 1.2    ┆ 1      ┆ 1       │
└───────────────────┴─────┴────┴─────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┴────────┴────────┴────────┴─────────┘
```

### Accumulate: read pointer vs read de-referenced value

Tie. Slightly prefer read de-referenced value.

```sh
clear && cargo run -q --profile release -- --frm acm,rd --sel mdn --srt len --grp ptr,val --trn len --cmp
```
```sh
┌───────────────────┬─────┬────┬────┬─────┬─────┬─────┬───────┬───────┬───────┬───────┬────────┬────────┬────────┬─────────┐
│ len               ┆ 16  ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024  ┆ 2048  ┆ 4096  ┆ 8192  ┆ 16384  ┆ 32768  ┆ 65536  ┆ 131072  │
╞═══════════════════╪═════╪════╪════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪═════════╡
│ acm,ptr,rd        ┆ 16  ┆ 0  ┆ 70 ┆ 156 ┆ 310 ┆ 582 ┆ 1,236 ┆ 2,366 ┆ 4,730 ┆ 9,470 ┆ 18,930 ┆ 37,086 ┆ 74,532 ┆ 152,650 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,rd,val        ┆ 14  ┆ 0  ┆ 68 ┆ 150 ┆ 302 ┆ 592 ┆ 1,558 ┆ 2,374 ┆ 4,716 ┆ 9,492 ┆ 18,898 ┆ 37,888 ┆ 76,000 ┆ 162,704 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1.1 ┆ 0  ┆ 1  ┆ 1   ┆ 1   ┆ 1   ┆ 1.3   ┆ 1     ┆ 1     ┆ 1     ┆ 1      ┆ 1      ┆ 1      ┆ 1.1     │
└───────────────────┴─────┴────┴────┴─────┴─────┴─────┴───────┴───────┴───────┴───────┴────────┴────────┴────────┴─────────┘
```

### Accumulate: total count vs multiple add one

Prefer total count.

```sh
clear && cargo run -q --profile release -- --frm acm,add --sel mdn --srt len --grp one,cnt --trn len --cmp
```
```sh
┌───────────────────┬────┬────┬─────┬─────┬─────┬─────┬───────┬───────┬───────┬────────┬────────┬────────┬────────┬─────────┐
│ len               ┆ 16 ┆ 32 ┆ 64  ┆ 128 ┆ 256 ┆ 512 ┆ 1024  ┆ 2048  ┆ 4096  ┆ 8192   ┆ 16384  ┆ 32768  ┆ 65536  ┆ 131072  │
╞═══════════════════╪════╪════╪═════╪═════╪═════╪═════╪═══════╪═══════╪═══════╪════════╪════════╪════════╪════════╪═════════╡
│ acm,add,one       ┆ 16 ┆ 48 ┆ 78  ┆ 162 ┆ 312 ┆ 638 ┆ 1,244 ┆ 2,632 ┆ 4,930 ┆ 10,300 ┆ 20,598 ┆ 39,580 ┆ 92,742 ┆ 159,482 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,add,cnt       ┆ 16 ┆ 2  ┆ 70  ┆ 200 ┆ 298 ┆ 604 ┆ 1,198 ┆ 2,358 ┆ 4,736 ┆ 9,460  ┆ 18,926 ┆ 37,904 ┆ 75,944 ┆ 152,638 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1  ┆ 24 ┆ 1.1 ┆ 1.2 ┆ 1   ┆ 1.1 ┆ 1     ┆ 1.1   ┆ 1     ┆ 1.1    ┆ 1.1    ┆ 1      ┆ 1.2    ┆ 1       │
└───────────────────┴────┴────┴─────┴─────┴─────┴─────┴───────┴───────┴───────┴────────┴────────┴────────┴────────┴─────────┘
```

### Accumulate: Unroll: Single accumulator: no unrolling vs unroll 8

Prefer no unrolling.

```sh
clear && cargo run -q --profile release -- --frm lop,acm --sel mdn --srt len --grp unr[0],unr[8]-var[1] --trn len --cmp
```
```sh
┌───────────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┬────────┐
│ len                   ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192  ┆ 16384 ┆ 32768 ┆ 65536  ┆ 131072 │
╞═══════════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╪════════╡
│ acm,lop,unr(0)        ┆ 4  ┆ 8  ┆ 24 ┆ 10  ┆ 18  ┆ 42  ┆ 114  ┆ 162  ┆ 344  ┆ 708   ┆ 2,006 ┆ 4,308 ┆ 11,334 ┆ 22,052 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(8),var(1) ┆ 4  ┆ 8  ┆ 24 ┆ 20  ┆ 44  ┆ 104 ┆ 204  ┆ 360  ┆ 818  ┆ 1,590 ┆ 3,204 ┆ 6,856 ┆ 14,848 ┆ 32,104 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)     ┆ 1  ┆ 1  ┆ 1  ┆ 2   ┆ 2.4 ┆ 2.5 ┆ 1.8  ┆ 2.2  ┆ 2.4  ┆ 2.2   ┆ 1.6   ┆ 1.6   ┆ 1.3    ┆ 1.5    │
└───────────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┴────────┘
```

### Accumulate: Unroll: 1 accumulator vs 8 accumulators

Prefer 8 accumulators.

```sh
clear && cargo run -q --profile release -- --frm lop,acm,unr[8] --sel mdn --srt len --grp var[1],var[8] --trn len --cmp
```
```sh
┌───────────────────────┬─────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┬────────┐
│ len                   ┆ 16  ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192  ┆ 16384 ┆ 32768 ┆ 65536  ┆ 131072 │
╞═══════════════════════╪═════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╪════════╡
│ acm,lop,unr(8),var(1) ┆ 6   ┆ 10 ┆ 26 ┆ 22  ┆ 44  ┆ 104 ┆ 202  ┆ 362  ┆ 802  ┆ 1,588 ┆ 3,094 ┆ 6,290 ┆ 14,056 ┆ 29,776 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(8),var(8) ┆ 8   ┆ 10 ┆ 26 ┆ 10  ┆ 18  ┆ 46  ┆ 90   ┆ 162  ┆ 300  ┆ 628   ┆ 1,588 ┆ 3,448 ┆ 8,450  ┆ 19,596 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)     ┆ 1.3 ┆ 1  ┆ 1  ┆ 2.2 ┆ 2.4 ┆ 2.3 ┆ 2.2  ┆ 2.2  ┆ 2.7  ┆ 2.5   ┆ 1.9   ┆ 1.8   ┆ 1.7    ┆ 1.5    │
└───────────────────────┴─────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┴────────┘
```

### Accumulate: Unroll: 8 accumulators vs 16 accumulators

Tie up to 1024 lengths.

Prefer 16 accumulators over 1024 lengths.

```sh
clear && cargo run -q --profile release -- --frm lop,acm --sel mdn --srt len --grp unr[8]-var[8],unr[16]-var[16] --trn len --cmp
```
```sh
┌─────────────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len                     ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═════════════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ acm,lop,unr(8),var(8)   ┆ 4  ┆ 10 ┆ 26 ┆ 10  ┆ 18  ┆ 48  ┆ 96   ┆ 170  ┆ 318  ┆ 630  ┆ 1,588 ┆ 3,678 ┆ 8,404 ┆ 20,150 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(16),var(16) ┆ 4  ┆ 10 ┆ 26 ┆ 34  ┆ 18  ┆ 42  ┆ 98   ┆ 184  ┆ 288  ┆ 628  ┆ 1,518 ┆ 3,146 ┆ 7,194 ┆ 19,812 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)       ┆ 1  ┆ 1  ┆ 1  ┆ 3.4 ┆ 1   ┆ 1.1 ┆ 1    ┆ 1.1  ┆ 1.1  ┆ 1    ┆ 1     ┆ 1.2   ┆ 1.2   ┆ 1      │
└─────────────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```



## Development notes

[Why my Rust benchmarks were wrong, or how to correctly use std::hint::black_box?](https://gendignoux.com/blog/2022/01/31/rust-benchmarks.html)

[Counting exactly the number of distinct elements: sorted arrays vs. hash sets?](https://lemire.me/blog/2017/05/23/counting-exactly-the-number-of-distinct-elements-sorted-arrays-vs-hash-sets/)
     
[Counting CPU cycles](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/05/23/benchmark.h#L5) Daniel Lemire, C code: `RDTSC_START`, `RDTSC_STOP`

[RDTSC — Read Time-Stamp Counter](https://www.felixcloutier.com/x86/rdtsc)

Emit assembly code from Rust:
* `cargo rustc -- --emit asm`
* Read assembly file `target/debug/deps/mtr-44866ab166973511.s`.

