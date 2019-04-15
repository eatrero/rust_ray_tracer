use crate::canvas::Canvas;
use crate::colors::Color;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vectors::{point, vector, Tuple};
use crate::world::World;
use std::f64;
use std::thread;
extern crate rayon;
use rayon::prelude::*;

pub struct Camera {
  hsize: usize,
  vsize: usize,
  half_width: f64,
  half_height: f64,
  fov: f64,
  pub transform: Matrix,
  pixel_size: f64,
}

impl Camera {
  pub fn new(hsize: usize, vsize: usize, fov: f64) -> Camera {
    let mut half_width;
    let mut half_height;
    let mut half_view = (fov / 2.0).tan();
    let aspect = hsize as f64 / vsize as f64;

    if aspect >= 1. {
      half_width = half_view;
      half_height = half_view / aspect;
    } else {
      half_width = half_view * aspect;
      half_height = half_view;
    }
    let pixel_size = half_width * 2.0 / hsize as f64;

    return Camera {
      hsize: hsize,
      vsize: vsize,
      half_width: half_width,
      half_height: half_height,
      fov: fov,
      pixel_size: pixel_size,
      transform: Matrix::identity(4),
    };
  }

  pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
    let xoffset = (x as f64 + 0.5) * self.pixel_size;
    let yoffset = (y as f64 + 0.5) * self.pixel_size;

    let world_x = self.half_width - xoffset;
    let world_y = self.half_height - yoffset;

    let pixel = Matrix::mult_4x4_by_1d(
      &Matrix::inverse(&self.transform),
      &point(world_x, world_y, -1.),
    );
    let origin = Matrix::mult_4x4_by_1d(&Matrix::inverse(&self.transform), &point(0., 0., 0.));
    let direction = pixel.sub(origin).norm();

    return Ray::new(origin, direction);
  }

  pub fn render_line(&self, world: &World, line: usize) -> Vec<Color> {
    let mut out: Vec<Color> = Vec::new();

    for x in 0..(self.hsize / 1) {
      let r = self.ray_for_pixel(x, line);
      let c = world.color_at(r, 5); // maximum recursion depth for camera is 5
      out.push(c);
    }

    return out;
  }

  pub fn render(&self, world: World) -> Canvas {
    let mut canvas = Canvas::new(self.hsize, self.vsize);

    let pixels: Vec<Vec<Color>> = (0..self.vsize)
      .into_par_iter()
      .map(|line| {
        let out = self.render_line(&world, line);
        return out;
      })
      .collect();

    for y in (0..self.vsize) {
      for x in (0..self.hsize) {
        canvas.set(x, y, pixels[y][x]);
      }
    }
    return canvas;
  }
}

#[test]
fn constructing_a_camera() {
  let hsize = 160;
  let vsize = 120;
  let fov = f64::consts::PI / 2.;

  let c = Camera::new(hsize, vsize, fov);

  assert_eq!(c.hsize, 160);
  assert_eq!(c.vsize, 120);
  assert_eq!(c.fov, f64::consts::PI / 2.);
  assert_eq!(Matrix::equals(&c.transform, &Matrix::identity(4)), true);
}

#[test]
fn pixel_size_for_a_horizontal_canvas() {
  let c = Camera::new(200, 125, f64::consts::PI / 2.);
  assert_eq!((c.pixel_size - 0.01).abs() < 0.00001, true);
}

#[test]
fn pixel_size_for_a_vertical_canvas() {
  let c = Camera::new(125, 200, f64::consts::PI / 2.);
  assert_eq!((c.pixel_size - 0.01).abs() < 0.00001, true);
}

#[test]
fn constructing_a_ray_through_center_of_canvas() {
  let c = Camera::new(201, 101, f64::consts::PI / 2.);
  let r = c.ray_for_pixel(100, 50);

  assert_eq!(r.origin.equals(point(0., 0., 0.)), true);
  assert_eq!(r.direction.equals(vector(0., 0., -1.)), true);
}

#[test]
fn constructing_a_ray_through_corner_of_canvas() {
  let c = Camera::new(201, 101, f64::consts::PI / 2.);
  let r = c.ray_for_pixel(0, 0);

  assert_eq!(r.origin.equals(point(0., 0., 0.)), true);
  assert_eq!(
    r.direction
      .approx_equals(vector(0.66519, 0.33259, -0.66851)),
    true
  );
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
  let mut c = Camera::new(201, 101, f64::consts::PI / 2.);
  c.transform = Transform::new()
    .rotate_y(f64::consts::PI / 4.)
    .translate(0., -2., 5.)
    .transform;
  let r = c.ray_for_pixel(100, 50);

  assert_eq!(r.origin.equals(point(0., 2., -5.)), true);
  assert_eq!(
    r.direction
      .approx_equals(vector(2.0f64.sqrt() / 2., 0., -2.0f64.sqrt() / 2.)),
    true
  );
}

#[test]
fn rendering_a_world_with_a_camera() {
  let world = World::default_world();
  let mut c = Camera::new(11, 11, f64::consts::PI / 2.);
  let from = point(0., 0., -5.);
  let to = point(0., 0., 0.);
  let up = vector(0., 1., 0.);

  c.transform = Transform::view_transform(from, to, up);

  let mut image = c.render(world);
  let pixel = image.get(5, 5);

  assert_eq!(
    Color::approx_equals(pixel, Color::new(0.38066, 0.47583, 0.2855)),
    true
  );
}
