use crate::colors::Color;
use crate::intersections::{prepare_computations, Intersection, Intersections};
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

  pub fn glass_sphere() -> Shape {
    let mut sphere = Shape::new(ShapeType::Sphere);
    sphere.material.transparency = 1.0;
    sphere.material.refractive_index = 1.5;

    return sphere;
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

impl PartialEq for Shape {
  fn eq(&self, other: &Shape) -> bool {
    self.handle == other.handle
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

#[test]
fn creates_a_glass_sphere() {
  let s = Shape::glass_sphere();

  assert_eq!(Matrix::equals(&s.transform, &Matrix::identity(4)), true);
  assert_eq!(s.material.transparency, 1.0);
  assert_eq!(s.material.refractive_index, 1.5);
}

#[test]
fn finding_n1_and_n2_and_various_intersections() {
  let mut a = Shape::glass_sphere();
  let a_transform = Transform::new().scale(2., 2., 2.).transform;
  a.set_transform(a_transform);
  a.material.refractive_index = 1.5;

  let mut b = Shape::glass_sphere();
  let b_transform = Transform::new().translate(0., 0., -0.25).transform;
  b.set_transform(b_transform);
  b.material.refractive_index = 2.0;

  let mut c = Shape::glass_sphere();
  let c_transform = Transform::new().translate(0., 0., 0.25).transform;
  c.set_transform(c_transform);
  c.material.refractive_index = 2.5;

  let r = Ray::new(point(0., 0., -4.), vector(0., 0., 1.));
  let xs = Intersections::new(vec![
    Intersection::new(2., a.clone()),
    Intersection::new(2.75, b.clone()),
    Intersection::new(3.25, c.clone()),
    Intersection::new(4.75, b.clone()),
    Intersection::new(5.25, c.clone()),
    Intersection::new(6., a.clone()),
  ]);

  let comp1 = prepare_computations(xs.intersections[0].clone(), r, xs.clone());
  assert_eq!(comp1.n1, 1.0);
  assert_eq!(comp1.n2, 1.5);

  let comp2 = prepare_computations(xs.intersections[1].clone(), r, xs.clone());
  assert_eq!(comp2.n1, 1.5);
  assert_eq!(comp2.n2, 2.0);

  let comp3 = prepare_computations(xs.intersections[2].clone(), r, xs.clone());
  assert_eq!(comp3.n1, 2.0);
  assert_eq!(comp3.n2, 2.5);

  let comp4 = prepare_computations(xs.intersections[3].clone(), r, xs.clone());
  assert_eq!(comp4.n1, 2.5);
  assert_eq!(comp4.n2, 2.5);

  let comp5 = prepare_computations(xs.intersections[4].clone(), r, xs.clone());
  assert_eq!(comp5.n1, 2.5);
  assert_eq!(comp5.n2, 1.5);

  let comp6 = prepare_computations(xs.intersections[5].clone(), r, xs.clone());
  assert_eq!(comp6.n1, 1.5);
  assert_eq!(comp6.n2, 1.0);
}
