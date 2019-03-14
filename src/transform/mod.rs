use crate::matrix::Matrix;
use crate::vectors::Tuple;
use crate::vectors::{point, vector};
use std::f64;

pub struct Transform {
  pub transform: Matrix,
}

impl Transform {
  pub fn new() -> Transform {
    Transform {
      transform: Matrix::identity(4),
    }
  }

  pub fn translate(&self, x: f64, y: f64, z: f64) -> Transform {
    let data = vec![1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.];
    let translate = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &translate),
    };
  }

  pub fn scale(&self, x: f64, y: f64, z: f64) -> Transform {
    let data = vec![x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.];
    let scale = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &scale),
    };
  }

  pub fn rotate_x(&self, rot: f64) -> Transform {
    let data = vec![
      1.,
      0.,
      0.,
      0.,
      0.,
      rot.cos(),
      -rot.sin(),
      0.,
      0.,
      rot.sin(),
      rot.cos(),
      0.,
      0.,
      0.,
      0.,
      1.,
    ];
    let rotate_x = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &rotate_x),
    };
  }

  pub fn rotate_y(&self, rot: f64) -> Transform {
    let data = vec![
      rot.cos(),
      0.,
      rot.sin(),
      0.,
      0.,
      1.,
      0.,
      0.,
      -rot.sin(),
      0.,
      rot.cos(),
      0.,
      0.,
      0.,
      0.,
      1.,
    ];
    let rotate_y = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &rotate_y),
    };
  }

  pub fn rotate_z(&self, rot: f64) -> Transform {
    let data = vec![
      rot.cos(),
      -rot.sin(),
      0.,
      0.,
      rot.sin(),
      rot.cos(),
      0.,
      0.,
      0.,
      0.,
      1.,
      0.,
      0.,
      0.,
      0.,
      1.,
    ];
    let rotate_z = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &rotate_z),
    };
  }

  pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transform {
    let data = vec![
      1.0, xy, xz, 0., yx, 1., yz, 0., zx, zy, 1., 0., 0., 0., 0., 1.,
    ];
    let rotate_z = Matrix::new(4, 4, data);
    return Transform {
      transform: Matrix::mult(&self.transform, &rotate_z),
    };
  }
}

#[test]
fn it_translates() {
  let p = point(-3., 4., 5.);
  let transform = Transform::new().translate(5., -3., 2.).transform;
  let pt = Matrix::mult_4x4_by_1d(&transform, &p);
  let expected = point(2., 1., 7.);

  assert_eq!(pt.equals(expected), true);
}

#[test]
fn it_translates_inverse() {
  let p = point(-3., 4., 5.);
  let transform = Transform::new().translate(5., -3., 2.).transform;
  let inv_transform = Matrix::inverse(&transform);
  let pt = Matrix::mult_4x4_by_1d(&inv_transform, &p);
  let expected = point(-8., 7., 3.);

  assert_eq!(pt.equals(expected), true);
}

#[test]
fn it_translates_inverse_vector() {
  let v = vector(-3., 4., 5.);
  let transform = Transform::new().translate(5., -3., 2.).transform;
  let vt = Matrix::mult_4x4_by_1d(&transform, &v);
  let expected = vector(-3., 4., 5.);

  assert_eq!(vt.equals(expected), true);
}

#[test]
fn it_scales_point() {
  let transform = Transform::new().scale(2., 3., 4.).transform;
  let p = point(-4., 6., 8.);
  let pt = Matrix::mult_4x4_by_1d(&transform, &p);
  let expected = point(-8., 18., 32.);

  assert_eq!(pt.equals(expected), true);
}

#[test]
fn it_scales_vector() {
  let transform = Transform::new().scale(2., 3., 4.).transform;
  let v = vector(-4., 6., 8.);
  let vt = Matrix::mult_4x4_by_1d(&transform, &v);
  let expected = vector(-8., 18., 32.);

  assert_eq!(vt.equals(expected), true);
}

#[test]
fn it_scales_inverse_vector() {
  let transform = Transform::new().scale(2., 3., 4.).transform;
  let inv_transform = Matrix::inverse(&transform);
  let v = vector(-4., 6., 8.);
  let vt = Matrix::mult_4x4_by_1d(&inv_transform, &v);
  let expected = vector(-2., 2., 2.);

  assert_eq!(vt.equals(expected), true);
}
#[test]
fn it_scales_reflects() {
  let transform = Transform::new().scale(-1., 1., 1.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&transform, &p);
  let expected = point(-2., 3., 4.);

  assert_eq!(pt.equals(expected), true);
}

