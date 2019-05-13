# Bounding Half-Space Hierarchy

Port of [https://github.com/bryanmcnett/bhh](https://github.com/bryanmcnett/bhh)

[![CircleCI](https://circleci.com/gh/snorrwe/bhh_rs/tree/master.svg?style=svg)](https://circleci.com/gh/snorrwe/bhh_rs/tree/master)

## Prerequisites

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Running benchmarks

- `cargo bench`

## Benchmark details

Example output

```
test bench_bhh_ordered            ... bench:   3,599,895 ns/iter (+/- 296,112)
test bench_bhh_unordered          ... bench:  78,478,130 ns/iter (+/- 16,134,899)
test bench_naive_par_ordered      ... bench: 124,591,830 ns/iter (+/- 54,217,048)
test bench_naive_par_unordered    ... bench: 107,692,180 ns/iter (+/- 31,061,016)
test bench_naive_ordered          ... bench: 195,426,500 ns/iter (+/- 34,261,012)
test bench_naive_unordered        ... bench: 371,697,510 ns/iter (+/- 13,081,041)
test bench_sorting                ... bench:  72,915,040 ns/iter (+/- 3,867,762)
test bench_sorting_already_sorted ... bench:  51,108,080 ns/iter (+/- 2,793,440)
```

- `bench_bhh_ordered`: Testing `bhh_search` on an already ordered collection
- `bench_bhh_unordered`: Testing cloning the unordered collection, sorting it using `bhh_sort` then using `bhh_search` on the ordered collection
- `bench_naive_ordered`: Testing the naive method* on the ordered collection
- `bench_naive_unordered`: Testing the naive method* on the unordered collection
- `bench_naive_par_ordered`: Testing the naive paralell method** on the ordered collection
- `bench_naive_par_unordered`: Testing the naive paralell method** on the unordered collection
- `bench_sorting`: Testing `bhh_sort` on the unordered collection
- `bench_sorting_already_sorted`: Testing `bhh_sort` on the already ordered collection


\* The naive method:

Iterate on all elements and check for intersections

```
fn naive_search(items: &[AABB], query: &AABB) -> u32 {
    items.iter().filter(|a| a.intersects(query)).count() as u32
}
```


\** The naive paralell method:

Iterate on all elements in paralell and check for intersections

```
fn naive_par_search(items: &[AABB], query: &AABB) -> u32 {
    use rayon::prelude::*;

    items.par_iter().filter(|a| a.intersects(query)).count() as u32
}
```
