use crate::colors::Color;
use crate::light::PointLight;
use crate::sphere::Sphere;
use crate::vectors::{point, Tuple};

#[derive(Copy, Clone)]
pub struct Proj {
  pub pos: Tuple, // point
  pub vel: Tuple, // vector
}

impl Proj {
  pub fn new(pos: Tuple, vel: Tuple) -> Proj {
    return Proj { pos: pos, vel: vel };
  }
}

#[derive(Copy, Clone)]
pub struct Env {
  wind: Tuple, // vector
  grav: Tuple, // vector
}

impl Env {
  pub fn new(wind: Tuple, grav: Tuple) -> Env {
    return Env {
      wind: wind,
      grav: grav,
    };
  }
}

pub fn tick(env: Env, proj: Proj) -> Proj {
  return Proj::new(proj.pos.add(proj.vel), proj.vel.add(env.grav).add(env.wind));
}

pub struct World {
  light: Option<PointLight>,
  objects: Option<Vec<Sphere>>,
}

impl World {
  pub fn new() -> World {
    World {
      light: None,
      objects: None,
    }
  }

  pub fn default_world() -> World {
    let s1 = Sphere::new(point(0., 0., 0.), 1.);
    let s2 = Sphere::new(point(0., 0., 0.), 1.);
    return World {
      light: Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
      )),
      objects: Some(vec![s1, s2]),
    };
  }
}

#[test]
fn new_world_contains_no_light_or_objects() {
  let world = World::new();

  assert_eq!(world.light.is_none(), true);
  assert_eq!(world.objects.is_none(), true);
}

#[test]
fn default_world_contains_no_light_or_objects() {
  let world = World::default_world();

  assert_eq!(world.light.is_some(), true);
  assert_eq!(world.objects.is_some(), true);

  assert_eq!(
    world.light.unwrap().position.equals(point(-10., 10., -10.)),
    true
  );
}
