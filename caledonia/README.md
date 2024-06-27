# Caledonia

Prototype implementation of _Approximate Lower Bound Arguments_ in Rust from the [paper](https://iohk.io/en/research/library/papers/approximate-lower-bound-arguments/) published by IOG Research:


[!IMPORTANT]
> This code is NOT fit for production, it's not been optimised, thoroughly tested, nor audited.

## Build
```
cargo build
```

## Test
```
cargo test
```

## Benchmarks

## Doc
```
cargo doc
```

## Running times
Here we give the running times for the ALBA's prehashed scheme and bounded DFS scheme. We run the test on macOS 14.5 on an Apple M1 Pro machine with 16 GB of RAM.

* ALBA prehashed
```shell
(n_p=10_0, n_f=2_0, λ=5): 	 u=4, 	 setup:1μs, 	 prove:79ms, 	 verify:83μs
(n_p=10_0, n_f=2_0, λ=10): 	 u=7, 	 setup:2μs, 	 prove:177ms, 	 verify:213μs
(n_p=100_0, n_f=60_0, λ=5): 	 u=12, 	 setup:1μs, 	 prove:818ms, 	 verify:233μs
(n_p=100_0, n_f=60_0, λ=10): 	 u=20, 	 setup:1μs, 	 prove:1s, 	 verify:595μs
```
* ALBA bounded DFS
```shell
(n_p=100, n_f=20, λ=10): 	 u=11, 	 setup:424ns, 	 prove:9ms, 	 verify:198μs, 	 max steps:3_746, 	 mean steps:761, 	 max retrial:0, 	 mean retrial:0
(n_p=100, n_f=20, λ=20): 	 u=17, 	 setup:300ns, 	 prove:16ms, 	 verify:301μs, 	 max steps:6_547, 	 mean steps:1_174, 	 max retrial:0, 	 mean retrial:0
(n_p=1_0, n_f=8, λ=10): 	 u=11, 	 setup:503ns, 	 prove:12ms, 	 verify:199μs, 	 max steps:1_13, 	 mean steps:206, 	 max retrial:0, 	 mean retrial:0
(n_p=1_0, n_f=8, λ=20): 	 u=17, 	 setup:685ns, 	 prove:18ms, 	 verify:306μs, 	 max steps:1_301, 	 mean steps:314, 	 max retrial:0, 	 mean retrial:0
(n_p=1_0, n_f=200, λ=10): 	 u=11, 	 setup:620ns, 	 prove:11ms, 	 verify:200μs, 	 max steps:689, 	 mean steps:162, 	 max retrial:0, 	 mean retrial:0
(n_p=1_0, n_f=200, λ=20): 	 u=17, 	 setup:695ns, 	 prove:18ms, 	 verify:306μs, 	 max steps:1_155, 	 mean steps:306, 	 max retrial:0, 	 mean retrial:0
```