use std::error::Error;
use std::fs::File;
use std::io::{LineWriter, Write};

// log modules
use log::debug;
use env_logger;

fn main() -> Result<(), Box<dyn Error>> {

		env_logger::init();

		// image 
    let image_width = 256;
		let image_height = 256;

		// render
		let img_file = File::create("images/image.ppm")?;
		let mut img_file = LineWriter::new(img_file);

		img_file.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

		for j in 0..image_height {
			debug!("Scanlines remaining: {}", image_height - j);
			for i in 0..image_width {
				let r = i as f64 / ((image_width - 1) as f64);
				let g = j as f64 / ((image_height - 1) as f64);
				let b = 0.0;

				let ir = (255.999 * r) as i32;
				let ig = (255.999 * g) as i32;
				let ib = (255.99 * b) as i32;

				img_file.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
			}
		}
		img_file.flush()?;

		Ok(())
}
