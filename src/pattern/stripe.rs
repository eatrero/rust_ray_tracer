use crate::colors::Color;
use crate::matrix::Matrix;
use crate::pattern::{Pattern, PatternType};
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};
use crate::Transform;

#[derive(Clone)]
pub struct Stripe {
  a: Color,
  b: Color,
}

impl Stripe {
  pub fn new(color1: Color, color2: Color) -> Stripe {
    return Stripe {
      a: color1,
      b: color2,
    };
  }

  pub fn pattern_at(pattern: &Pattern, point: Tuple) -> Color {
    if point.x >= 0.0 {
      if point.x % 2. >= 1.0 {
        return pattern.b.clone();
      }
      return pattern.a.clone();
    } else {
      if point.x.abs() % 2. > 1.0 {
        return pattern.a.clone();
      }
      return pattern.b.clone();
    }
  }
}

#[test]
fn creating_a_stripe_pattern() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Stripe, white, black);
  assert_eq!(Color::equals(pattern.a, white.clone()), true);
  assert_eq!(Color::equals(pattern.b, black.clone()), true);
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Stripe, white, black);

  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 0., 0.)),
      white.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 1., 0.)),
      white.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 2., 0.)),
      white.clone()
    ),
    true
  );
}
#[test]
fn a_stripe_pattern_is_constant_in_z() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Stripe, white, black);

  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 0., 0.)),
      white.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 0., 1.)),
      white.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 0., 2.)),
      white.clone()
    ),
    true
  );
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Stripe, white, black);

  let x = Stripe::pattern_at(&pattern, point(-0.1, 0., 0.));

  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0., 0., 0.)),
      white.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(0.9, 0., 0.)),
      white.clone()
    ),
    true
  );

  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(1., 0., 0.)),
      black.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(-0.1, 0., 0.)),
      black.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(-1.0, 0., 0.)),
      black.clone()
    ),
    true
  );
  assert_eq!(
    Color::equals(
      Stripe::pattern_at(&pattern, point(-1.1, 1., 0.)),
      white.clone()
    ),
    true
  );
}
