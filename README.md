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
    - [Iteration: Vector: iterator vs into iterator](#iteration-vector-iterator-vs-into-iterator)
    - [Iteration: Slice: iterator vs into iterator](#iteration-slice-iterator-vs-into-iterator)
    - [Cast: u8 vs usize](#cast-u8-vs-usize)
    - [Accumulate: read pointer vs read de-referenced value](#accumulate-read-pointer-vs-read-de-referenced-value)
    - [Accumulate: total count vs multiple add one](#accumulate-total-count-vs-multiple-add-one)
    - [Accumulate: Unroll: Single accumulator: no unrolling vs unroll 8](#accumulate-unroll-single-accumulator-no-unrolling-vs-unroll-8)
    - [Accumulate: Unroll: 1 accumulator vs 8 accumulators](#accumulate-unroll-1-accumulator-vs-8-accumulators)
    - [Accumulate: Unroll: 8 accumulators vs 16 accumulators](#accumulate-unroll-8-accumulators-vs-16-accumulators)
    - [Accumulate: Unroll: no unrolling vs unroll 8 with 8 accumulators](#accumulate-unroll-no-unrolling-vs-unroll-8-with-8-accumulators)
    - [Accumulate: Unroll: no unrolling vs unroll 16 with 16 accumulators](#accumulate-unroll-no-unrolling-vs-unroll-16-with-16-accumulators)
    - [Accumulate: Parallel: single thread, single accumulator vs 2 threads, 2 accumulators](#accumulate-parallel-single-thread-single-accumulator-vs-2-threads-2-accumulators)
    - [Accumuate: single thread, unroll 2, 2 accumulator vs two thread, two accumulator, mspc](#accumuate-single-thread-unroll-2-2-accumulator-vs-two-thread-two-accumulator-mspc)
    - [Accumulate: Parallel: 2 threads, 2 accumulators, join vs 2 threads, 2 accumulators, mpsc](#accumulate-parallel-2-threads-2-accumulators-join-vs-2-threads-2-accumulators-mpsc)
    - [Accumulate: Parallel: 2 threads, 2 accumulators, mspc vs 4 threads, 4 accumulators, mpsc](#accumulate-parallel-2-threads-2-accumulators-mspc-vs-4-threads-4-accumulators-mpsc)
    - [Accumulate: Parallel: 1 thread, 1 accumulator vs 4 threads, 4 accumulators, mpsc](#accumulate-parallel-1-thread-1-accumulator-vs-4-threads-4-accumulators-mpsc)
    - [Accumulate: Parallel: 4 threads, 4 accumulators, mspc vs 8 threads, 8 accumulators, mpsc](#accumulate-parallel-4-threads-4-accumulators-mspc-vs-8-threads-8-accumulators-mpsc)
    - [Accumulate: Parallel: 8 threads, 8 accumulators, mspc vs 16 threads, 16 accumulators, mpsc](#accumulate-parallel-8-threads-8-accumulators-mspc-vs-16-threads-16-accumulators-mpsc)
  - [Development notes](#development-notes)

## Usage

```sh
clear && cargo run -q --profile release
```

* Run with optimizations on. Either:
  * `cargo run --profile release`
  * `rustc -C opt-level=3`

## Examples

### Allocation: array vs vector macro

Prefer array.

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

### Iteration: Vector: iterator vs into iterator

Prefer iterator.

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

### Iteration: Slice: iterator vs into iterator

Tie.

```sh
┌───────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len               ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═══════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ itr,lop,slc       ┆ 2  ┆ 4  ┆ 2  ┆ 2   ┆ 2   ┆ 2   ┆ 2    ┆ 2    ┆ 2    ┆ 2    ┆ 2     ┆ 2     ┆ 14    ┆ 20     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ into_itr,lop,slc  ┆ 2  ┆ 2  ┆ 4  ┆ 2   ┆ 2   ┆ 2   ┆ 2    ┆ 2    ┆ 2    ┆ 2    ┆ 2     ┆ 2     ┆ 14    ┆ 22     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 1  ┆ 2  ┆ 2  ┆ 1   ┆ 1   ┆ 1   ┆ 1    ┆ 1    ┆ 1    ┆ 1    ┆ 1     ┆ 1     ┆ 1     ┆ 1.1    │
└───────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

Note:
```
warning: this `.into_iter()` call is equivalent to `.iter()` and will not consume the `slice`
     --> src/bens.rs:18207:44
      |
18207 |                 for val in vals.as_slice().into_iter() {
      |                                            ^^^^^^^^^ help: call directly: `iter`
      |
      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#into_iter_on_ref
      = note: `#[warn(clippy::into_iter_on_ref)]` on by default
```

### Cast: u8 vs usize

Prefer usize.

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

Tie up to 4096 lengths.

Prefer 16 accumulators over 4096 lengths.

```sh
┌─────────────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len                     ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═════════════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ acm,lop,unr(8),var(8)   ┆ 6  ┆ 6  ┆ 22 ┆ 12  ┆ 30  ┆ 54  ┆ 90   ┆ 152  ┆ 280  ┆ 622  ┆ 1,918 ┆ 3,576 ┆ 8,648 ┆ 20,598 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(16),var(16) ┆ 2  ┆ 6  ┆ 22 ┆ 30  ┆ 22  ┆ 56  ┆ 120  ┆ 168  ┆ 280  ┆ 584  ┆ 1,386 ┆ 3,126 ┆ 7,734 ┆ 19,144 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)       ┆ 3  ┆ 1  ┆ 1  ┆ 2.5 ┆ 1.4 ┆ 1   ┆ 1.3  ┆ 1.1  ┆ 1    ┆ 1.1  ┆ 1.4   ┆ 1.1   ┆ 1.1   ┆ 1.1    │
└─────────────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

### Accumulate: Unroll: no unrolling vs unroll 8 with 8 accumulators

Prefer no unrolling up to 1024 lengths.

Prefer unrolling with 16 accumulators over 1024 lengths.

```sh
┌───────────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬────────┬────────┐
│ len                   ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536  ┆ 131072 │
╞═══════════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪════════╪════════╡
│ acm,lop,unr(0)        ┆ 4  ┆ 8  ┆ 24 ┆ 10  ┆ 26  ┆ 42  ┆ 96   ┆ 178  ┆ 442  ┆ 634  ┆ 1,974 ┆ 4,366 ┆ 10,238 ┆ 22,388 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(8),var(8) ┆ 4  ┆ 8  ┆ 24 ┆ 10  ┆ 26  ┆ 66  ┆ 86   ┆ 176  ┆ 288  ┆ 622  ┆ 1,594 ┆ 3,574 ┆ 9,072  ┆ 20,140 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)     ┆ 1  ┆ 1  ┆ 1  ┆ 1   ┆ 1   ┆ 1.6 ┆ 1.1  ┆ 1    ┆ 1.5  ┆ 1    ┆ 1.2   ┆ 1.2   ┆ 1.1    ┆ 1.1    │
└───────────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴────────┴────────┘
```

### Accumulate: Unroll: no unrolling vs unroll 16 with 16 accumulators

Prefer no unrolling up to 4096 lengths.

Prefer unrolling with 16 accumulators over 4096 lengths.

```sh
┌─────────────────────────┬────┬────┬────┬─────┬─────┬─────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
│ len                     ┆ 16 ┆ 32 ┆ 64 ┆ 128 ┆ 256 ┆ 512 ┆ 1024 ┆ 2048 ┆ 4096 ┆ 8192 ┆ 16384 ┆ 32768 ┆ 65536 ┆ 131072 │
╞═════════════════════════╪════╪════╪════╪═════╪═════╪═════╪══════╪══════╪══════╪══════╪═══════╪═══════╪═══════╪════════╡
│ acm,lop,unr(0)          ┆ 6  ┆ 6  ┆ 22 ┆ 8   ┆ 26  ┆ 40  ┆ 90   ┆ 164  ┆ 342  ┆ 660  ┆ 1,960 ┆ 4,324 ┆ 9,646 ┆ 22,788 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ acm,lop,unr(16),var(16) ┆ 2  ┆ 6  ┆ 22 ┆ 30  ┆ 26  ┆ 78  ┆ 132  ┆ 184  ┆ 292  ┆ 588  ┆ 1,586 ┆ 2,954 ┆ 7,222 ┆ 19,378 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ ratio (max / min)       ┆ 3  ┆ 1  ┆ 1  ┆ 3.8 ┆ 1   ┆ 2   ┆ 1.5  ┆ 1.1  ┆ 1.2  ┆ 1.1  ┆ 1.2   ┆ 1.5   ┆ 1.3   ┆ 1.2    │
└─────────────────────────┴────┴────┴────┴─────┴─────┴─────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴────────┘
```

### Accumulate: Parallel: single thread, single accumulator vs 2 threads, 2 accumulators

Prefer single thread, single accumulator.

```sh
┌───────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ len               ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192    ┆ 16384   ┆ 32768   ┆ 65536   ┆ 131072  │
╞═══════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪═════════╪═════════╪═════════╪═════════╡
│ acm,lop,unr(0)    ┆ 6      ┆ 12     ┆ 26     ┆ 12     ┆ 20     ┆ 44     ┆ 88     ┆ 168    ┆ 420    ┆ 664     ┆ 1,790   ┆ 4,518   ┆ 10,108  ┆ 22,222  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,pll(2),var(2) ┆ 34,702 ┆ 36,512 ┆ 36,224 ┆ 37,452 ┆ 41,914 ┆ 39,452 ┆ 42,320 ┆ 44,202 ┆ 45,132 ┆ 159,180 ┆ 168,724 ┆ 213,796 ┆ 239,166 ┆ 395,640 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min) ┆ 5,783  ┆ 3,042  ┆ 1,393  ┆ 3,121  ┆ 2,095  ┆ 896    ┆ 480    ┆ 263    ┆ 107    ┆ 239     ┆ 94      ┆ 47      ┆ 23      ┆ 17      │
└───────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴─────────┴─────────┴─────────┘
```

### Accumuate: single thread, unroll 2, 2 accumulator vs two thread, two accumulator, mspc

Prefer single thread, unroll 2, 2 accumulator.

```sh
┌────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┬─────────┬─────────┬─────────┐
│ len                    ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192    ┆ 16384  ┆ 32768   ┆ 65536   ┆ 131072  │
╞════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪════════╪═════════╪═════════╪═════════╡
│ acm,lop,unr(2),var(2)  ┆ 6      ┆ 10     ┆ 12     ┆ 12     ┆ 22     ┆ 74     ┆ 140    ┆ 268    ┆ 542    ┆ 1,062   ┆ 2,174  ┆ 4,576   ┆ 14,596  ┆ 23,670  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(2),var(2) ┆ 23,058 ┆ 26,994 ┆ 19,186 ┆ 16,060 ┆ 14,092 ┆ 13,152 ┆ 13,194 ┆ 13,116 ┆ 11,250 ┆ 107,456 ┆ 44,952 ┆ 130,378 ┆ 202,186 ┆ 196,820 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)      ┆ 3,843  ┆ 2,699  ┆ 1,598  ┆ 1,338  ┆ 640    ┆ 177    ┆ 94     ┆ 48     ┆ 20     ┆ 101     ┆ 20     ┆ 28      ┆ 13      ┆ 8.3     │
└────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┴─────────┴─────────┴─────────┘
```

### Accumulate: Parallel: 2 threads, 2 accumulators, join vs 2 threads, 2 accumulators, mpsc

Prefer mspc during multi-threading.

```sh
┌────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ len                    ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192    ┆ 16384   ┆ 32768   ┆ 65536   ┆ 131072  │
╞════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪═════════╪═════════╪═════════╪═════════╡
│ acm,join,pll(2),var(2) ┆ 76,932 ┆ 78,054 ┆ 78,868 ┆ 78,562 ┆ 78,140 ┆ 76,442 ┆ 78,434 ┆ 79,352 ┆ 89,660 ┆ 139,852 ┆ 221,310 ┆ 213,838 ┆ 288,378 ┆ 293,264 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(2),var(2) ┆ 16,432 ┆ 16,742 ┆ 16,768 ┆ 23,030 ┆ 18,740 ┆ 16,498 ┆ 17,462 ┆ 14,876 ┆ 22,480 ┆ 31,332  ┆ 82,010  ┆ 106,762 ┆ 169,888 ┆ 253,878 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)      ┆ 4.7    ┆ 4.7    ┆ 4.7    ┆ 3.4    ┆ 4.2    ┆ 4.6    ┆ 4.5    ┆ 5.3    ┆ 4      ┆ 4.5     ┆ 2.7     ┆ 2       ┆ 1.7     ┆ 1.2     │
└────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴─────────┴─────────┴─────────┘
```

### Accumulate: Parallel: 2 threads, 2 accumulators, mspc vs 4 threads, 4 accumulators, mpsc

Either. Based on array length.

```sh
┌────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬─────────┐
│ len                    ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192   ┆ 16384  ┆ 32768   ┆ 65536   ┆ 131072  │
╞════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪═════════╪═════════╡
│ acm,mpsc,pll(2),var(2) ┆ 22,506 ┆ 26,326 ┆ 36,808 ┆ 14,416 ┆ 13,962 ┆ 14,016 ┆ 10,624 ┆ 13,084 ┆ 14,474 ┆ 19,966 ┆ 90,878 ┆ 165,690 ┆ 169,274 ┆ 229,682 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(4),var(4) ┆ 16,736 ┆ 17,966 ┆ 16,780 ┆ 16,360 ┆ 16,634 ┆ 17,964 ┆ 20,088 ┆ 19,064 ┆ 24,506 ┆ 68,304 ┆ 75,174 ┆ 104,700 ┆ 141,570 ┆ 176,032 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)      ┆ 1.3    ┆ 1.5    ┆ 2.2    ┆ 1.1    ┆ 1.2    ┆ 1.3    ┆ 1.9    ┆ 1.5    ┆ 1.7    ┆ 3.4    ┆ 1.2    ┆ 1.6     ┆ 1.2     ┆ 1.3     │
└────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴─────────┘
```

### Accumulate: Parallel: 1 thread, 1 accumulator vs 4 threads, 4 accumulators, mpsc

Prefer single thread, single accumulator.

```sh
┌────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬─────────┐
│ len                    ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192   ┆ 16384  ┆ 32768   ┆ 65536   ┆ 131072  │
╞════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪═════════╪═════════╡
│ acm,lop,unr(0)         ┆ 4      ┆ 8      ┆ 24     ┆ 10     ┆ 18     ┆ 40     ┆ 114    ┆ 162    ┆ 312    ┆ 690    ┆ 1,838  ┆ 4,194   ┆ 9,516   ┆ 21,652  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(4),var(4) ┆ 21,540 ┆ 18,648 ┆ 19,180 ┆ 16,006 ┆ 16,184 ┆ 19,348 ┆ 20,046 ┆ 21,188 ┆ 20,478 ┆ 26,048 ┆ 97,520 ┆ 107,120 ┆ 137,950 ┆ 176,004 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)      ┆ 5,385  ┆ 2,331  ┆ 799    ┆ 1,600  ┆ 899    ┆ 483    ┆ 175    ┆ 130    ┆ 65     ┆ 37     ┆ 53     ┆ 25      ┆ 14      ┆ 8.1     │
└────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴─────────┘
```

### Accumulate: Parallel: 4 threads, 4 accumulators, mspc vs 8 threads, 8 accumulators, mpsc

Prefer 4 threads, 4 accumulators.

```sh
┌────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬─────────┐
│ len                    ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192   ┆ 16384  ┆ 32768   ┆ 65536   ┆ 131072  │
╞════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╪═════════╪═════════╡
│ acm,mpsc,pll(4),var(4) ┆ 17,502 ┆ 18,440 ┆ 15,356 ┆ 15,318 ┆ 22,854 ┆ 21,888 ┆ 19,370 ┆ 21,734 ┆ 25,494 ┆ 31,432 ┆ 50,824 ┆ 102,810 ┆ 153,184 ┆ 189,592 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(8),var(8) ┆ 42,524 ┆ 34,378 ┆ 77,716 ┆ 83,572 ┆ 37,062 ┆ 26,914 ┆ 31,024 ┆ 25,520 ┆ 25,710 ┆ 28,488 ┆ 35,308 ┆ 102,234 ┆ 97,490  ┆ 195,394 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)      ┆ 2.4    ┆ 1.9    ┆ 5.1    ┆ 5.5    ┆ 1.6    ┆ 1.2    ┆ 1.6    ┆ 1.2    ┆ 1      ┆ 1.1    ┆ 1.4    ┆ 1       ┆ 1.6     ┆ 1       │
└────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴─────────┘
```

### Accumulate: Parallel: 8 threads, 8 accumulators, mspc vs 16 threads, 16 accumulators, mpsc

Prefer 16 threads. Based on array length.

```sh
┌──────────────────────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬─────────┐
│ len                      ┆ 16     ┆ 32     ┆ 64     ┆ 128    ┆ 256    ┆ 512    ┆ 1024   ┆ 2048   ┆ 4096   ┆ 8192   ┆ 16384  ┆ 32768  ┆ 65536  ┆ 131072  │
╞══════════════════════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪═════════╡
│ acm,mpsc,pll(8),var(8)   ┆ 92,006 ┆ 65,698 ┆ 38,986 ┆ 62,264 ┆ 78,024 ┆ 26,742 ┆ 23,334 ┆ 24,964 ┆ 27,164 ┆ 31,152 ┆ 35,682 ┆ 86,152 ┆ 91,126 ┆ 193,204 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ acm,mpsc,pll(16),var(16) ┆ 37,610 ┆ 29,558 ┆ 31,118 ┆ 33,060 ┆ 32,712 ┆ 33,690 ┆ 33,050 ┆ 34,400 ┆ 47,812 ┆ 36,602 ┆ 45,736 ┆ 48,402 ┆ 68,382 ┆ 123,410 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ratio (max / min)        ┆ 2.4    ┆ 2.2    ┆ 1.3    ┆ 1.9    ┆ 2.4    ┆ 1.3    ┆ 1.4    ┆ 1.4    ┆ 1.8    ┆ 1.2    ┆ 1.3    ┆ 1.8    ┆ 1.3    ┆ 1.6     │
└──────────────────────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴─────────┘
```

## Development notes

[Why my Rust benchmarks were wrong, or how to correctly use std::hint::black_box?](https://gendignoux.com/blog/2022/01/31/rust-benchmarks.html)

[Counting exactly the number of distinct elements: sorted arrays vs. hash sets?](https://lemire.me/blog/2017/05/23/counting-exactly-the-number-of-distinct-elements-sorted-arrays-vs-hash-sets/)
     
[Counting CPU cycles](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/05/23/benchmark.h#L5) Daniel Lemire, C code: `RDTSC_START`, `RDTSC_STOP`

[RDTSC — Read Time-Stamp Counter](https://www.felixcloutier.com/x86/rdtsc)

Emit assembly code from Rust:
* `cargo rustc -- --emit asm`
* Read assembly file `target/debug/deps/mtr-44866ab166973511.s`.