#[test]
fn it_rotates_a_point_around_x_axis() {
  let rotate_90 = Transform::new().rotate_x(f64::consts::PI / 4.).transform;
  let rotate_180 = Transform::new().rotate_x(f64::consts::PI / 2.).transform;
  let p = point(0., 1., 0.);
  let pt1 = Matrix::mult_4x4_by_1d(&rotate_90, &p);
  let ex1 = point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.);
  assert_eq!(pt1.equals(ex1), true);

  let pt2 = Matrix::mult_4x4_by_1d(&rotate_180, &p);
  let ex2 = point(0., 0., 1.);
  assert_eq!(pt2.equals(ex2), true);
}
#[test]
fn it_rotates_a_point_around_x_axis_opposite_direction() {
  let rotate_90 = Transform::new().rotate_x(-f64::consts::PI / 4.).transform;
  let p = point(0., 1., 0.);
  let pt1 = Matrix::mult_4x4_by_1d(&rotate_90, &p);
  let ex1 = point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
  assert_eq!(pt1.equals(ex1), true);
}

#[test]
fn it_rotates_a_point_around_y_axis() {
  let rotate_90 = Transform::new().rotate_y(f64::consts::PI / 4.).transform;
  let rotate_180 = Transform::new().rotate_y(f64::consts::PI / 2.).transform;
  let p = point(0., 0., 1.);
  let pt = Matrix::mult_4x4_by_1d(&rotate_90, &p);
  let ex = point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.);
  assert_eq!(pt.equals(ex), true);

  let pt2 = Matrix::mult_4x4_by_1d(&rotate_180, &p);
  let ex2 = point(1., 0., 0.);
  assert_eq!(pt2.equals(ex2), true);
}

#[test]
fn it_rotates_a_point_around_z_axis() {
  let rotate_90 = Transform::new().rotate_z(f64::consts::PI / 4.).transform;
  let rotate_180 = Transform::new().rotate_z(f64::consts::PI / 2.).transform;
  let p = point(0., 1., 0.);
  let pt = Matrix::mult_4x4_by_1d(&rotate_90, &p);
  let ex = point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.);
  assert_eq!(pt.equals(ex), true);

  let pt2 = Matrix::mult_4x4_by_1d(&rotate_180, &p);
  let ex2 = point(-1., 0., 0.);
  assert_eq!(pt2.equals(ex2), true);
}

#[test]
fn it_shears_along_x_in_proportion_to_y() {
  let shear = Transform::new().shear(1., 0., 0., 0., 0., 0.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(5., 3., 4.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_shears_along_x_in_proportion_to_z() {
  let shear = Transform::new().shear(0., 1., 0., 0., 0., 0.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(6., 3., 4.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_shears_along_y_in_proportion_to_x() {
  let shear = Transform::new().shear(0., 0., 1., 0., 0., 0.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(2., 5., 4.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_shears_along_y_in_proportion_to_z() {
  let shear = Transform::new().shear(0., 0., 0., 1., 0., 0.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(2., 7., 4.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_shears_along_z_in_proportion_to_x() {
  let shear = Transform::new().shear(0., 0., 0., 0., 1., 0.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(2., 3., 6.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_shears_along_z_in_proportion_to_y() {
  let shear = Transform::new().shear(0., 0., 0., 0., 0., 1.).transform;
  let p = point(2., 3., 4.);
  let pt = Matrix::mult_4x4_by_1d(&shear, &p);
  let ex = point(2., 3., 7.);
  assert_eq!(pt.equals(ex), true);
}

#[test]
fn it_individual_transforms_work_sequentially() {
  let p = point(1., 0., 1.);

  let p1 = Matrix::mult_4x4_by_1d(
    &Transform::new().rotate_x(f64::consts::PI / 2.).transform,
    &p,
  );
  let ex1 = point(1., -1., 0.);
  assert_eq!(p1.equals(ex1), true);

  let p2 = Matrix::mult_4x4_by_1d(&Transform::new().scale(5., 5., 5.).transform, &p1);
  let ex2 = point(5., -5., 0.);
  assert_eq!(p2.equals(ex2), true);

  let p3 = Matrix::mult_4x4_by_1d(&Transform::new().translate(10., 5., 7.).transform, &p2);
  let ex3 = point(15., 0., 7.);
  assert_eq!(p3.equals(ex3), true);
}

#[test]
fn it_chains_transformations_in_reverse_order() {
  let p = point(1., 0., 1.);
  let transforms = Transform::new()
    .translate(10., 5., 7.)
    .scale(5., 5., 5.)
    .rotate_x(f64::consts::PI / 2.)
    .transform;
  let pt = Matrix::mult_4x4_by_1d(&transforms, &p);
  let ex = point(15., 0., 7.);
  assert_eq!(pt.equals(ex), true);
}
