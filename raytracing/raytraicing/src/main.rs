use std::fs;
use std::io::{BufWriter,Write};

mod vec3;
mod ray;

use vec3::{Vec3};
use ray::{Ray};

//ppmを生成
fn create_ppm(nx:i32, ny:i32) {
    let mut f = BufWriter::new(fs::File::create("rs.ppm").unwrap());
    f.write(b"P3\n").
    unwrap();
    f.write(format!("{} {} {}\n",nx, ny, 255).as_bytes()).unwrap();
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();
    for y in 0..ny {
        for x in 0..nx {
            let u = x as f32 / nx as f32;
            let v = y as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            let col = color(r);
            let ir:i32 = (255.99 * col[0]) as i32;
            let ig:i32 = (255.99 * col[1]) as i32;
            let ib:i32 = (255.99 * col[2]) as i32;
            let s = format!("{} {} {}\n",ir, ig, ib);
            f.write(s.as_bytes()).unwrap();
        }
    }
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0,1.0,1.0)*(1.0-t) + Vec3::new(0.5,0.7,1.0)*t
}

fn main() {

    create_ppm(255, 255);
    println!("Finish!");
}
