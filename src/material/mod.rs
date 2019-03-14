use crate::colors::Color;

#[derive(Copy, Clone)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
}

impl Material {
  pub fn new() -> Material {
    Material {
      color: Color::new(1., 1., 1.),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.,
    }
  }
}

#[test]
fn create_a_new_material() {
  let m = Material::new();

  assert_eq!(Color::equals(m.color, Color::new(1., 1., 1.,)), true);
}
