use crate::colors::Color;
use crate::light::{lighting, PointLight};
use crate::pattern::{Pattern, PatternType};
use crate::shape::{Shape, ShapeType};
use crate::vectors::{point, vector, Tuple};

#[derive(Clone)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
  pub reflectiveness: f64,
  pub transparency: f64,
  pub refractive_index: f64,
  pub pattern: Option<Pattern>,
}

impl Material {
  pub fn new() -> Material {
    Material {
      color: Color::new(1., 1., 1.),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.,
      reflectiveness: 0.0,
      transparency: 0.0,
      refractive_index: 1.0,
      pattern: None,
    }
  }

  pub fn set_pattern(&mut self, pattern: Pattern) {
    self.pattern = Some(pattern);
  }
}

#[test]
fn create_a_new_material() {
  let m = Material::new();

  assert_eq!(Color::equals(m.color, Color::new(1., 1., 1.,)), true);
  assert_eq!(m.reflectiveness, 0.0);
  assert_eq!(m.transparency, 0.0);
  assert_eq!(m.refractive_index, 1.0);
}

#[test]
fn lighting_an_applied_pattern() {
  let mut m = Material::new();
  m.set_pattern(Pattern::new(
    PatternType::Stripe,
    Color::new(1., 1., 1.),
    Color::new(0., 0., 0.),
  ));
  m.ambient = 1.0;
  m.diffuse = 0.0;
  m.specular = 0.0;

  let eyev = vector(0., 0., -1.);
  let normalv = vector(0., 0., -1.);
  let light = PointLight::new(point(0., 0., -10.), Color::new(1., 1., 1.));

  let mut o = Shape::new(ShapeType::Sphere);
  o.material = m.clone();

  let c1 = lighting(
    o.material.clone(),
    o.clone(),
    light,
    point(0.9, 0., 0.),
    eyev,
    normalv,
    false,
  );
  let c2 = lighting(
    o.material.clone(),
    o.clone(),
    light,
    point(1.5, 0., 0.),
    eyev,
    normalv,
    false,
  );

  assert_eq!(Color::equals(c1, Color::new(1., 1., 1.)), true);
  assert_eq!(Color::equals(c2, Color::new(0., 0., 0.)), true);
}
