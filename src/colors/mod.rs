#[derive(Copy, Clone)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
  }

  pub fn add(a: Color, b: Color) -> Color {
    return Color::new(a.r + b.r, a.g + b.g, a.b + b.b);
  }
  pub fn sub(a: Color, b: Color) -> Color {
    return Color::new(a.r - b.r, a.g - b.g, a.b - b.b);
  }
  pub fn negate(&self) -> Color {
    return Color::new(-self.r, -self.g, -self.b);
  }
  pub fn mult(c: Color, a: f64) -> Color {
    return Color::new(c.r * a, c.g * a, c.b * a);
  }
  pub fn dot(a: Color, b: Color) -> Color {
    return Color::new(a.r * b.r, a.g * b.g, a.b * b.b);
  }
  pub fn div(c: Color, a: f64) -> Color {
    return Color::new(c.r / a, c.g / a, c.b / a);
  }
  pub fn equals(a: Color, b: Color) -> bool {
    return (a.r - b.r).abs() < 1e-10 && (a.g - b.g).abs() < 1e-10 && (a.b - b.b).abs() < 1e-10;
  }
  pub fn approx_equals(a: Color, b: Color) -> bool {
    return (a.r - b.r).abs() < 1e-3 && (a.g - b.g).abs() < 1e-3 && (a.b - b.b).abs() < 1e-3;
  }
}

#[test]
fn it_adds_color() {
  let c1 = Color::new(0.9, 0.6, 0.75);
  let c2 = Color::new(0.7, 0.1, 0.25);
  let sum = Color::add(c1, c2);
  let expected = Color::new(1.6, 0.7, 1.0);
  assert_eq!(Color::equals(sum, expected), true);
}

#[test]
fn it_subs_color() {
  let c1 = Color::new(0.9, 0.6, 0.75);
  let c2 = Color::new(0.7, 0.1, 0.25);
  let sum = Color::sub(c1, c2);
  let expected = Color::new(0.2, 0.5, 0.5);
  assert_eq!(Color::equals(sum, expected), true);
}

#[test]
fn it_scales_a_color() {
  let c1 = Color::new(0.2, 0.3, 0.4);
  let product = Color::mult(c1, 2.0);
  let expected = Color::new(0.4, 0.6, 0.8);
  assert_eq!(Color::equals(product, expected), true);
}

#[test]

fn it_multiplies_two_colors() {
  let c1 = Color::new(1.0, 0.2, 0.4);
  let c2 = Color::new(0.9, 1.0, 0.1);
  let product = Color::dot(c1, c2);
  let expected = Color::new(0.9, 0.2, 0.04);
  assert_eq!(Color::equals(product, expected), true);
}
