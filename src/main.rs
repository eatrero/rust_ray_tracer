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
use shape::sphere::Sphere;
use std::f64;
mod light;
use light::{lighting, PointLight};
mod camera;
use camera::Camera;
mod material;
use material::Material;

fn sphere_projection() {
    let width = 100;
    let height = width;
    let mut c = Canvas::new(width, height);
    //    let red = Color::new(1., 0., 0.);
    let wall_size = 7.0 as f64;
    let pixel_size = wall_size / width as f64;
    let half = wall_size / 2.0;
    let wall_z = 10.0 as f64;
    let ray_origin = point(0., 0., -10.5);

    let mut shape = Sphere::new(point(0., 0., 0.), 1.);
    shape.material.color = Color::new(0.6, 0.9, 1.);
    shape.material.specular = 0.01;
    shape.material.diffuse = 1.5;
    shape.material.ambient = 0.001;
    let light_position = point(-3., 5., -10.);
    let light_color = Color::new(1., 1., 1.);
    let l = PointLight::new(light_position, light_color);

    let _transform = Transform::new()
        .rotate_z(f64::consts::PI / 4.)
        .scale(0.5, 1., 1.)
        .transform;
    //   shape.set_transform(_transform);

    for y in 1..height {
        let world_y = half - pixel_size * y as f64;
        for x in 1..width {
            let world_x = half - pixel_size * x as f64;

            let position = point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, position.sub(ray_origin).norm());

            /*
            let xs = shape.intersects(r);
            let hit = xs.hit();
            if hit.intersections.len() > 0 {
                let first_hit = &hit.intersections[0];
                let m = first_hit.object.material;
                let p = r.position(first_hit.t);
                let normv = first_hit.object.normal_at(p);
                let eye = r.direction.negate();
                let is_in_shadow = false;
                let color = lighting(m, l, p, eye, normv, is_in_shadow);
                c.set(x, y, color);
            }
            */
        }
    }

    c.write();
}

fn world() {
    let width = 100;
    let height = 50;
    let fov = f64::consts::PI / 3.0;
    let mut camera = Camera::new(width, height, fov);
    let mut world = World::new();

    // setup light
    world.set_light(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    ));

    // setup floor
    let mut floor = Sphere::new(point(0., 0., 0.), 1.0);
    floor.transform = Transform::new().scale(10., 0.01, 10.0).transform;
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    world.add_object(floor);

    // setup left wall
    let mut left_wall = Sphere::new(point(0., 0., 0.), 1.0);
    left_wall.transform = Transform::new()
        .translate(0., 0., 5.)
        .rotate_y(-f64::consts::PI / 4.0)
        .rotate_x(f64::consts::PI / 2.0)
        .scale(10., 0.01, 10.)
        .transform;
    left_wall.material = Material::new();
    left_wall.material.color = Color::new(1.0, 0.9, 0.9);
    left_wall.material.specular = 0.;
    world.add_object(left_wall);

    // setup right wall
    let mut right_wall = Sphere::new(point(0., 0., 0.), 1.0);
    right_wall.transform = Transform::new()
        .translate(0., 0., 5.)
        .rotate_y(f64::consts::PI / 4.0)
        .rotate_x(f64::consts::PI / 2.0)
        .scale(10., 0.01, 10.)
        .transform;
    right_wall.material = Material::new();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.;
    world.add_object(right_wall);

    // setup middle sphere
    let mut middle = Sphere::new(point(0., 0., 0.), 1.0);
    middle.transform = Transform::new().translate(-0.5, 1., 0.5).transform;
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    // setup right sphere
    let mut right = Sphere::new(point(0., 0., 0.), 1.0);
    right.transform = Transform::new()
        .translate(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5)
        .transform;
    right.material = Material::new();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);
    camera.transform =
        Transform::view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));

    // setup left sphere
    let mut left = Sphere::new(point(0., 0., 0.), 1.0);
    left.transform = Transform::new()
        .translate(-1.8, 1.8, -1.)
        .scale(0.33, 0.33, 0.33)
        .transform;
    left.material = Material::new();
    left.material.color = Color::new(1., 0.8, -0.75);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);
    camera.transform =
        Transform::view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));

    let image = camera.render(world);
    image.write();
}

fn main() {
    world();
}
