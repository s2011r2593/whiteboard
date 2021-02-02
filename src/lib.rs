pub mod trig {
  pub fn get_unit_vec(a: Vec<f64>) -> Vec<f64> {
    let mut mag: f64 = 0.0;
    for i in a.iter() {
      mag += i*i;
    }
    a.iter().map(|x| x / mag.sqrt()).collect::<Vec<f64>>()
  }

  pub fn get_theta(a: Vec<f64>, b: Vec<f64>) -> f64 {
    dot(a, b).acos()
  }

  pub fn dot(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut res: f64 = 0.0;
    for (i, j) in a.iter().zip(b.iter()) {
      res += i * j;
    }
    res
  }
}

pub mod lin_alg {
  pub fn gauss_jordan(mut aug: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let length: usize = aug.len();
        
    for i in 0..(length - 1) {
      for j in (i+1)..length {
        let co_i = aug[i][i];
        let co_j = aug[j][i];
        for k in i..(length + 1) {
          aug[j][k] -= (co_j / co_i) * aug[i][k];
        }
      }
    }
    for i in (0..length).rev() {
      let co_i: f64 = aug[i][i];
      aug[i][i] /= co_i;
      aug[i][length] /= co_i;
      for j in 0..i {
        let co_j: f64 = aug[j][i];
        aug[j][length] -= co_j * aug[i][length];
      }
    }
       
    aug
  }
}

pub mod misc {
  pub fn rotate(point: (f64, f64), center: (f64, f64), theta: f64) -> (f64, f64) {
    (
      (point.0 - center.0) * theta.cos() - (point.1 - center.1) * theta.sin() + center.0,
      (point.0 - center.0) * theta.sin() + (point.1 - center.1) * theta.cos() + center.1
    )
  }
}