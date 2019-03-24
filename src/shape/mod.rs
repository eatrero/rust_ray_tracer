use crate::intersections::Intersections;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vectors::Tuple;

pub mod sphere;

pub trait Shape {
  fn intersects(&self, ray: Ray) -> Intersections;
  fn set_transform(&mut self, transform: Matrix);
  fn normal_at(&self, p: Tuple) -> Tuple;
}
