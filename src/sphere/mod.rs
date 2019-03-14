use crate::colors::Color;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vectors::Tuple;
use crate::vectors::{dot, point, vector};
use rand::Rng;
use std::f64;

#[derive(Clone)]
pub struct Sphere {
  pub origin: Tuple,
  pub radius: f64,
  pub handle: u32,
  pub transform: Matrix,
  pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Intersects {
  pub x1: f64,
  pub x2: f64,
  pub count: usize,
}

impl Sphere {
  pub fn new(origin: Tuple, radius: f64) -> Sphere {
    let mut rng = rand::thread_rng();
    return Sphere {
      origin: origin,
      radius: radius,
      handle: rng.gen::<u32>(),
      transform: Matrix::identity(4),
      material: Material::new(),
    };
  }

  pub fn intersects(&self, ray: Ray) -> Intersections {
    let i = Matrix::inverse(&self.transform);
    let ray2 = ray.transform(&i);
    let sphere_to_ray = ray2.origin.sub(point(0., 0., 0.));
    let a = dot(ray2.direction, ray2.direction);
    let b = 2. * dot(ray2.direction, sphere_to_ray);
    let c = dot(sphere_to_ray, sphere_to_ray) - 1.;
    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
      return Intersections {
        intersections: vec![],
      };
    }

    let dsqrt = discriminant.sqrt();
    let t1 = (-b - dsqrt) / 2. / a;
    let t2 = (-b + dsqrt) / 2. / a;
    if t2 < t1 {
      return Intersections {
        intersections: vec![
          Intersection::new(t2, self.clone()),
          Intersection::new(t1, self.clone()),
        ],
      };
    }
    return Intersections {
      intersections: vec![
        Intersection::new(t1, self.clone()),
        Intersection::new(t2, self.clone()),
      ],
    };
  }

  pub fn set_transform(&mut self, transform: Matrix) {
    self.transform = transform;
  }

  pub fn normal_at(&self, p: Tuple) -> Tuple {
    let inverse_transform = Matrix::inverse(&self.transform);
    let object_point = Matrix::mult_4x4_by_1d(&inverse_transform, &p);
    let origin = point(0., 0., 0.);
    let object_normal = object_point.sub(origin).norm();
    let transposed_inverse_transform = Matrix::transpose(&inverse_transform);
    let mut world_normal = Matrix::mult_4x4_by_1d(&transposed_inverse_transform, &object_normal);
    world_normal.w = 0.;
    return world_normal.norm();
  }
}

#[test]
fn it_computes_intersects_1() {
  let origin = point(0., 0., -5.);
  let direction = vector(0., 0., 1.);
  let r = Ray::new(origin, direction);
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 2);
  assert_eq!(intersects.intersections[0].t, 4.);
  assert_eq!(intersects.intersections[1].t, 6.);
}

#[test]
fn it_computes_intersects_2() {
  let origin = point(0., 1., -5.);
  let direction = vector(0., 0., 1.);
  let r = Ray::new(origin, direction);
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 2);
  assert_eq!(intersects.intersections[0].t, 5.);
  assert_eq!(intersects.intersections[1].t, 5.);
}

#[test]
fn it_computes_intersects_3() {
  let origin = point(0., 2., -5.);
  let direction = vector(0., 0., 1.);
  let r = Ray::new(origin, direction);
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 0);
}

#[test]
fn it_computes_intersects_4() {
  let origin = point(0., 0., 0.);
  let direction = vector(0., 0., 1.);
  let r = Ray::new(origin, direction);
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 2);
  assert_eq!(intersects.intersections[0].t, -1.);
  assert_eq!(intersects.intersections[1].t, 1.);
}

#[test]
fn it_computes_intersects_5() {
  let origin = point(0., 0., 5.);
  let direction = vector(0., 0., 1.);
  let r = Ray::new(origin, direction);
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 2);
  assert_eq!(intersects.intersections[0].t, -6.);
  assert_eq!(intersects.intersections[1].t, -4.);
}

