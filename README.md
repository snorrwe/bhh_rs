# Bounding Half-Space Hierarchy

Port of [https://github.com/bryanmcnett/bhh](https://github.com/bryanmcnett/bhh)

## Prerequisites

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Running benchmarks

- `cargo bench`

## Benchmark details

Example output

```
test bench_bhh_ordered            ... bench:   3,413,233 ns/iter (+/- 2,454,793)
test bench_bhh_unordered          ... bench:  84,368,470 ns/iter (+/- 2,732,935)
test bench_naive_ordered          ... bench: 196,799,830 ns/iter (+/- 61,523,953)
test bench_naive_unordered        ... bench: 384,638,430 ns/iter (+/- 33,605,416)
test bench_sorting                ... bench:  85,186,500 ns/iter (+/- 19,263,717)
test bench_sorting_already_sorted ... bench:  56,740,580 ns/iter (+/- 9,606,894)
```

- `bench_bhh_ordered`: Testing `bhh_search` on an already ordered collection
- `bench_bhh_unordered`: Testing cloning the unordered collection, sorting it using `bhh_sort` then using `bhh_search` on the ordered collection
- `bench_naive_ordered`: Testing the naive method* on the ordered collection
- `bench_naive_unordered`: Testing the naive method* on the unordered collection
- `bench_sorting`: Testing `bhh_sort` on the unordered collection
- `bench_sorting_already_sorted`: Testing `bhh_sort` on the already ordered collection


\* The naive method:

Iterate on all elements and check for intersections

```
fn naive_search(items: &[AABB], query: &AABB) -> u32 {
    items.iter().filter(|a| a.intersects(query)).count() as u32
}
```
