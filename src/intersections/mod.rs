use crate::ray::Ray;
use crate::shape::plane::Plane;
use crate::shape::sphere::Sphere;
use crate::shape::{Shape, ShapeType};
use crate::transform::Transform;
use crate::vectors::{dot, point, reflect, vector, Tuple};

#[derive(Clone)]
pub struct Intersection {
  pub t: f64,
  pub object: Shape,
}

impl Intersection {
  pub fn new(t: f64, object: Shape) -> Intersection {
    Intersection {
      t: t,
      object: object,
    }
  }
}

#[derive(Clone)]
pub struct Intersections {
  pub intersections: Vec<Intersection>,
}

#[derive(Clone)]
pub struct Computations {
  pub t: f64,
  pub object: Shape,
  pub point: Tuple,
  pub eyev: Tuple,
  pub normalv: Tuple,
  pub reflectv: Tuple,
  pub inside: bool,
  pub over_point: Tuple,
  pub under_point: Tuple,
  pub n1: f64,
  pub n2: f64,
}

pub fn prepare_computations(i: Intersection, r: Ray, xs: Intersections) -> Computations {
  let t = i.t;
  let point = r.position(t);
  let mut normalv = i.object.normal_at(point);
  let eyev = r.direction.negate();
  let mut inside = dot(normalv, eyev) < 0.;
  if inside {
    normalv = normalv.negate();
  } else {
    inside = false;
  }
  let over_point = point.add(normalv.mult(1.0e-10));
  let under_point = point.sub(normalv.mult(1.0e-10));

  let reflectv = reflect(r.direction, normalv);

  let mut containers: Vec<Shape> = vec![];

  let mut n1 = 1.0;
  let mut n2 = 1.0;

  let hit = xs.hit();

  for intersect in xs.intersections {
    if intersect.t == i.t {
      if containers.len() == 0 {
        n1 = 1.;
      } else {
        n1 = containers[containers.len() - 1].material.refractive_index;
      }
    }

    let mut index: usize = 0;

    for idx in 0..containers.len() {
      if containers[idx].handle == intersect.object.handle {
        index = idx + 1;
        break;
      }
    }

    if index > 0 {
      containers.remove(index - 1);
    } else {
      containers.push(intersect.object.clone());
    }

    if intersect.t == i.t {
      if containers.len() == 0 {
        n2 = 1.0;
      } else {
        n2 = containers[containers.len() - 1].material.refractive_index;
      }
      break;
    }
  }

  return Computations {
    t: i.t,
    object: i.object,
    point: point,
    eyev: eyev,
    normalv: normalv,
    reflectv: reflectv,
    inside: inside,
    over_point: over_point,
    under_point: under_point,
    n1: n1,
    n2: n2,
  };
}

impl Intersections {
  pub fn new(intersections: Vec<Intersection>) -> Intersections {
    Intersections {
      intersections: intersections,
    }
  }

  pub fn hit(&self) -> Intersections {
    let len = self.intersections.len();

    if len == 0 {
      return Intersections::new(vec![]);
    }
    let mut hit = Intersection::new(
      self.intersections[0].t,
      self.intersections[0].object.clone(),
    );

    for intersect in &self.intersections {
      if hit.t < 0. || intersect.t < hit.t && intersect.t >= 0. {
        hit = Intersection::new(intersect.t, intersect.object.clone());
      }
    }

    if hit.t >= 0. {
      return Intersections::new(vec![Intersection::new(hit.t, hit.object)]);
    } else {
      return Intersections::new(vec![]);
    }
  }
}

pub fn schlick(comps: Computations) -> f64 {
  let mut cos = dot(comps.eyev, comps.normalv);

  // total internal reflection can only occur if n1 > n2
  if comps.n1 > comps.n2 {
    let n = comps.n1 / comps.n2;
    let sin2_t = n * n * (1.0 - cos * cos);
    if sin2_t > 1.0 {
      return 1.0;
    }

    let cos_t = (1.0 - sin2_t).sqrt();
    cos = cos_t;
  }

  let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
  return r0 + (1. - r0) * (1. - cos).powi(5);
}

