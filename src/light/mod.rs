use crate::colors::Color;
use crate::material::Material;
use crate::shape::{Shape, ShapeType};
use crate::vectors::{dot, point, reflect, vector, Tuple};

#[derive(Copy, Clone)]
pub struct PointLight {
  pub position: Tuple,
  pub intensity: Color,
}

impl PointLight {
  pub fn new(position: Tuple, intensity: Color) -> PointLight {
    PointLight {
      position: position,
      intensity: intensity,
    }
  }
}

pub fn lighting(
  m: Material,
  o: Shape,
  l: PointLight,
  position: Tuple,
  eyev: Tuple,
  normalv: Tuple,
  is_in_shadow: bool,
) -> Color {
  let mut diffuse;
  let mut specular;

  let color = match m.pattern {
    Some(_) => m.pattern.unwrap().pattern_at_object(o, position),
    None => m.color,
  };

  let effective_color = Color::dot(color, l.intensity);
  let lightv = l.position.sub(position).norm();
  let ambient = Color::mult(effective_color, m.ambient);
  let light_dot_normal = dot(lightv, normalv);

  if light_dot_normal < 0. {
    diffuse = Color::new(0., 0., 0.);
    specular = Color::new(0., 0., 0.);
  } else {
    diffuse = Color::mult(effective_color, m.diffuse * light_dot_normal);

    let neg_lightv = lightv.negate();
    let reflectv = reflect(neg_lightv, normalv);
    let reflect_dot_eye = dot(reflectv, eyev);
    if reflect_dot_eye <= 0. {
      specular = Color::new(0., 0., 0.);
    } else {
      let factor = reflect_dot_eye.powf(m.shininess);
      specular = Color::mult(l.intensity, m.specular * factor);
    }
  }

  if (is_in_shadow) {
    return ambient;
  }

  return Color::add(Color::add(ambient, diffuse), specular);
}

#[test]
fn a_point_light_has_position_and_intensity() {
  let intensity = Color::new(1., 1., 1.);
  let position = point(0., 0., 0.);

  let light = PointLight::new(position, intensity);

  assert_eq!(Color::equals(light.intensity, intensity.clone()), true);
}

#[test]
fn lighting_with_eye_between_light_and_surface() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 0., -10.), intensity);
  let eyev = vector(0., 0., -1.);
  let normalv = vector(0., 0., -1.);
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, false);

  assert_eq!(Color::equals(light, Color::new(1.9, 1.9, 1.9)), true);
}

#[test]
fn lighting_with_eye_between_light_and_surface_eye_at_45() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 0., -10.), intensity);
  let eyev = vector(0., 2.0f64.sqrt() / 2., -2.0f64.sqrt() / 2.);
  let normalv = vector(0., 0., -1.);
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, false);

  assert_eq!(Color::equals(light, Color::new(1.0, 1.0, 1.0)), true);
}

#[test]
fn lighting_with_eye_between_light_and_surface_light_at_45() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 10., -10.), intensity);
  let eyev = vector(0., 0., -1.);
  let normalv = vector(0., 0., -1.);
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, false);

  assert_eq!(
    Color::approx_equals(light, Color::new(0.7364, 0.7364, 0.7364)),
    true
  );
}

#[test]
fn lighting_with_eye_in_path_of_reflection() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 10., -10.), intensity);
  let eyev = vector(0., -2.0f64.sqrt() / 2., -2.0f64.sqrt() / 2.);
  let normalv = vector(0., 0., -1.);
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, false);

  assert_eq!(
    Color::approx_equals(light, Color::new(1.6364, 1.6364, 1.6364)),
    true
  );
}

#[test]
fn lighting_with_light_behind_surface() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 0., 10.), intensity);
  let eyev = vector(0., 0., -1.);
  let normalv = vector(0., 0., -1.);
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, false);

  assert_eq!(Color::equals(light, Color::new(0.1, 0.1, 0.1)), true);
}

#[test]
fn lighting_with_with_the_surface_in_shadow() {
  let m = Material::new();
  let position = point(0., 0., 0.);
  let eyev = vector(0., 0., -1.);
  let normalv = vector(0., 0., -1.);
  let intensity = Color::new(1., 1., 1.);
  let l = PointLight::new(point(0., 0., -10.), intensity);
  let is_in_shadow = true;
  let o = Shape::new(ShapeType::Sphere);

  let light = lighting(m, o, l, position, eyev, normalv, is_in_shadow);

  assert_eq!(Color::equals(light, Color::new(0.1, 0.1, 0.1)), true);
}
