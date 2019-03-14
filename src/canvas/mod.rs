use crate::colors::Color;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Canvas {
  width: usize,
  height: usize,
  canvas: Vec<Color>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Canvas {
    let mut canvas = Vec::with_capacity(width * height);

    for _i in 0..width * height {
      canvas.push(Color::new(0., 0., 0.));
    }

    return Canvas {
      width: width,
      height: height,
      canvas: canvas,
    };
  }

  pub fn get(&mut self, x: usize, y: usize) -> Color {
    return self.canvas[x + y * self.width];
  }

  pub fn set(&mut self, x: usize, y: usize, c: Color) {
    self.canvas[x + y * self.width] = c;
  }

  pub fn write(&self) {
    let path = Path::new("./test-output.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why.description()),
      Ok(file) => file,
    };

    let canvas_output = Canvas::canvas_to_ppm(self);

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(canvas_output.as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
      Ok(_) => println!("successfully wrote to {}", display),
    }
  }

  fn canvas_to_ppm(&self) -> String {
    let Canvas {
      width,
      height,
      canvas,
    } = self;

    let mut output: String = format!("P3\n{} {}\n255\n", width, height).to_owned();

    fn c2u8(c: f64) -> u32 {
      let mut out = (c * 255.0) as u32;
      out = if out > 255 { 255 } else { out };
      return out;
    }

    println!("start canvas_to_ppm");

    for i in 0..width * height {
      let red = c2u8(canvas[i].r).to_string() + " ";
      let green = c2u8(canvas[i].g).to_string() + " ";
      let blue = c2u8(canvas[i].g).to_string() + " ";

      output.push_str(&red[..]);
      output.push_str(&green[..]);
      output.push_str(&blue[..]);
      if i % 10 == 9 {
        output.push_str("\n");
      }
    }
    output.push_str("\n");
    println!("end canvas_to_ppm");
    return output;
  }
}

#[test]
fn it_creates_a_canvas() {
  let width = 10;
  let height = 10;
  let c = Canvas::new(width, height);
  let black = Color::new(0., 0., 0.);

  c.write();

  for i in 0..width * height {
    let pixel = c.canvas[i];
    assert_eq!(Color::equals(pixel, black), true);
  }
}

#[test]
fn it_writes_a_pixel() {
  let mut c = Canvas::new(10, 10);
  let red = Color::new(1., 0., 0.);
  c.set(2, 3, red);

  let pixel = c.get(2, 3);
  assert_eq!(Color::equals(pixel, red), true);
}

#[test]
fn it_writes_to_ppm() {
  let mut c = Canvas::new(10, 10);
  let red = Color::new(1., 1.0, 1.0);
  c.set(1, 1, red);
  //let output = c.canvas_to_ppm();
  //println!("{}", output)

  c.write();
  //  assert_eq!(Color::equals(pixel, red), true);
}
