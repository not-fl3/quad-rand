# quad-rand

[![Crates.io version](https://img.shields.io/crates/v/quad-rand.svg)](https://crates.io/crates/quad-rand)
[![Documentation on docs.rs](https://docs.rs/quad-rand/badge.svg)](https://docs.rs/quad-rand)

`quad-rand` implements pseudo-random generator http://www.pcg-random.org/download.html based on rust atomics. 

Compatible with wasm and also no-std compatible.

Basic usage, no dependencies involved:
```rust
use quad_rand as qrand;

// seed random
qrand::srand(12345);

// get random number from 0 to u32::MAX
let x = qrand::rand();

// get random number from given range
let x = qrand::gen_range(0., 1.);
assert!(x >= 0. && x < 1.);

// gen_range works for most of standard number types
let x: u8 = qrand::gen_range(64, 128);
assert!(x >= 64 && x < 128);
```

Optional compatibility layer with `rand` crate:

```rust
use quad_rand::compat::QuadRand;
use rand::seq::SliceRandom;

let mut vec = vec![1, 2, 3, 4, 5, 6];

// QuadRand is rand::RngCore implementation, allowing to use all the cool stuff from rand
vec.shuffle(&mut QuadRand);

```
