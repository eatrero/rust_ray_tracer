use crate::colors::Color;
use crate::intersections::{Intersection, Intersections};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::vectors::{point, vector, Tuple};

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
    let mut s1 = Sphere::new(point(0., 0., 0.), 1.);
    let transform1 = Transform::new().scale(0.5, 0.5, 0.5).transform;
    s1.set_transform(transform1);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let s2 = Sphere::new(point(0., 0., 0.), 1.);

    return World {
      light: Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
      )),
      objects: Some(vec![s1, s2]),
    };
  }

  pub fn intersect_world(&self, r: Ray) -> Option<Vec<Intersection>> {
    let objects = self.objects.clone().unwrap_or(vec![]);
    if objects.is_empty() {
      return None;
    }

    let mut _intersections: Vec<Intersection> = objects
      .clone()
      .iter()
      .map(|o| {
        return o.intersects(r).intersections;
      })
      .flatten()
      .collect::<Vec<Intersection>>();
    _intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    return Some(_intersections);
  }
}

#[test]
fn new_world_contains_no_light_or_objects() {
  let world = World::new();

  assert_eq!(world.light.is_none(), true);
  assert_eq!(world.objects.is_none(), true);
}

#[test]
fn default_world_contains_some_light_or_objects() {
  let world = World::default_world();

  assert_eq!(world.light.is_some(), true);
  assert_eq!(world.objects.is_some(), true);

  assert_eq!(
    world.light.unwrap().position.equals(point(-10., 10., -10.)),
    true
  );
}

#[test]
fn ray_along_z_axis_intersects_default_world() {
  let world = World::default_world();

  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let xs = world.intersect_world(r).unwrap();

  assert_eq!(xs.len(), 4);
  assert_eq!(xs[0].t, 4.0);
  assert_eq!(xs[1].t, 4.5);
  assert_eq!(xs[2].t, 5.5);
  assert_eq!(xs[3].t, 6.0);
}
