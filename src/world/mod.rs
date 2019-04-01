use crate::colors::Color;
use crate::intersections::{prepare_computations, Computations, Intersection, Intersections};
use crate::light::{lighting, PointLight};
use crate::ray::Ray;
use crate::shape::sphere::Sphere;
use crate::shape::{Shape, ShapeType};
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

#[derive(Clone)]
pub struct World {
  light: Option<PointLight>,
  objects: Vec<Shape>,
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

  pub fn add_object(&mut self, object: Shape) {
    self.objects.push(object);
  }

  pub fn default_world() -> World {
    let mut s1 = Shape::new(ShapeType::Sphere);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Shape::new(ShapeType::Sphere);
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
    let mut s1 = Shape::new(ShapeType::Sphere);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.ambient = 1.0;

    let mut s2 = Shape::new(ShapeType::Sphere);
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

  pub fn shade_hit(&self, comps: Computations, remaining: u32) -> Color {
    let is_in_shadow = self.is_shadowed(comps.over_point);

    let surface = lighting(
      comps.object.material.clone(),
      comps.object.clone(),
      self.light.unwrap(),
      comps.point,
      comps.eyev,
      comps.normalv,
      is_in_shadow,
    );

    let reflected = self.reflected_color(comps, remaining);
    return Color::add(surface, reflected);
  }

  pub fn color_at(&self, r: Ray, remaining: u32) -> Color {
    let xs = self.intersect_world(r);
    if xs.intersections.len() > 0 {
      let hit = xs.hit();
      if hit.intersections.len() > 0 {
        for h in hit.intersections {
          if h.t >= 0. {
            let intersect = h.clone();
            let comps = prepare_computations(intersect, r);
            return self.shade_hit(comps, remaining);
          }
        }
      }
    }
    return Color::new(0., 0., 0.);
  }

  pub fn reflected_color(&self, comps: Computations, remaining: u32) -> Color {
    if remaining == 0 || comps.object.material.reflectiveness == 0.0 {
      return Color::new(0., 0., 0.);
    }

    let reflected_ray = Ray::new(comps.over_point, comps.reflectv);
    let color = self.color_at(reflected_ray, remaining - 1);

    return Color::mult(color, comps.object.material.reflectiveness);
  }

  pub fn is_shadowed(&self, point: Tuple) -> bool {
    let v = self.light.unwrap().position.sub(point);
    let distance = v.mag();
    let direction = v.norm();

    let r = Ray::new(point, direction);
    let intersections = self.intersect_world(r);

    let h = intersections.hit();
    if h.intersections.len() > 0 && h.intersections[0].t < distance {
      return true;
    }
    return false;
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
  let c = world.shade_hit(comps, 1);

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
  let c = world.shade_hit(comps, 1);

  assert_eq!(
    Color::approx_equals(c, Color::new(0.90498, 0.90498, 0.90498)),
    true
  );
}

#[test]
fn the_color_when_a_ray_misses() {
  let world = World::default_world();
  let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
  let c = world.color_at(r, 1);

  assert_eq!(Color::approx_equals(c, Color::new(0., 0., 0.)), true);
}

#[test]
fn the_color_when_a_ray_hits() {
  let world = World::default_world();
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let c = world.color_at(r, 1);

  assert_eq!(
    Color::approx_equals(c, Color::new(0.38066, 0.47583, 0.2855)),
    true
  );
}

#[test]
fn the_color_with_an_intersection_from_behind() {
  let mut world = World::default_world2();

  let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
  let c = world.color_at(r, 1);

  assert_eq!(Color::approx_equals(c, Color::new(1.0, 1.0, 1.0)), true);
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
  let mut world = World::default_world();
  let point = point(0., 110., 0.);
  let is_in_shadow = world.is_shadowed(point);

  assert_eq!(is_in_shadow, false);
}

#[test]
fn there_is_a_shadow_when_an_object_is_between_the_point_and_the_light() {
  let mut world = World::default_world();
  let point = point(10., -10., 10.);
  let is_in_shadow = world.is_shadowed(point);

  assert_eq!(is_in_shadow, true);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_light() {
  let mut world = World::default_world();
  let point = point(-20., 20., -20.);
  let is_in_shadow = world.is_shadowed(point);

  assert_eq!(is_in_shadow, false);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {
  let mut world = World::default_world();
  let point = point(-2., 2., -2.);
  let is_in_shadow = world.is_shadowed(point);

  assert_eq!(is_in_shadow, false);
}

#[test]
fn the_reflected_color_of_a_non_reflective_material() {
  let mut world = World::default_world();
  let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
  let mut shape = world.objects[1].clone();
  shape.material.ambient = 1.0;
  let i = Intersection::new(1., shape);

  let comps = prepare_computations(i, r);
  let color = world.reflected_color(comps, 1);

  //println!("{:#?}", color);
  assert_eq!(Color::equals(color, Color::new(0., 0., 0.)), true);
}

#[test]
fn the_reflected_color_of_a_reflective_material() {
  let half_root2 = 2.0f64.sqrt() / 2.0;
  let mut world = World::default_world();
  let mut plane = Shape::new(ShapeType::Plane);
  plane.material.reflectiveness = 0.5;
  let tx = Transform::new().translate(0., -1.0, 0.).transform;
  plane.set_transform(tx);
  world.add_object(plane.clone());

  // 45 degree angle ray
  let r = Ray::new(point(0., 0., -3.), vector(0., -half_root2, half_root2));
  let i = Intersection::new(2.0f64.sqrt(), plane);
  let comps = prepare_computations(i, r);
  let color = world.reflected_color(comps, 1);

  //println!("{:#?}", color);
  assert_eq!(
    Color::approx_equals(color, Color::new(0.19032, 0.2379, 0.14274)),
    true
  );
}
#[test]
fn shade_hit_of_a_reflective_material() {
  let half_root2 = 2.0f64.sqrt() / 2.0;
  let mut world = World::default_world();
  let mut plane = Shape::new(ShapeType::Plane);
  plane.material.reflectiveness = 0.5;
  let tx = Transform::new().translate(0., -1.0, 0.).transform;
  plane.set_transform(tx);
  world.add_object(plane.clone());

  // 45 degree angle ray
  let r = Ray::new(point(0., 0., -3.), vector(0., -half_root2, half_root2));
  let i = Intersection::new(2.0f64.sqrt(), plane);
  let comps = prepare_computations(i, r);
  let color = world.shade_hit(comps, 1);

  println!("{:#?}", color);
  assert_eq!(
    Color::approx_equals(color, Color::new(0.87677, 0.92436, 0.82918)),
    true
  );
}

#[test]
fn color_at_with_mutually_reflective_surfaces() {
  let mut world = World::new();
  world.set_light(PointLight::new(point(0., 0., 0.), Color::new(1., 1., 1.)));

  let mut lower_plane = Shape::new(ShapeType::Plane);
  lower_plane.material.reflectiveness = 1.0;
  let lower_tx = Transform::new().translate(0., -1., 0.).transform;
  lower_plane.set_transform(lower_tx);
  world.add_object(lower_plane);

  let mut upper_plane = Shape::new(ShapeType::Plane);
  upper_plane.material.reflectiveness = 1.0;
  let upper_tx = Transform::new().translate(0., 1., 0.).transform;
  upper_plane.set_transform(upper_tx);
  world.add_object(upper_plane);

  let r = Ray::new(point(0., 0., 0.), vector(0., 1., 0.));
  let color = world.color_at(r, 4);

  println!("{:#?}", color);
}
#[test]
fn the_reflected_color_at_maximum_recursion_depth() {
  let half_root2 = 2.0f64.sqrt() / 2.0;
  let mut world = World::default_world();
  let mut plane = Shape::new(ShapeType::Plane);
  plane.material.reflectiveness = 0.5;
  let tx = Transform::new().translate(0., -1.0, 0.).transform;
  plane.set_transform(tx);
  world.add_object(plane.clone());

  // 45 degree angle ray
  let r = Ray::new(point(0., 0., -3.), vector(0., -half_root2, half_root2));
  let i = Intersection::new(2.0f64.sqrt(), plane);
  let comps = prepare_computations(i, r);
  let color = world.reflected_color(comps, 0);

  println!("{:#?}", color);
  assert_eq!(Color::approx_equals(color, Color::new(0., 0., 0.)), true);
}
