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