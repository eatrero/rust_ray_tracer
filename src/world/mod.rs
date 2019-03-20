use crate::colors::Color;
use crate::intersections::{prepare_computations, Computations, Intersection, Intersections};
use crate::light::{lighting, PointLight};
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
  objects: Vec<Sphere>,
}

impl World {
  pub fn new() -> World {
    World {
      light: None,
      objects: vec![],
    }
  }

  pub fn set_light(&mut self, point_light: PointLight) {
    self.light = Some(point_light);
  }

  pub fn add_object(&mut self, object: Sphere) {
    self.objects.push(object);
  }

  pub fn default_world() -> World {
    let mut s1 = Sphere::new(point(0., 0., 0.), 1.);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Sphere::new(point(0., 0., 0.), 1.);
    let transform2 = Transform::new().scale(0.5, 0.5, 0.5).transform;
    s2.set_transform(transform2);

    return World {
      light: Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
      )),
      objects: vec![s1, s2],
    };
  }

  pub fn default_world2() -> World {
    let mut s1 = Sphere::new(point(0., 0., 0.), 1.);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.ambient = 1.0;

    let mut s2 = Sphere::new(point(0., 0., 0.), 1.);
    let transform2 = Transform::new().scale(0.5, 0.5, 0.5).transform;
    s2.set_transform(transform2);
    s2.material.ambient = 1.0;

    return World {
      light: Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
      )),
      objects: vec![s1, s2],
    };
  }

  pub fn intersect_world(&self, r: Ray) -> Intersections {
    let objects = self.objects.clone();

    let mut _intersections: Vec<Intersection> = objects
      .clone()
      .iter()
      .map(|o| {
        return o.intersects(r).intersections;
      })
      .flatten()
      .collect::<Vec<Intersection>>();
    _intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    return Intersections::new(_intersections);
  }

  pub fn shade_hit(&self, comps: Computations) -> Color {
    return lighting(
      comps.object.material,
      self.light.unwrap(),
      comps.point,
      comps.eyev,
      comps.normalv,
    );
  }

  pub fn color_at(&self, r: Ray) -> Color {
    let xs = self.intersect_world(r);
    if xs.intersections.len() > 0 {
      let hit = xs.hit();
      if hit.intersections.len() > 0 {
        for h in hit.intersections {
          if h.t >= 0. {
            let intersect = h.clone();
            let comps = prepare_computations(intersect, r);
            return self.shade_hit(comps);
          }
        }
      }
    }
    return Color::new(0., 0., 0.);
  }
}

#[test]
fn new_world_contains_no_light_or_objects() {
  let world = World::new();

  assert_eq!(world.light.is_none(), true);
  assert_eq!(world.objects.len(), 0);
}

#[test]
fn default_world_contains_some_light_or_objects() {
  let world = World::default_world();

  assert_eq!(world.light.is_some(), true);
  assert_eq!(world.objects.len() > 0, true);

  assert_eq!(
    world.light.unwrap().position.equals(point(-10., 10., -10.)),
    true
  );
}

#[test]
fn ray_along_z_axis_intersects_default_world() {
  let world = World::default_world();

  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let xs = world.intersect_world(r).intersections;

  assert_eq!(xs.len(), 4);
  assert_eq!(xs[0].t, 4.0);
  assert_eq!(xs[1].t, 4.5);
  assert_eq!(xs[2].t, 5.5);
  assert_eq!(xs[3].t, 6.0);
}

#[test]
fn shading_an_intersection() {
  let world = World::default_world();
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let xs = world.intersect_world(r).intersections;
  let shape = xs[0].object.clone();
  let i = Intersection::new(4., shape);
  let comps = prepare_computations(i, r);
  let c = world.shade_hit(comps);

  assert_eq!(
    Color::approx_equals(c, Color::new(0.38066, 0.47583, 0.2855)),
    true
  );
}

#[test]
fn shading_an_intersection_from_inside() {
  let mut world = World::default_world();
  world.light = Some(PointLight::new(point(0., 0.25, 0.), Color::new(1., 1., 1.)));
  let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
  let shape = world.objects.clone()[1].clone();
  let i = Intersection::new(0.5, shape);
  let comps = prepare_computations(i, r);
  let c = world.shade_hit(comps);

  assert_eq!(
    Color::approx_equals(c, Color::new(0.90498, 0.90498, 0.90498)),
    true
  );
}

#[test]
fn the_color_when_a_ray_misses() {
  let world = World::default_world();
  let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
  let c = world.color_at(r);

  assert_eq!(Color::approx_equals(c, Color::new(0., 0., 0.)), true);
}

#[test]
fn the_color_when_a_ray_hits() {
  let world = World::default_world();
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let c = world.color_at(r);

  assert_eq!(
    Color::approx_equals(c, Color::new(0.38066, 0.47583, 0.2855)),
    true
  );
}

#[test]
fn the_color_with_an_intersection_from_behind() {
  let mut world = World::default_world2();

  let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
  let c = world.color_at(r);

  assert_eq!(Color::approx_equals(c, Color::new(1.0, 1.0, 1.0)), true);
}
