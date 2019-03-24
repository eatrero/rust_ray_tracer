use crate::colors::Color;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::transform::Transform;
use crate::vectors::{dot, point, vector, Tuple};
use rand::Rng;
use std::f64;

#[derive(Clone)]
pub struct Plane {
  pub handle: u32,
  pub transform: Matrix,
  pub material: Material,
}

impl Plane {
  pub fn new() -> Plane {
    let mut rng = rand::thread_rng();
    return Plane {
      handle: rng.gen::<u32>(),
      transform: Matrix::identity(4),
      material: Material::new(),
    };
  }

  pub fn intersects(object: &Shape, ray: Ray) -> Intersections {
    if (ray.direction.y.abs() < 1e-10) {
      return Intersections {
        intersections: vec![],
      };
    } else {
      let t = -ray.origin.y / ray.direction.y;
      return Intersections {
        intersections: vec![Intersection::new(t, object.clone())],
      };
    }
  }

  pub fn set_transform(object: &mut Shape, transform: Matrix) {
    object.transform = transform;
  }

  pub fn normal_at(object: &Shape, local_point: Tuple) -> Tuple {
    return vector(0., 1., 0.);
  }
}

#[test]
fn the_normal_of_a_plane_is_constant_everywhere() {}
