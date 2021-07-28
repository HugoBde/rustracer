/*
    To-do:
        - implement `Hittable` trait for `Triangle`
        - implement `TryFrom` trait for `Triangle`?? to parse 3D files and create triangels out of them
        - implement 3D file parser to import Objects in the scene
        - implement `Material` struct
*/

use std::fs::File;
use std::io::{BufWriter, Write};

use rustracer::color::Color;
use rustracer::geometry::{Point3, Ray, Vec3};
use rustracer::hittable::{Hittable, Sphere};

struct Image {
    width: u32,
    height: u32,
    fov: f32,
    name: String,
    buf: Vec<Color>,
}

fn render(w: u32, h: u32) -> Image {
    let sphere = Sphere {
        center: Point3::new(0.5, 0.0, -3.0),
        radius: 0.1,
    };
    let sphere_2 = Sphere {
        center: Point3::new(-0.5, 0.0, -3.0),
        radius: 0.1,
    };
    let background = Color::grey_shade(60);
    let sphere_color = Color::new(145, 76, 69);

    let width = w;
    let height = h;
    let fov = std::f32::consts::FRAC_PI_3;
    let name = String::from("output.ppm");
    let mut buf: Vec<Color> = Vec::with_capacity((width * height) as usize);

    let camera_pos = Point3::new(0.0, 0.0, 0.0);

    for i in 0..height {
        let y =
            (-(i as f32 / height as f32) + 0.5) * (fov / 2.0).tan() * height as f32 / width as f32;
        for j in 0..width {
            let x = ((j as f32 / width as f32) - 0.5) * (fov / 2.0).tan();
            let dir = Vec3::new(x, y, -1.0);
            let ray = Ray::new(camera_pos, dir);
            if let Some(_) = sphere.check_hit(&ray) {
                buf.push(sphere_color);
            } else if let Some(_) = sphere_2.check_hit(&ray) {
                buf.push(sphere_color);
            } else {
                buf.push(background);
            }
        }
    }

    Image {
        width,
        height,
        fov,
        name,
        buf,
    }
}

fn save(img: Image) {
    let mut writer = BufWriter::new(File::create(img.name).unwrap());
    let header = format!("P6\n{} {}\n255\n", img.width, img.height);
    writer.write(header.as_bytes()).unwrap();
    for pixel in img.buf {
        writer.write(&[pixel.r, pixel.g, pixel.b]).unwrap();
    }
}

fn main() {
    println!("Hello, World!");
    let args: Vec<String> = std::env::args().collect();
    let w: u32 = args[1].parse().unwrap();
    let h: u32 = args[2].parse().unwrap();
    println!("{} {}", w, h);
    save(render(w, h));
}
