mod vectors;
use vectors::{point, vector};

mod world;
use world::{tick, Env, Proj};

mod colors;
use colors::Color;

mod canvas;
use canvas::Canvas;

mod matrix;
use matrix::Matrix;

mod transform;
use transform::Transform;

mod intersections;
mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
use std::f64;
mod light;
use light::{lighting, PointLight};
mod camera;
mod material;

fn canon() {
    let width = 600;
    let height = 550;
    let mut c = Canvas::new(width, height);
    let red = Color::new(1., 0., 0.);

    let mut p = Proj::new(point(0., 1., 0.), vector(0.5, 1.8, 0.).norm().mult(9.25));
    let e = Env::new(vector(0., -0.1, 0.), vector(0.01, 0., 0.));
    let mut index = 0;

    fn clamp(a: usize, max: usize) -> usize {
        let mut out = if a >= max { max } else { a };
        out = if out <= 0 { 0 } else { out };
        return out;
    }

    while p.pos.y > 0.0 {
        let mut x = clamp(p.pos.x as usize, width - 1);
        let mut y = height - 1 - clamp(p.pos.y as usize, height - 1);

        println!("position: {} {}", x, y);
        c.set(x, y, red);
        p = tick(e, p);
        index = index + 1;
    }
    println!("position: {} {}, steps: {}", p.pos.x, p.pos.y, index);
    c.write();
}

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

            let xs = shape.intersects(r);
            let hit = xs.hit();
            if hit.intersections.len() > 0 {
                let first_hit = &hit.intersections[0];
                let m = first_hit.object.material;
                let p = r.position(first_hit.t);
                let normv = first_hit.object.normal_at(p);
                let eye = r.direction.negate();
                let color = lighting(m, l, p, eye, normv);
                c.set(x, y, color);
            }
        }
    }

    c.write();
}

fn clock() {
    let width = 100;
    let height = 100;
    let mut c = Canvas::new(width, height);
    let white = Color::new(1., 1., 1.);

    fn clamp(a: usize, max: usize) -> usize {
        let mut out = if a >= max { max } else { a };
        out = if out <= 0 { 0 } else { out };
        return out;
    }

    for i in 0..12 {
        let mut p = point(0., 0., 1.);
        let transform = Transform::new()
            .translate(50., 0., 50.)
            .scale(40., 0., 40.)
            .rotate_y((i as f64) * f64::consts::PI / 6.)
            .transform;
        p = Matrix::mult_4x4_by_1d(&transform, &p);
        let x = clamp(p.x as usize, width - 1);
        let y = clamp(p.z as usize, height - 1);
        println!("{} {} {} {} {} {}", p.x, p.y, p.z, p.w, x, y);
        c.set(x, y, white);
    }

    c.write();
}

fn main() {
    //canon();
    //clock();

    sphere_projection();
}
