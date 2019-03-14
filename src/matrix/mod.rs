use crate::vectors::Tuple;

#[derive(Clone)]
pub struct Matrix {
  rows: usize,
  cols: usize,
  data: Vec<f64>,
}

impl Matrix {
  pub fn new(rows: usize, cols: usize, d: Vec<f64>) -> Matrix {
    Matrix {
      data: d,
      rows: rows,
      cols: cols,
    }
  }

  pub fn get(&self, r: usize, c: usize) -> f64 {
    return self.data[r * self.cols + c];
  }

  pub fn set(&mut self, r: usize, c: usize, d: f64) {
    self.data[r * self.cols + c] = d;
  }

  pub fn equals(a: &Matrix, b: &Matrix) -> bool {
    if a.rows != b.rows || a.cols != b.cols {
      return false;
    }

    for i in 0..a.rows * a.cols {
      if (a.data[i] - b.data[i]).abs() > 1e-10 {
        return false;
      }
    }

    return true;
  }

  pub fn approx_equals(a: &Matrix, b: &Matrix) -> bool {
    if a.rows != b.rows || a.cols != b.cols {
      return false;
    }

    for i in 0..a.rows * a.cols {
      if (a.data[i] - b.data[i]).abs() > 1e-4 {
        return false;
      }
    }

    return true;
  }

  pub fn mult(a: &Matrix, b: &Matrix) -> Matrix {
    let mut out: Vec<f64> = Vec::new();
    for row in 0..a.rows {
      for col in 0..a.cols {
        let mut sum: f64 = 0.;
        for k in 0..a.rows {
          sum = sum + a.get(row, k) * b.get(k, col);
        }
        out.push(sum);
      }
    }
    return Matrix::new(4, 4, out);
  }

  pub fn transpose(a: &Matrix) -> Matrix {
    let mut out: Vec<f64> = Vec::new();
    for col in 0..a.cols {
      for row in 0..a.rows {
        out.push(a.get(row, col));
      }
    }
    return Matrix::new(4, 4, out);
  }

  pub fn mult_4x4_by_1d(a: &Matrix, b: &Tuple) -> Tuple {
    return Tuple::new(
      a.get(0, 0) * b.x + a.get(0, 1) * b.y + a.get(0, 2) * b.z + a.get(0, 3) * b.w,
      a.get(1, 0) * b.x + a.get(1, 1) * b.y + a.get(1, 2) * b.z + a.get(1, 3) * b.w,
      a.get(2, 0) * b.x + a.get(2, 1) * b.y + a.get(2, 2) * b.z + a.get(2, 3) * b.w,
      a.get(3, 0) * b.x + a.get(3, 1) * b.y + a.get(3, 2) * b.z + a.get(3, 3) * b.w,
    );
  }

  pub fn identity(size: usize) -> Matrix {
    if size == 2 {
      let identity = vec![1., 0., 0., 1.];
      return Matrix::new(2, 2, identity);
    }

    if size == 3 {
      let identity = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
      return Matrix::new(3, 3, identity);
    }

    if size == 4 {
      let identity = vec![
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
      ];
      return Matrix::new(4, 4, identity);
    }
    panic!("bad size");
  }

  pub fn determinant(a: &Matrix) -> f64 {
    let mut determinant = 0.;

    if a.rows == 2 {
      return a.get(0, 0) * a.get(1, 1) - a.get(0, 1) * a.get(1, 0);
    }

    for i in 0..a.cols {
      determinant += Matrix::cofactor(&a, 0, i) * a.get(0, i);
    }
    return determinant;
  }

  pub fn submatrix(a: &Matrix, row: usize, col: usize) -> Matrix {
    if row >= a.rows || col >= a.cols {
      panic!("bad row or col param");
    }

    let mut out: Vec<f64> = Vec::new();

    for i in 0..a.rows {
      if i != row {
        for j in 0..a.cols {
          if j != col {
            out.push(a.get(i, j));
          }
        }
      }
    }
    return Matrix::new(a.rows - 1, a.cols - 1, out);
  }

  pub fn minor(a: &Matrix, row: usize, col: usize) -> f64 {
    if row >= a.rows || col >= a.cols {
      panic!("bad row or col param");
    }

    let sub = Matrix::submatrix(&a, row, col);
    return Matrix::determinant(&sub);
  }

