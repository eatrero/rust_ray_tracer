use crate::colors::Color;
use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vectors::{dot, point, vector, Tuple};
use rand::Rng;
use std::f64;

pub mod plane;
pub mod sphere;

#[derive(Clone)]
pub enum ShapeType {
  Sphere,
  Plane,
  Test,
}

#[derive(Clone)]
pub struct Shape {
  pub shape_type: ShapeType,
  pub origin: Tuple,
  pub handle: u32,
  pub transform: Matrix,
  pub material: Material,
}

impl Shape {
  pub fn new(shape_type: ShapeType) -> Shape {
    let mut rng = rand::thread_rng();
    return Shape {
      shape_type: shape_type,
      origin: point(0., 0., 0.),
      handle: rng.gen::<u32>(),
      transform: Matrix::identity(4),
      material: Material::new(),
    };
  }

  pub fn intersects(&self, ray: Ray) -> Intersections {
    let i = Matrix::inverse(&self.transform);
    let local_ray = ray.transform(&i);

    return match &self.shape_type {
      ShapeType::Sphere => sphere::Sphere::intersects(self, local_ray),
      ShapeType::Plane => plane::Plane::intersects(self, local_ray),
      ShapeType::Test => Intersections::new(vec![]),
    };
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    return match &self.shape_type {
      ShapeType::Sphere => sphere::Sphere::set_transform(self, transform),
      ShapeType::Plane => plane::Plane::set_transform(self, transform),
      ShapeType::Test => self.transform = transform,
    };
  }

  pub fn normal_at(&self, p: Tuple) -> Tuple {
    let inverse_transform = Matrix::inverse(&self.transform);
    let object_point = Matrix::mult_4x4_by_1d(&inverse_transform, &p);
    let object_normal = match &self.shape_type {
      ShapeType::Sphere => sphere::Sphere::normal_at(self, object_point),
      ShapeType::Plane => plane::Plane::normal_at(self, object_point),
      ShapeType::Test => vector(1., 1., 1.).norm(),
    };

    let transposed_inverse_transform = Matrix::transpose(&inverse_transform);
    let mut world_normal = Matrix::mult_4x4_by_1d(&transposed_inverse_transform, &object_normal);
    world_normal.w = 0.;
    return world_normal.norm();
  }
}

#[test]
fn the_default_transformation() {
  let s = Shape::new(ShapeType::Test);

  assert_eq!(Matrix::equals(&s.transform, &Matrix::identity(4)), true);
}

#[test]
fn assigns_a_transformation() {
  let mut s = Shape::new(ShapeType::Test);
  let transform = Transform::new().translate(2., 3., 4.).transform;
  let expected = transform.clone();
  s.set_transform(transform);

  assert_eq!(Matrix::equals(&s.transform, &expected), true);
}

#[test]
fn assigns_a_material() {
  let mut s = Shape::new(ShapeType::Test);

  s.material = Material::new();
  s.material.ambient = 1.0;

  assert_eq!(s.material.ambient, 1.0);
}
