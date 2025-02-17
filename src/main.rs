use std::error::Error;
use std::fs::File;
use std::io::{LineWriter, Write};

// log modules
use log::debug;
use env_logger;

mod vectors;
use vectors::color::Color;

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
				let pixel_color = Color(
					i as f64 / ((image_width - 1) as f64),
					j as f64 / ((image_height - 1) as f64),
					0.0
				);

				img_file.write_all(pixel_color.to_string().as_bytes())?;
			}
		}
		img_file.flush()?;

		Ok(())
}