  pub fn cofactor(a: &Matrix, row: usize, col: usize) -> f64 {
    if row >= a.rows || col >= a.cols {
      panic!("bad row or col param");
    }

    let sub = Matrix::submatrix(a, row, col);
    let minor = Matrix::determinant(&sub);

    if (row + col) % 2 == 1 {
      return -minor;
    }
    return minor;
  }

  pub fn invertible(a: &Matrix) -> bool {
    if Matrix::determinant(a) != 0. {
      return true;
    }
    return false;
  }

  pub fn inverse(a: &Matrix) -> Matrix {
    if !Matrix::invertible(a) {
      panic!("cannot invert matrix")
    }

    let mut out: Vec<f64> = Vec::new();
    let d = Matrix::determinant(a);

    for i in 0..a.rows {
      for j in 0..a.cols {
        let c = Matrix::cofactor(a, i, j);
        out.push(c / d);
      }
    }
    return Matrix::transpose(&Matrix::new(a.rows, a.cols, out));
  }
}

#[test]
fn it_creates_a_2x2_matrix() {
  let flat_data = vec![-3., 5., 1., -2.];
  let m = Matrix::new(2, 2, flat_data);

  assert_eq!(m.get(0, 0), -3.0);
  assert_eq!(m.get(0, 1), 5.);
  assert_eq!(m.get(1, 0), 1.);
  assert_eq!(m.get(1, 1), -2.);
}

#[test]
fn it_creates_a_3x3_matrix() {
  let flat_data = vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.];
  let m = Matrix::new(3, 3, flat_data);

  assert_eq!(m.get(0, 0), -3.0);
  assert_eq!(m.get(1, 1), -2.);
  assert_eq!(m.get(2, 2), 1.);
}

#[test]
fn it_creates_a_4x4_matrix() {
  let flat_data = vec![
    1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
  ];
  let m = Matrix::new(4, 4, flat_data);

  assert_eq!(m.get(0, 0), 1.0);
  assert_eq!(m.get(1, 0), 5.5);
  assert_eq!(m.get(1, 2), 7.5);
  assert_eq!(m.get(2, 2), 11.0);
}

