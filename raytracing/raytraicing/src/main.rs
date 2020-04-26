use std::fs;
use std::io::{BufWriter,Write};

mod vec3;
use vec3::{Vec3};

//ppmを生成
fn create_ppm(nx:i32, ny:i32) {
    let mut f = BufWriter::new(fs::File::create("rs.ppm").unwrap());
    f.write(b"P3\n").
    unwrap();
    f.write(format!("{} {} {}\n",nx, ny, 255).as_bytes()).unwrap();
    for y in 0..ny {
        for x in 0..nx {
            let col = Vec3::new(x as f32 / nx as f32, y as f32/ ny as f32, 0.2);
            let ir:i32 = (255.99 * col[0]) as i32;
            let ig:i32 = (255.99 * col[1]) as i32;
            let ib:i32 = (255.99 * col[2]) as i32;
            let s = format!("{} {} {}\n",ir, ig, ib);
            f.write(s.as_bytes()).unwrap();
        }
    }
}

fn main() {
    create_ppm(255, 255);
    println!("Finish!");
}