#[test]
fn default_transformation_is_identity() {
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = s.transform;
  let i = Matrix::identity(4);

  assert_eq!(Matrix::equals(&transform, &i), true);
}

#[test]
fn can_set_transformation() {
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = Transform::new().translate(2., 3., 4.).transform;
  s.set_transform(transform);
  let exp = Transform::new().translate(2., 3., 4.).transform;

  assert_eq!(Matrix::equals(&s.transform, &exp), true);
}

#[test]
fn intersecting_scaled_sphere_with_a_ray() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = Transform::new().scale(2., 2., 2.).transform;
  s.set_transform(transform);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 2);
  assert_eq!(intersects.intersections[0].t, 3.);
  assert_eq!(intersects.intersections[1].t, 7.);
}

#[test]
fn intersecting_translated_sphere_with_a_ray() {
  let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = Transform::new().translate(5., 0., 0.).transform;
  s.set_transform(transform);
  let intersects = s.intersects(r);

  assert_eq!(intersects.intersections.len(), 0);
}

#[test]
fn compute_normal_at_a_point_on_x_axis() {
  let s = Sphere::new(point(0., 0., 0.), 1.);

  let n = s.normal_at(point(1., 0., 0.));
  assert_eq!(n.equals(vector(1., 0., 0.)), true);
}

#[test]
fn compute_normal_at_a_point_on_y_axis() {
  let s = Sphere::new(point(0., 0., 0.), 1.);

  let n = s.normal_at(point(0., 1., 0.));
  assert_eq!(n.equals(vector(0., 1., 0.)), true);
}

#[test]
fn compute_normal_at_a_point_on_z_axis() {
  let s = Sphere::new(point(0., 0., 0.), 1.);

  let n = s.normal_at(point(0., 0., 1.));
  assert_eq!(n.equals(vector(0., 0., 1.)), true);
}

#[test]
fn compute_normal_at_a_non_axial_point() {
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let root3 = 3.0f64.sqrt() / 3.;

  let n = s.normal_at(point(root3, root3, root3));
  assert_eq!(n.equals(vector(root3, root3, root3)), true);
}

#[test]
fn the_normal_is_a_normalized_vector() {
  let s = Sphere::new(point(0., 0., 0.), 1.);
  let root3 = 3.0f64.sqrt() / 3.;

  let n = s.normal_at(point(root3, root3, root3));

  let exp = n.norm();
  assert_eq!(n.equals(exp), true);
}

#[test]
fn compute_normal_on_a_translated_sphere() {
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = Transform::new().translate(0., 1., 0.).transform;
  s.set_transform(transform);

  let n = s.normal_at(point(0., 1.70711, -0.70711));

  assert_eq!(n.approx_equals(vector(0., 0.70711, -0.70711)), true);
}

#[test]
fn compute_normal_on_a_transformed_sphere() {
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  let transform = Transform::new()
    .scale(1., 0.5, 1.)
    .rotate_z(f64::consts::PI / 5.)
    .transform;
  s.set_transform(transform);

  let n = s.normal_at(point(0., 2.0f64.sqrt() / 2., -2.0f64.sqrt() / 2.));
  assert_eq!(n.approx_equals(vector(0., 0.97014, -0.24254)), true);
}

#[test]
fn a_sphere_has_a_default_material() {
  let mut s = Sphere::new(point(0., 0., 0.), 1.);

  let m = s.material;
  assert_eq!(Color::equals(m.color, Color::new(1., 1., 1.)), true);
}

#[test]
fn a_sphere_can_change_its_material() {
  let mut s = Sphere::new(point(0., 0., 0.), 1.);
  s.material.ambient = 1.0;
  s.material.color = Color::new(1., 0.2, 1.0);

  assert_eq!(s.material.ambient, 1.);
  assert_eq!(
    Color::equals(s.material.color, Color::new(1., 0.2, 1.0)),
    true
  );
}