#[test]
fn intersection_encapsulates_a_t_and_object() {
  let s = Shape::new(ShapeType::Sphere);
  let handle = s.handle;
  let i = Intersection::new(3.5, s);

  assert_eq!(i.t, 3.5);
  assert_eq!(i.object.handle, handle);
}

#[test]
fn intersections_aggregate() {
  let s = Shape::new(ShapeType::Sphere);
  let s2 = s.clone();
  let i1 = Intersection::new(1., s);
  let i2 = Intersection::new(2., s2);

  let xs = Intersections::new(vec![i1, i2]);

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(xs.intersections[0].t, 1.);
  assert_eq!(xs.intersections[1].t, 2.);
}

#[test]
fn intersections_aggregate_2() {
  let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
  let s = Shape::new(ShapeType::Sphere);

  let xs = s.intersects(r);

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(xs.intersections[0].object.handle, s.handle);
  assert_eq!(xs.intersections[1].object.handle, s.handle);
}

#[test]
fn intersections_hit_when_all_intersections_are_postive() {
  let s = Shape::new(ShapeType::Sphere);
  let s2 = s.clone();

  let i1 = Intersection::new(1., s);
  let handle = i1.object.handle;
  let i2 = Intersection::new(2., s2);

  let xs = Intersections::new(vec![i2, i1]);
  let hit = xs.hit();

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(hit.intersections[0].t, 1.);
  assert_eq!(hit.intersections[0].object.handle, handle);
}

#[test]
fn intersections_hit_when_intersections_are_inside() {
  let s = Shape::new(ShapeType::Sphere);
  let s2 = s.clone();

  let i1 = Intersection::new(-1., s);
  let i2 = Intersection::new(1., s2);
  let handle = i2.object.handle;

  let xs = Intersections::new(vec![i2, i1]);
  let hit = xs.hit();

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(hit.intersections.len(), 1);
  assert_eq!(hit.intersections[0].t, 1.);
  assert_eq!(hit.intersections[0].object.handle, handle);
}

#[test]
fn intersections_hit_when_intersections_are_negative() {
  let s = Shape::new(ShapeType::Sphere);
  let s2 = s.clone();

  let i1 = Intersection::new(-2., s);
  let i2 = Intersection::new(-1., s2);

  let xs = Intersections::new(vec![i2, i1]);
  let hit = xs.hit();

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(hit.intersections.len(), 0);
}

#[test]
fn intersections_hit_is_lowest_non_negative() {
  let s = Shape::new(ShapeType::Sphere);
  let s2 = s.clone();
  let s3 = s.clone();
  let s4 = s.clone();

  let i1 = Intersection::new(5., s);
  let i2 = Intersection::new(7., s2);
  let i3 = Intersection::new(-3., s3);
  let i4 = Intersection::new(2., s4);
  let handle = i4.object.handle;

  let xs = Intersections::new(vec![i1, i2, i3, i4]);
  let hit = xs.hit();

  assert_eq!(xs.intersections.len(), 4);
  assert_eq!(hit.intersections.len(), 1);
  assert_eq!(hit.intersections[0].t, 2.);
  assert_eq!(hit.intersections[0].object.handle, handle);
}

#[test]

