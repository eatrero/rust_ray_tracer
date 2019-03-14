use crate::matrix::Matrix;
use crate::transform::Transform;
use crate::vectors::Tuple;
use crate::vectors::{point, vector};

#[derive(Copy, Clone)]
pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple,
}

impl Ray {
  pub fn new(origin: Tuple, direction: Tuple) -> Ray {
    Ray {
      origin: origin,
      direction: direction,
    }
  }

  pub fn position(&self, time: f64) -> Tuple {
    let delta = self.direction.mult(time);
    let p = self.origin.add(delta);
    return p;
  }

  pub fn transform(&self, transform: &Matrix) -> Ray {
    let ot = Matrix::mult_4x4_by_1d(&transform, &self.origin);
    let dt = Matrix::mult_4x4_by_1d(&transform, &self.direction);

    return Ray::new(ot, dt);
  }
}

#[test]
fn it_computes_position_over_t() {
  let origin = point(2., 3., 4.);
  let direction = vector(1., 0., 0.);
  let r = Ray::new(origin, direction);

  let p1 = r.position(0.);
  let e1 = point(2., 3., 4.);
  assert_eq!(p1.equals(e1), true);

  let p2 = r.position(1.);
  let e2 = point(3., 3., 4.);
  assert_eq!(p2.equals(e2), true);

  let p3 = r.position(-1.);
  let e3 = point(1., 3., 4.);
  assert_eq!(p3.equals(e3), true);

  let p4 = r.position(2.5);
  let e4 = point(4.5, 3., 4.);
  assert_eq!(p4.equals(e4), true);
}

#[test]
fn it_translates_a_ray() {
  let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
  let m = Transform::new().translate(3., 4., 5.).transform;
  let r2 = r.transform(&m);

  println!(
    "******** {} {} {} {}",
    r2.direction.x, r2.direction.y, r2.direction.z, r2.direction.w
  );
  assert_eq!(r2.origin.equals(point(4., 6., 8.)), true);
  assert_eq!(r2.direction.equals(vector(0., 1., 0.)), true);
}

#[test]
fn it_scales_a_ray() {
  let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
  let m = Transform::new().scale(2., 3., 4.).transform;
  let r2 = r.transform(&m);

  assert_eq!(r2.origin.equals(point(2., 6., 12.)), true);
  assert_eq!(r2.direction.equals(vector(0., 3., 0.)), true);
}
