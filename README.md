# fuss [![Build Status](https://travis-ci.org/surrsurus/fuss.svg?branch=master)](https://travis-ci.org/surrsurus/fuss) ![Language](https://img.shields.io/badge/rust-stable-orange.svg) [![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Fuss - Small, lightweight simplex noise generator for Rust.

This library only provides functionality for generating 2D and 3D simplex noise, as well as the ability to normalize the noise through sum octaves (fractal Brownian motion).

[View the Docs](http://surrsur.us/fuss/fuss/index.html).

## Getting Started

### Prerequisites

Add fuss to your `Cargo.toml` in one of two ways:

```toml
[dependencies]
# Get the current stable
fuss = "0.2.0"
# Get the latest master version
fuss = { git = "https://github.com/surrsurus/fuss.git" }
```

### Examples

All fuss interactions happen through the `Simplex` struct. 

Here's how you get one:

```rust
extern crate fuss;
use fuss::Simplex;

let sn = Simplex::new();
```

`Simplex` lets you generate 2D and 3D noise.

```rust
let sn = Simplex::new();
sn.noise_2d(1.0, -1.0);
sn.noise_3d(1.0, -1.0, 0.0);
```

Which lets you generate noise for large sets of points:

```rust
let sn = Simplex::new();

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

`Simplex` generates it's own permutation table based on the rust RNG, however you can apply your own seed to get a `Simplex` as such:

```rust
let sn = Simplex::from_seed(vec![0, 1, 2, 3, 4, 5]);
let other_sn = Simplex::from_seed(vec![0, 1, 2, 3, 4, 5]);

// The two simplexes will generate the same noise for the same points
// if given the same RNG seed
assert_eq!(sn.noise_2d(0.0, 0.0), other_sn.noise_2d(0.0, 0.0));
```

`Simplex` uses the same type of seeds that `SeedableRng` does, being a slice of `usize`s, however to make this easier `Simplex` will accept a `Vec<usize>` instead.

### Running the tests

Run

```bash
$ cargo test
```

to run all unit tests and doc tests.

## License

This project is licensed under the MPPL 2.0 License - See the [LICENSE.md](LICENSE.md) file for details