fn precompute_the_state_of_an_intersection() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let s = Shape::new(ShapeType::Sphere);
  let i = Intersection::new(4., s);
  let t = i.t;
  let xs = Intersections::new(vec![i.clone()]);

  let comps = prepare_computations(i, r, xs);

  assert_eq!(comps.t, t);
  assert_eq!(comps.point.equals(point(0., 0., -1.)), true);
  assert_eq!(comps.eyev.equals(vector(0., 0., -1.)), true);
  assert_eq!(comps.normalv.equals(vector(0., 0., -1.)), true);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_outside() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let s = Shape::new(ShapeType::Sphere);
  let i = Intersection::new(4., s);
  let xs = Intersections::new(vec![i.clone()]);

  let comps = prepare_computations(i, r, xs);

  assert_eq!(comps.inside, false);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_inside() {
  let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
  let s = Shape::new(ShapeType::Sphere);
  let i = Intersection::new(1., s);
  let xs = Intersections::new(vec![i.clone()]);

  let comps = prepare_computations(i, r, xs);

  assert_eq!(comps.inside, true);
  assert_eq!(comps.point.equals(point(0., 0., 1.)), true);
  assert_eq!(comps.eyev.equals(vector(0., 0., -1.)), true);
  assert_eq!(comps.normalv.equals(vector(0., 0., -1.)), true);
}

#[test]
fn the_hit_should_offset_the_point() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let mut s = Shape::new(ShapeType::Sphere);
  s.transform = Transform::new().translate(0., 0., 1.).transform;
  let i = Intersection::new(5., s);
  let xs = Intersections::new(vec![i.clone()]);

  let comps = prepare_computations(i, r, xs);

  assert_eq!(comps.over_point.z < -1.0e-10 / 2.0, true);
  assert_eq!(comps.point.z > comps.over_point.z, true);
}

#[test]
fn precompute_the_reflection_vector() {
  let half_root2 = 2.0f64.sqrt() / 2.0;
  let r = Ray::new(point(0., 1., -1.), vector(0., -half_root2, half_root2));
  let shape = Shape::new(ShapeType::Plane);

  let i = Intersection::new(half_root2, shape);
  let xs = Intersections::new(vec![i.clone()]);
  let comps = prepare_computations(i, r, xs);

  assert_eq!(
    comps.reflectv.equals(vector(0.0, half_root2, half_root2)),
    true
  );
}

#[test]
fn the_under_point_is_offset_below_the_surface() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.0));
  let mut shape = Shape::glass_sphere();
  let tx = Transform::new().translate(0., 0., 1.).transform;
  shape.set_transform(tx);

  let i = Intersection::new(5.0, shape);
  let xs = Intersections::new(vec![i.clone()]);
  let comps = prepare_computations(i, r, xs);

  assert_eq!(comps.point.z < comps.under_point.z, true);
}

#[test]
fn schlick_approximation_under_total_internal_reflection() {
  let shape = Shape::glass_sphere();
  let r = Ray::new(point(0., 0., 2.0f64.sqrt() / 2.), vector(0., 1., 0.));
  let i1 = Intersection::new(-2.0f64.sqrt() / 2., shape.clone());
  let i2 = Intersection::new(2.0f64.sqrt() / 2., shape.clone());
  let xs = Intersections::new(vec![i1, i2]);
  let comps = prepare_computations(xs.intersections[1].clone(), r, xs);
  let reflectance = schlick(comps);

  assert_eq!(reflectance, 1.0);
}

#[test]
fn schlick_approximation_under_a_perpendicular_viewing_angle() {
  let shape = Shape::glass_sphere();
  let r = Ray::new(point(0., 0., 0.), vector(0., 1., 0.));
  let i1 = Intersection::new(-1., shape.clone());
  let i2 = Intersection::new(1., shape.clone());
  let xs = Intersections::new(vec![i1, i2]);
  let comps = prepare_computations(xs.intersections[1].clone(), r, xs);
  let reflectance = schlick(comps);

  assert_eq!((reflectance - 0.04).abs() < 0.001, true);
}

#[test]
fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
  let shape = Shape::glass_sphere();
  let r = Ray::new(point(0., 0.99, -2.), vector(0., 0., 1.));
  let i1 = Intersection::new(1.8589, shape.clone());
  let xs = Intersections::new(vec![i1]);
  let comps = prepare_computations(xs.intersections[0].clone(), r, xs);
  let reflectance = schlick(comps);

  assert_eq!(reflectance, 0.4887308101221217);
}
