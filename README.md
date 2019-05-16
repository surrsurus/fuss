# fuss [![Build Status](https://travis-ci.org/surrsurus/fuss.svg?branch=master)](https://travis-ci.org/surrsurus/fuss) [![Crates.io](https://img.shields.io/crates/v/fuss.svg)](https://crates.io/crates/fuss) [![Docs.rs](https://docs.rs/fuss/badge.svg)](https://docs.rs/fuss/0.2.2/fuss/) ![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-green.svg) [![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Fuss - Small, lightweight simplex noise generator for Rust.

This library only provides functionality for generating 2D and 3D simplex noise, as well as the ability to normalize the noise through sum octaves (fractal Brownian motion).

[View the Docs](https://docs.rs/fuss/0.2.2/fuss/)

## Getting Started

### Prerequisites

Add fuss to your `Cargo.toml`:

```toml
[dependencies]
# Get the current stable
fuss = "0.3.0"
```

You also need a random number generator that implements `SeedableRng + RngCore` from the rand crate. For example:

```toml
[dependencies]
# Get the current stable
fuss = "0.3.0"
# Get a fast, but non-cryptographic random number generator
rand_xoshiro = "0.1"
```

The next section will use this generator in the examples.

### Examples

All fuss interactions happen through the `Simplex` struct.

Here's how you get one:

```rust
extern crate fuss;
extern crate rand_xoshiro;
use fuss::Simplex;
use rand_xoshiro::Xoshiro256Plus;

let sn = Simplex::new::<Xoshiro256Plus>();
```

`Simplex` lets you generate 2D and 3D noise.

```rust
let sn = Simplex::new::<Xoshiro256Plus>();
sn.noise_2d(1.0, -1.0);
sn.noise_3d(1.0, -1.0, 0.0);
```

Which lets you generate noise for large sets of points:

```rust
let sn = Simplex::new::<Xoshiro256Plus>();

let mut map = Vec::<Vec<Vec<f32>>>::new();
for x in 0..10 {
  map.push(Vec::<Vec<f32>>::new());
  for y in 0..10 {
    map[x as usize].push(Vec::<f32>::new());
    for z in 0..10 {
      map[x as usize][y as usize].push(sn.noise_3d(x as f32, y as f32, z as f32));
    }
  }
}
```

`Simplex` generates its own permutation table based on the thread-local RNG, however you can apply your own seed to get a `Simplex` as such:

```rust
let sn = Simplex::from_seed::<Xoshiro256Plus>([0, 1, 2, 3, 4, 5, ...]);
let other_sn = Simplex::from_seed::<Xoshiro256Plus>([0, 1, 2, 3, 4, 5, ...]);

// The two simplexes will generate the same noise for the same points
// if given the same RNG seed
assert_eq!(sn.noise_2d(0.0, 0.0), other_sn.noise_2d(0.0, 0.0));
```

The type of the seed is `SeedableRng::Seed`, so it depends on which random number generator you're using. For `Xoshiro256Plus`, it's `[u8; 32]`.

### Running the tests

Run

```bash
$ cargo test
```

to run all unit tests and doc tests.

## Versioning

fuss follows cargo's rules of Semantic Versioning:

  - Before we reach 1.0.0, anything goes, but if we make breaking changes, we increment the minor version. In Rust, breaking changes include adding fields to structs or variants to enums.
  - After 1.0.0, we only make breaking changes when we increment the major version.
  - After 1.0.0, we don’t add any new public API (no new pub anything) in tiny versions. We always increment the minor version if you add any new pub structs, traits, fields, types, functions, methods or anything else.
  - We use version numbers with three numeric parts such as 1.0.0 rather than 1.0.

## License

This project is licensed under the MPPL 2.0 License - See the [LICENSE.md](LICENSE) file for details