#[test]
fn it_checks_for_equality_for_identical_matrices() {
  let flat_data_1 = vec![
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
  ];
  let flat_data_2 = vec![
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let m_2 = Matrix::new(4, 4, flat_data_2);

  assert_eq!(Matrix::equals(&m_1, &m_2), true);
}

#[test]
fn it_checks_for_not_equality_for_different_matrices() {
  let flat_data_1 = vec![
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
  ];
  let flat_data_2 = vec![
    5., 4., 3., 2., 1., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let m_2 = Matrix::new(4, 4, flat_data_2);

  assert_eq!(Matrix::equals(&m_1, &m_2), false);
}

#[test]
fn it_multiplies_matrices() {
  let flat_data_1 = vec![
    1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
  ];
  let flat_data_2 = vec![
    -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
  ];
  let expected_prod_data = vec![
    20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let m_2 = Matrix::new(4, 4, flat_data_2);
  let expected_prod = Matrix::new(4, 4, expected_prod_data);
  let prod = Matrix::mult(&m_1, &m_2);

  assert_eq!(Matrix::equals(&prod, &expected_prod), true);
}

#[test]
fn it_multiplies_matrices_by_tuple() {
  let flat_data_1 = vec![
    1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
  ];
  let tuple = Tuple::new(1., 2., 3., 1.);
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let prod = Matrix::mult_4x4_by_1d(&m_1, &tuple);
  let expected = Tuple::new(18., 24., 33., 1.);

  assert_eq!(expected.equals(prod), true);
}

#[test]
fn it_multiplies_matrix_by_identity() {
  let flat_data_1 = vec![
    0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let prod = Matrix::mult(&m_1, &(Matrix::identity(4)));

  assert_eq!(Matrix::equals(&prod, &m_1), true);
}

#[test]
fn it_returns_submatrix_of_3x3() {
  let flat_data_1 = vec![1., 5., 0., -3., 2., 7., 0., 6., -3.];
  let expected_data = vec![-3., 2., 0., 6.];
  let m = Matrix::new(3, 3, flat_data_1);
  let submat = Matrix::submatrix(&m, 0, 2);
  let expected = Matrix::new(2, 2, expected_data);
  assert_eq!(Matrix::equals(&submat, &expected), true);
}

#[test]
fn it_returns_submatrix_of_4x4() {
  let flat_data_1 = vec![
    -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
  ];
  let expected_data = vec![-6., 1., 6., -8., 8., 6., -7., -1., 1.0];
  let m = Matrix::new(4, 4, flat_data_1);
  let submat = Matrix::submatrix(&m, 2, 1);
  let expected = Matrix::new(3, 3, expected_data);
  assert_eq!(Matrix::equals(&submat, &expected), true);
}

#[test]
fn it_gets_minor_of_3x3() {
  let flat_data_1 = vec![3., 5., 0., 2., -1., -7., 6., -1., 5.];
  let m = Matrix::new(3, 3, flat_data_1);
  let minor = Matrix::minor(&m, 1, 0);
  assert_eq!(minor, 25.);
}

#[test]
fn it_gets_cofactor_of_3x3_even() {
  let flat_data_1 = vec![3., 5., 0., 2., -1., -7., 6., -1., 5.];
  let m = Matrix::new(3, 3, flat_data_1);
  let minor = Matrix::minor(&m, 0, 0);
  let cofactor = Matrix::cofactor(&m, 0, 0);
  assert_eq!(minor, -12.);
  assert_eq!(cofactor, -12.);
}

#[test]
fn it_gets_cofactor_of_3x3_odd() {
  let flat_data_1 = vec![3., 5., 0., 2., -1., -7., 6., -1., 5.];
  let m = Matrix::new(3, 3, flat_data_1);
  let minor = Matrix::minor(&m, 1, 0);
  let cofactor = Matrix::cofactor(&m, 1, 0);
  assert_eq!(minor, 25.);
  assert_eq!(cofactor, -25.);
}

#[test]
fn it_calculates_determinant_of_3x3() {
  let flat_data_1 = vec![1., 2., 6., -5., 8., -4., 2., 6., 4.];
  let m = Matrix::new(3, 3, flat_data_1);
  let determinant = Matrix::determinant(&m);
  assert_eq!(determinant, -196.);
}

#[test]
fn it_calculates_determinant_of_4x4() {
  let flat_data_1 = vec![
    -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
  ];
  let m = Matrix::new(4, 4, flat_data_1);
  let determinant = Matrix::determinant(&m);
  assert_eq!(determinant, -4071.);
}

#[test]
fn it_calculates_determinant_of_2x2_pt2() {
  let flat_data_1 = vec![1., 5., -3., 2.];
  let m = Matrix::new(2, 2, flat_data_1);
  let determinant = Matrix::determinant(&m);
  assert_eq!(determinant, 17.0);
}

#[test]
fn it_transposes_a_matrix() {
  let flat_data_1 = vec![
    0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
  ];
  let expected_data = vec![
    0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);
  let expected = Matrix::new(4, 4, expected_data);
  let transposed = Matrix::transpose(&m_1);

  assert_eq!(Matrix::equals(&transposed, &expected), true);
}

#[test]
fn it_transposes_an_identity_matrix() {
  let i = Matrix::identity(4);
  let transposed = Matrix::transpose(&i);

  assert_eq!(Matrix::equals(&transposed, &i), true);
}

#[test]
fn it_checks_if_invertible() {
  let flat_data_1 = vec![
    6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);

  assert_eq!(Matrix::invertible(&m_1), true);
}

#[test]
fn it_checks_if_not_invertible() {
  let flat_data_1 = vec![
    -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
  ];
  let m_1 = Matrix::new(4, 4, flat_data_1);

  assert_eq!(Matrix::invertible(&m_1), false);
}

#[test]
fn it_inverts_a_matrix() {
  let flat_data = vec![
    -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
  ];
  let m = Matrix::new(4, 4, flat_data);
  let inv = Matrix::inverse(&m);

  let expected_data = vec![
    0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895, -0.22368,
    -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
  ];
  let expected = Matrix::new(4, 4, expected_data);

  let cof1 = Matrix::cofactor(&m, 2, 3);
  assert_eq!(cof1, -160.);
  assert_eq!(inv.get(3, 2), -160. / 532.);

  let cof2 = Matrix::cofactor(&m, 3, 2);
  assert_eq!(cof2, 105.);
  assert_eq!(inv.get(2, 3), 105. / 532.);

  assert_eq!(Matrix::approx_equals(&inv, &expected), true);

  let prod = Matrix::mult(&m, &inv);
  assert_eq!(Matrix::approx_equals(&prod, &Matrix::identity(4)), true);
}
