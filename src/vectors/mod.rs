#[derive(Copy, Clone)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Tuple {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple {
      x: x,
      y: y,
      z: z,
      w: w,
    }
  }

  pub fn is_a_point(&self) -> bool {
    return self.w == 1.0;
  }
  pub fn is_a_vector(&self) -> bool {
    return self.w == 0.0;
  }
  pub fn equals(&self, p: Tuple) -> bool {
    return (self.x - p.x).abs() < 1e-10
      && (self.y - p.y).abs() < 1e-10
      && (self.z - p.z).abs() < 1e-10
      && (self.w - p.w).abs() < 1e-10;
  }
  pub fn approx_equals(&self, p: Tuple) -> bool {
    return (self.x - p.x).abs() < 1e-3
      && (self.y - p.y).abs() < 1e-3
      && (self.z - p.z).abs() < 1e-3
      && (self.w - p.w).abs() < 1e-3;
  }
  pub fn add(&self, p: Tuple) -> Tuple {
    return Tuple::new(self.x + p.x, self.y + p.y, self.z + p.z, self.w + p.w);
  }
  pub fn sub(&self, p: Tuple) -> Tuple {
    return Tuple::new(self.x - p.x, self.y - p.y, self.z - p.z, self.w - p.w);
  }
  pub fn negate(&self) -> Tuple {
    return Tuple::new(-self.x, -self.y, -self.z, -self.w);
  }
  pub fn mult(&self, a: f64) -> Tuple {
    return Tuple::new(self.x * a, self.y * a, self.z * a, self.w * a);
  }
  pub fn div(&self, a: f64) -> Tuple {
    return Tuple::new(self.x / a, self.y / a, self.z / a, self.w / a);
  }
  pub fn mag(&self) -> f64 {
    return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
  }
  pub fn norm(&self) -> Tuple {
    let m = self.mag();
    return self.div(m);
  }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
  return Tuple::new(x, y, z, 1.0);
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
  return Tuple::new(x, y, z, 0.0);
}

pub fn dot(a: Tuple, b: Tuple) -> f64 {
  return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
}

pub fn cross(a: Tuple, b: Tuple) -> Tuple {
  return vector(
    a.y * b.z - a.z * b.y,
    a.z * b.x - a.x * b.z,
    a.x * b.y - a.y * b.x,
  );
}

pub fn reflect(incidence: Tuple, norm: Tuple) -> Tuple {
  return incidence.sub(norm.mult(2. * dot(incidence, norm)));
}

#[test]
fn it_creates_a_tuple_where_w_0_is_a_vector() {
  let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
  assert_eq!(tuple.x, 4.3);
  assert_eq!(tuple.y, -4.2);
  assert_eq!(tuple.z, 3.1);
  assert_eq!(tuple.w, 0.0);
  assert_eq!(tuple.is_a_point(), false);
  assert_eq!(tuple.is_a_vector(), true);
}

#[test]
fn it_creates_a_point() {
  let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
  let p1 = point(4.3, -4.2, 3.1);
  assert_eq!(p1.equals(tuple), true);
}

#[test]
fn it_creates_a_vector() {
  let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
  let p1 = vector(4.3, -4.2, 3.1);
  assert_eq!(p1.equals(tuple), true);
}

#[test]
fn it_adds_two_tuples() {
  let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
  let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
  let sum = a1.add(a2);
  assert_eq!(sum.equals(Tuple::new(1.0, 1.0, 6.0, 1.0)), true);
}

#[test]
fn it_subtracts_two_tuples() {
  let a1 = point(3.0, 2.0, 1.0);
  let a2 = point(5.0, 6.0, 7.0);
  let sum = a1.sub(a2);
  assert_eq!(sum.equals(vector(-2.0, -4.0, -6.0)), true);
}

#[test]
fn it_subtracts_a_vector_from_a_point() {
  let p = point(3.0, 2.0, 1.0);
  let v = vector(5.0, 6.0, 7.0);
  assert_eq!(p.sub(v).equals(point(-2.0, -4.0, -6.0)), true);
}

#[test]
fn it_subtracts_a_vector_from_zero_vector() {
  let zero = vector(0.0, 0.0, 0.0);
  let v = vector(1.0, 2.0, 3.0);
  assert_eq!(zero.sub(v).equals(vector(-1.0, -2.0, -3.0)), true);
}

#[test]
fn it_negates_a_tuple() {
  let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert_eq!(a.negate().equals(Tuple::new(-1.0, 2.0, -3.0, 4.0)), true);
}

#[test]
fn it_scales_a_tuple() {
  let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert_eq!(a.mult(3.5).equals(Tuple::new(3.5, -7.0, 10.5, -14.0)), true);
}

#[test]
fn it_divides_a_tuple() {
  let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert_eq!(a.div(2.0).equals(Tuple::new(0.5, -1.0, 1.5, -2.0)), true);
}

#[test]
fn vector_0_1_0_magnitude_is_1() {
  let v = vector(0.0, 1.0, 0.0);
  assert_eq!(v.mag(), 1.0);
}

#[test]
fn vector_0_0_1_magnitude_is_1() {
  let v = vector(0.0, 0.0, 1.0);
  assert_eq!(v.mag(), 1.0);
}

#[test]
fn vector_1_2_3_magnitude_is_root_14() {
  let v = vector(1.0, 2.0, 3.0);
  assert_eq!(v.mag(), 14.0_f64.sqrt());
}

#[test]
fn vector_minus1_minus2_minus3_magnitude_is_root_14() {
  let v = vector(-1.0, -2.0, -3.0);
  assert_eq!(v.mag(), 14.0_f64.sqrt());
}

#[test]
fn normalizes_vector_4_0_0() {
  let v = vector(4.0, 0.0, 0.0);
  assert_eq!(v.norm().equals(vector(1.0, 0., 0.)), true);
}

#[test]
fn normalizes_vector_1_2_3() {
  let v = vector(1.0, 2.0, 3.0);
  assert_eq!(
    v.norm().approx_equals(vector(0.26726, 0.53452, 0.80178)),
    true
  );
}

#[test]
fn magnitude_of_normalized_vector() {
  let v = vector(1.0, 2.0, 3.0);
  let norm = v.norm();
  assert_eq!(norm.mag(), 1.0);
}

#[test]
fn it_computes_dot_product() {
  let a = vector(1., 2., 3.);
  let b = vector(2., 3., 4.);
  assert_eq!(dot(a, b), 20.0);
}

#[test]
fn it_computes_cross_product() {
  let a = vector(1., 2., 3.);
  let b = vector(2., 3., 4.);
  assert_eq!(cross(a, b).equals(vector(-1., 2., -1.)), true);
  assert_eq!(cross(b, a).equals(vector(1., -2., 1.)), true);
}

#[test]
fn reflect_a_45_deg_vector() {
  let v = vector(1., -1., 0.);
  let n = vector(0., 1., 0.);
  assert_eq!(reflect(v, n).equals(vector(1., 1., 0.)), true);
}

#[test]
fn reflect_a_45_deg_vector_rotated() {
  let v = vector(0., -1., 0.);
  let n = vector(2.0f64.sqrt() / 2., 2.0f64.sqrt() / 2., 0.);
  assert_eq!(reflect(v, n).equals(vector(1., 0., 0.)), true);
}
