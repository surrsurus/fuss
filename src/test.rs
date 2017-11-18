#[cfg(test)]
mod tests {
  use Simplex;

  #[test]
  fn test_simplex() {
    let sn = Simplex::new();
    sn.noise_2d(0.0, 0.0);
  }

  #[test]
  fn test_from_seed() {
    let mut sn = Simplex::from_seed(vec![1, 2, 3]);
    let mut other_sn = Simplex::from_seed(vec![1, 2, 3]);

    assert_eq!(other_sn.noise_2d(1.0, 14.2), sn.noise_2d(1.0, 14.2));
    assert_eq!(other_sn.noise_3d(1.0, 14.2, -5.4), sn.noise_3d(1.0, 14.2, -5.4));

    sn = Simplex::from_seed(vec![4, 5, 6]);
    other_sn = Simplex::from_seed(vec![1, 2, 3]);

    assert!(other_sn.noise_2d(1.0, 14.2) != sn.noise_2d(1.0, 14.2));
    assert!(other_sn.noise_3d(1.0, 14.2, -5.4) != sn.noise_3d(1.0, 14.2, -5.4));
  }

  #[test]
  fn test_sum_octave_2d() {
    let sn = Simplex::new();
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
    let sn = Simplex::new();
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
    let sn = Simplex::from_seed(vec![5, 3, 2, 1, 1]);
    assert_eq!(sn.noise_2d(1.5, -0.5), sn.noise_2d(1.5, -0.5));
    let other_sn = Simplex::from_seed(vec![0, 1, 2, 3, 4, 5]);
    assert!(sn.noise_2d(1.5, -0.5) != other_sn.noise_2d(1.5, -0.5));
  }

  #[test]
  fn test_noise_3d() {
    let sn = Simplex::from_seed(vec![5, 3, 2, 1, 1]);
    assert_eq!(sn.noise_3d(1.5, -0.5, 2.1), sn.noise_3d(1.5, -0.5, 2.1));
    let other_sn = Simplex::from_seed(vec![0, 1, 2, 3, 4, 5]);
    assert!(sn.noise_3d(1.5, -0.5, 2.1) != other_sn.noise_3d(1.5, -0.5, 2.1));
  }

}