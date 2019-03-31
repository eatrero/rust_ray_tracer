mod vectors;
use vectors::{point, vector};

mod world;
use world::World;

mod colors;
use colors::Color;

mod canvas;
use canvas::Canvas;

mod matrix;

mod transform;
use transform::Transform;

mod intersections;
mod ray;
use ray::Ray;
mod shape;
use shape::{Shape, ShapeType};
use std::f64;
mod light;
use light::{lighting, PointLight};
mod camera;
use camera::Camera;
mod material;
use material::Material;
use std::time::{Duration, Instant};
mod pattern;
use pattern::{Pattern, PatternType};

fn world() {
    let width = 800;
    let height = 400;
    let fov = f64::consts::PI / 3.0;
    let mut camera = Camera::new(width, height, fov);
    let mut world = World::new();

    // setup light
    world.set_light(PointLight::new(
        point(-5., 5., -10.),
        Color::new(1., 1., 1.),
    ));

    // setup floor
    let mut floor = Shape::new(ShapeType::Plane);
    floor.transform = Transform::new().translate(0., 0., 0.).transform;
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    let mut pattern = Pattern::new(
        PatternType::Checker,
        Color::new(1.0, 1., 1.),
        Color::new(0.5, 0.5, 5.),
    );
    let pattern_tx = Transform::new().scale(1., 1., 1.).transform;
    pattern.set_transform(pattern_tx);
    floor.material.set_pattern(pattern);
    world.add_object(floor);

    // setup left wall
    let mut left_wall = Shape::new(ShapeType::Plane);
    left_wall.transform = Transform::new()
        .translate(-10., 0., 4.)
        .rotate_y(3.0 * f64::consts::PI / 2.0)
        .rotate_x(f64::consts::PI / 2.0)
        .transform;
    left_wall.material = Material::new();
    left_wall.material.color = Color::new(1.0, 0.9, 0.9);
    left_wall.material.specular = 0.;
    let mut pattern = Pattern::new(
        PatternType::Checker,
        Color::new(1.0, 0., 0.),
        Color::new(0., 0., 1.),
    );
    let pattern_tx = Transform::new().scale(0.2, 0.2, 0.2).transform;
    pattern.set_transform(pattern_tx);
    left_wall.material.set_pattern(pattern);
    world.add_object(left_wall);

    // setup right wall
    let mut right_wall = Shape::new(ShapeType::Plane);
    right_wall.transform = Transform::new()
        .translate(0., 0., 8.)
        .rotate_y(1.0 * f64::consts::PI / 4.0)
        .rotate_x(f64::consts::PI / 2.0)
        .transform;
    right_wall.material = Material::new();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.;
    let mut pattern = Pattern::new(
        PatternType::Ring,
        Color::new(1., 1., 1.),
        Color::new(0., 0., 1.),
    );
    let pattern_tx = Transform::new().scale(0.2, 0.2, 0.2).transform;
    pattern.set_transform(pattern_tx);
    right_wall.material.set_pattern(pattern);
    world.add_object(right_wall);

    // setup middle sphere
    let mut middle = Shape::new(ShapeType::Sphere);

    let pattern_tx = Transform::new()
        .scale(0.20, 0.20, 0.20)
        .translate(-0.5, 0., 0.8)
        .rotate_y(2.7 * f64::consts::PI / 4.0)
        .rotate_x(2.3 * f64::consts::PI / 4.0)
        .rotate_z(0.3333 * f64::consts::PI / 4.0)
        .transform;

    middle.transform = Transform::new()
        .scale(2.0, 2.0, 2.0)
        .translate(-1., 1., -0.8)
        .transform;
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.1);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut pattern = Pattern::new(
        PatternType::Stripe,
        Color::new(1.0, 0., 0.),
        Color::new(0., 1., 1.),
    );
    pattern.set_transform(pattern_tx);
    middle.material.set_pattern(pattern);
    world.add_object(middle);

    // setup right sphere
    let mut right = Shape::new(ShapeType::Sphere);
    right.transform = Transform::new()
        .translate(0.5, 1., -5.)
        .scale(0.5, 0.5, 0.5)
        .transform;
    right.material = Material::new();
    right.material.color = Color::new(1.0, 0., 0.);
    right.material.diffuse = 0.8;
    right.material.specular = 0.3;
    let mut pattern = Pattern::new(
        PatternType::Gradient,
        Color::new(1.0, 0.25, 0.),
        Color::new(0., 0.25, 1.),
    );
    let pattern_tx = Transform::new()
        .translate(0.5, 0., 0.)
        .scale(2.0, 2.0, 2.0)
        .transform;
    pattern.set_transform(pattern_tx);
    right.material.set_pattern(pattern);
    world.add_object(right);

    // setup left sphere
    let mut left = Shape::new(ShapeType::Sphere);
    left.transform = Transform::new()
        .translate(-3.5, 2.3, -6.)
        .scale(0.66, 0.66, 0.66)
        .transform;
    left.material = Material::new();
    left.material.color = Color::new(0.1, 0.1, 1.0);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    let mut pattern = Pattern::new(
        PatternType::Ring,
        Color::new(1.0, 0., 0.),
        Color::new(0., 0., 1.),
    );
    let pattern_tx = Transform::new().scale(0.2, 0.2, 0.2).transform;
    pattern.set_transform(pattern_tx);
    left.material.set_pattern(pattern);
    world.add_object(left);

    camera.transform =
        Transform::view_transform(point(0., 1.5, -14.), point(0., 1., 0.), vector(0., 1., 0.));

    let image = camera.render(world);
    image.write();
}

fn main() {
    let now = Instant::now();
    world();
    println!("seconds elapsed: {}", now.elapsed().as_secs());
}
