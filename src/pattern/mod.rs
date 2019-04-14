use crate::colors::Color;
use crate::matrix::Matrix;
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};
use crate::Transform;

pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;

#[derive(Clone)]
pub enum PatternType {
  Checker,
  Gradient,
  Ring,
  Stripe,
  Test,
}

#[derive(Clone)]
pub struct Pattern {
  pattern_type: PatternType,
  a: Color,
  b: Color,
  transform: Matrix,
}

impl Pattern {
  pub fn new(pattern_type: PatternType, color1: Color, color2: Color) -> Pattern {
    return Pattern {
      pattern_type: pattern_type,
      a: color1,
      b: color2,
      transform: Matrix::identity(4),
    };
  }

  pub fn pattern_at(&self, point: Tuple) -> Color {
    return match &self.pattern_type {
      PatternType::Checker => checker::Checker::pattern_at(self, point),
      PatternType::Gradient => gradient::Gradient::pattern_at(self, point),
      PatternType::Stripe => stripe::Stripe::pattern_at(self, point),
      PatternType::Ring => ring::Ring::pattern_at(self, point),
      PatternType::Test => {
        println!(
          "************* test pattern {} {} {}",
          point.x, point.y, point.z
        );
        return Color::new(point.x, point.y, point.z);
      }
    };
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    self.transform = transform;
  }

  pub fn pattern_at_object(&self, object: Shape, point: Tuple) -> Color {
    let i_object_tx = Matrix::inverse(&object.transform);
    let object_point = Matrix::mult_4x4_by_1d(&i_object_tx, &point);
    let i_pattern_tx = Matrix::inverse(&self.transform);
    let pattern_point = Matrix::mult_4x4_by_1d(&i_pattern_tx, &object_point);

    return self.pattern_at(pattern_point);
  }
}

#[test]
fn pattern_with_an_object_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform = Transform::new().scale(2., 2., 2.).transform;
  let pattern = Pattern::new(
    PatternType::Test,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );

  object.set_transform(transform);
  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(2., 3., 4.));

  assert_eq!(Color::equals(c, Color::new(1., 1.5, 2.)), true);
}

#[test]
fn pattern_with_a_pattern_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform = Transform::new().scale(2., 2., 2.).transform;
  let mut pattern = Pattern::new(
    PatternType::Test,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );

  pattern.set_transform(transform);
  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(2., 3., 4.));

  assert_eq!(Color::equals(c, Color::new(1., 1.5, 2.)), true);
}

#[test]
fn pattern_with_both_object_and_pattern_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform1 = Transform::new().translate(0.5, 1., 1.5).transform;
  let transform2 = Transform::new().scale(2., 2., 2.).transform;
  let mut pattern = Pattern::new(
    PatternType::Test,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );
  pattern.set_transform(transform1);

  object.set_transform(transform2);
  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(2.5, 3., 3.5));

  assert_eq!(Color::equals(c, Color::new(0.75, 0.5, 0.25)), true);
}

#[test]
fn stripes_with_an_object_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform = Transform::new().scale(2., 2., 2.).transform;
  let pattern = Pattern::new(
    PatternType::Stripe,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );

  object.set_transform(transform);
  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(1.5, 0., 0.));

  assert_eq!(Color::equals(c, Color::new(1., 1., 1.)), true);
}

#[test]
fn stripes_with_a_pattern_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform = Transform::new().scale(2., 2., 2.).transform;
  let mut pattern = Pattern::new(
    PatternType::Stripe,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );
  pattern.set_transform(transform);

  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(2.5, 0., 0.));

  assert_eq!(Color::equals(c, Color::new(0., 0., 0.)), true);
}

#[test]
fn stripes_with_both_object_and_pattern_transformation() {
  let mut object = Shape::new(ShapeType::Sphere);
  let transform1 = Transform::new().translate(0.5, 0., 0.).transform;
  let transform2 = Transform::new().scale(2., 2., 2.).transform;
  let mut pattern = Pattern::new(
    PatternType::Stripe,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  );
  pattern.set_transform(transform1);

  object.set_transform(transform2);
  object.material.set_pattern(pattern.clone());
  let c = pattern.pattern_at_object(object, point(2.5, 0., 0.));

  assert_eq!(Color::equals(c, Color::new(1., 1., 1.)), true);
}
