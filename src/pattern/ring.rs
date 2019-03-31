use crate::colors::Color;
use crate::matrix::Matrix;
use crate::pattern::{Pattern, PatternType};
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};
use crate::Transform;

#[derive(Clone)]
pub struct Ring {
  a: Color,
  b: Color,
}

impl Ring {
  pub fn new(color1: Color, color2: Color) -> Ring {
    return Ring {
      a: color1,
      b: color2,
    };
  }

  pub fn pattern_at(pattern: &Pattern, point: Tuple) -> Color {
    if ((point.x * point.x + point.z * point.z).sqrt()).floor() % 2.0 == 0.0 {
      return pattern.a.clone();
    } else {
      return pattern.b.clone();
    }
  }
}

#[test]
fn creating_a_ring() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Ring, white, black);

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.,)), white.clone()),
    true
  );

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 1.,)), black.clone()),
    true
  );

  assert_eq!(
    Color::equals(pattern.pattern_at(point(1., 0., 0.,)), black.clone()),
    true
  );

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0.708, 0., 0.708,)), black.clone()),
    true
  );
}
