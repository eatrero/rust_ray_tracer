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
    //    floor.transform = Transform::new().translate(0., 0., 0.).transform;
    floor.material = Material::new();
    //    floor.material.color = Color::new(0.5, 0.5, 0.5);
    //floor.transform = Transform::new().rotate_x(f64::consts::PI / 2.0).transform;

    floor.material.specular = 0.0;
    floor.material.reflectiveness = 0.25;
    let mut floor_pattern = Pattern::new(
        PatternType::Checker,
        Color::new(1.0, 1., 1.),
        Color::new(0., 0.0, 0.),
    );
    floor.material.set_pattern(floor_pattern);
    world.add_object(floor);

    // setup left wall
    let mut left_wall = Shape::new(ShapeType::Plane);
    left_wall.transform = Transform::new()
        .translate(-10., 0., 4.)
        .rotate_y(3.0 * f64::consts::PI / 2.0)
        .rotate_x(f64::consts::PI / 2.0)
        .transform;
    left_wall.material = Material::new();
    let mut pattern = Pattern::new(
        PatternType::Checker,
        Color::new(0.7, 0.7, 0.7),
        Color::new(0., 0.0, 0.),
    );
    left_wall.material.reflectiveness = 0.1;
    left_wall.material.set_pattern(pattern);
    world.add_object(left_wall);

    // setup right wall
    let mut right_wall = Shape::new(ShapeType::Plane);
    right_wall.transform = Transform::new()
        .translate(0., 0., 3.)
        .rotate_y(1.0 * f64::consts::PI / 4.0)
        .rotate_x(f64::consts::PI / 2.0)
        .transform;
    right_wall.material = Material::new();
    let mut pattern = Pattern::new(
        PatternType::Checker,
        Color::new(0.7, 0.7, 0.7),
        Color::new(0., 0.0, 0.),
    );
    right_wall.material.reflectiveness = 0.1;
    right_wall.material.set_pattern(pattern);

    world.add_object(right_wall);

    // setup middle sphere
    let mut middle = Shape::new(ShapeType::Sphere);
    middle.transform = Transform::new()
        .scale(2.0, 2.0, 2.0)
        .translate(-1., 1., -0.8)
        .transform;
    middle.material = Material::new();
    middle.material.color = Color::new(0.0, 0.0, 0.0);
    middle.material.ambient = 0.0;
    middle.material.diffuse = 0.0;
    middle.material.reflectiveness = 1.0;
    world.add_object(middle);

    // setup right sphere
    let mut right = Shape::new(ShapeType::Sphere);
    right.transform = Transform::new()
        .translate(0.5, 1., -5.)
        .scale(0.5, 0.5, 0.5)
        .transform;
    right.material = Material::new();
    right.material.color = Color::new(0.0, 1.0, 0.0);
    right.material.reflectiveness = 0.5;
    world.add_object(right);

    // setup left sphere
    let mut left = Shape::new(ShapeType::Sphere);
    left.transform = Transform::new()
        .translate(-3.5, 2.3, -6.)
        .scale(0.66, 0.66, 0.66)
        .transform;
    left.material = Material::new();
    left.material.color = Color::new(1.0, 0.0, 0.0);
    left.material.reflectiveness = 0.5;
    world.add_object(left);

    // setup back sphere
    let mut back = Shape::new(ShapeType::Sphere);
    back.transform = Transform::new()
        .translate(2.8, 2.0, -6.2)
        .scale(2.0, 2.0, 2.0)
        .transform;
    back.material = Material::new();
    back.material.color = Color::new(0.0, 0.0, 0.0);
    back.material.transparency = 1.0;
    back.material.refractive_index = 1.5;
    back.material.reflectiveness = 0.9;
    back.material.ambient = 0.0;

    world.add_object(back);

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
