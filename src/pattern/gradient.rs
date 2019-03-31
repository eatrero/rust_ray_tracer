use crate::colors::Color;
use crate::matrix::Matrix;
use crate::pattern::{Pattern, PatternType};
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};
use crate::Transform;

#[derive(Clone)]
pub struct Gradient {
  a: Color,
  b: Color,
}

impl Gradient {
  pub fn new(color1: Color, color2: Color) -> Gradient {
    return Gradient {
      a: color1,
      b: color2,
    };
  }

  pub fn pattern_at(pattern: &Pattern, point: Tuple) -> Color {
    let distance = Color::sub(pattern.b, pattern.a);
    let fraction = point.x - point.x.floor();

    return Color::add(pattern.a, Color::mult(distance, fraction));
  }
}

#[test]
fn creating_a_gradient() {
  let black = Color::new(0., 0., 0.);
  let white = Color::new(1., 1., 1.);

  let pattern = Pattern::new(PatternType::Gradient, white, black);

  assert_eq!(
    Color::equals(pattern.pattern_at(point(0., 0., 0.,)), white.clone()),
    true
  );

  assert_eq!(
    Color::equals(
      pattern.pattern_at(point(0.25, 0., 0.,)),
      Color::new(0.75, 0.75, 0.75)
    ),
    true
  );

  assert_eq!(
    Color::equals(
      pattern.pattern_at(point(0.5, 0., 0.,)),
      Color::new(0.5, 0.5, 0.5)
    ),
    true
  );

  assert_eq!(
    Color::equals(
      pattern.pattern_at(point(0.75, 0., 0.,)),
      Color::new(0.25, 0.25, 0.25)
    ),
    true
  );
}
