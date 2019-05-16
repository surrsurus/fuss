#[cfg(test)]
mod tests {
  extern crate rand_xoshiro;
  use Simplex;
  use self::rand_xoshiro::Xoshiro256Plus;

  #[test]
  fn test_simplex() {
    let sn = Simplex::new::<Xoshiro256Plus>();
    sn.noise_2d(0.0, 0.0);
  }

  #[test]
  fn test_from_seed() {
    let mut sn = Simplex::from_seed::<Xoshiro256Plus>([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
    let mut other_sn = Simplex::from_seed::<Xoshiro256Plus>([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);

    assert_eq!(other_sn.noise_2d(1.0, 14.2), sn.noise_2d(1.0, 14.2));
    assert_eq!(other_sn.noise_3d(1.0, 14.2, -5.4), sn.noise_3d(1.0, 14.2, -5.4));

    sn = Simplex::from_seed::<Xoshiro256Plus>([5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
    other_sn = Simplex::from_seed::<Xoshiro256Plus>([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);

    assert!(other_sn.noise_2d(1.0, 14.2) != sn.noise_2d(1.0, 14.2));
    assert!(other_sn.noise_3d(1.0, 14.2, -5.4) != sn.noise_3d(1.0, 14.2, -5.4));
  }

  #[test]
  fn test_sum_octave_2d() {
    let sn = Simplex::new::<Xoshiro256Plus>();
    let mut luminance = Vec::<Vec<f32>>::new();
    for x in 0..100 {
      luminance.push(Vec::<f32>::new());
      for y in 0..100 {
        luminance[x as usize].push(sn.sum_octave_2d(16, x as f32, y as f32, 0.5, 0.008));
      }
    }
  }

  #[test]
  fn test_sum_octave_3d() {
    let sn = Simplex::new::<Xoshiro256Plus>();
    let mut luminance = Vec::<Vec<Vec<f32>>>::new();
    for x in 0..10 {
      luminance.push(Vec::<Vec<f32>>::new());
      for y in 0..10 {
        luminance[x as usize].push(Vec::<f32>::new());
        for z in 0..10 {
          luminance[x as usize][y as usize].push(sn.sum_octave_3d(16, x as f32, y as f32, z as f32, 0.5, 0.008));
        }
      }
    }
  }

  #[test]
  fn test_noise_2d() {
    let sn = Simplex::from_seed::<Xoshiro256Plus>([5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
    assert_eq!(sn.noise_2d(1.5, -0.5), sn.noise_2d(1.5, -0.5));
    let other_sn = Simplex::from_seed::<Xoshiro256Plus>([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    assert!(sn.noise_2d(1.5, -0.5) != other_sn.noise_2d(1.5, -0.5));
  }

  #[test]
  fn test_noise_3d() {
    let sn = Simplex::from_seed::<Xoshiro256Plus>([5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
    assert_eq!(sn.noise_3d(1.5, -0.5, 2.1), sn.noise_3d(1.5, -0.5, 2.1));
    let other_sn = Simplex::from_seed::<Xoshiro256Plus>([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    assert!(sn.noise_3d(1.5, -0.5, 2.1) != other_sn.noise_3d(1.5, -0.5, 2.1));
  }

}