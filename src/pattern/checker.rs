use crate::colors::Color;
use crate::matrix::Matrix;
use crate::pattern::{Pattern, PatternType};
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};
use crate::Transform;

#[derive(Clone)]
pub struct Checker {
  a: Color,
  b: Color,
}

impl Checker {
  pub fn new(color1: Color, color2: Color) -> Checker {
    return Checker {
      a: color1,
      b: color2,
    };
  }

  pub fn pattern_at(pattern: &Pattern, point: Tuple) -> Color {
    /*
    println!(
      "{} {} {} {} {}",
      point.x.round(),
      point.y.round(),
      point.z.round(),
      point.x.round().floor() + point.y.round().floor() + point.z.round().floor(),
      (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0f64
    );
    */

    if ((point.x.round().floor() + point.y.round().floor() + point.z.round().floor()) % 2.0f64)
      .abs()
      < 1e-9
    {
      return pattern.a.clone();
    } else {
      return pattern.b.clone();
    }
  }
}

#[test]
fn checker_should_repeat_in_x() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Checker, white, black);

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(0.99, 0., 0.,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(1.01, 0., 0.,)), black.clone()),
    true
  );
}
#[test]
fn checker_should_repeat_in_y() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Checker, white, black);

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0.99, 0.,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(0.0, 1.01, 0.,)), black.clone()),
    true
  );
}
#[test]
fn checker_should_repeat_in_z() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Checker, white, black);

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.99,)), white.clone()),
    true
  );
  assert_eq!(
    Color::equals(pattern.pattern_at(point(0.0, 0., 1.01,)), black.clone()),
    true
  );
}
