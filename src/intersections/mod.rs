use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vectors::{point, vector};

#[derive(Clone)]
pub struct Intersection {
  pub t: f64,
  pub object: Sphere,
}

impl Intersection {
  pub fn new(t: f64, object: Sphere) -> Intersection {
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

impl Intersections {
  pub fn new(intersections: Vec<Intersection>) -> Intersections {
    Intersections {
      intersections: intersections,
    }
  }

  pub fn hit(&self) -> Intersections {
    if self.intersections.len() == 0 {
      return Intersections::new(vec![]);
    }
    let mut hit = Intersection::new(
      self.intersections[0].t,
      self.intersections[0].object.clone(),
    );

    for intersect in &self.intersections {
      if intersect.t < hit.t && intersect.t >= 0. {
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

#[test]
fn intersection_encapsulates_a_t_and_object() {
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
  let handle = s.handle;
  let i = Intersection::new(3.5, s);

  assert_eq!(i.t, 3.5);
  assert_eq!(i.object.handle, handle);
}

#[test]
fn intersections_aggregate() {
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
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
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);

  let xs = s.intersects(r);

  assert_eq!(xs.intersections.len(), 2);
  assert_eq!(xs.intersections[0].object.handle, s.handle);
  assert_eq!(xs.intersections[1].object.handle, s.handle);
}

#[test]
fn intersections_hit_when_all_intersections_are_postive() {
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
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
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
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
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
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
  let s = Sphere::new(point(0.0, 0.0, 0.0), 1.);
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
