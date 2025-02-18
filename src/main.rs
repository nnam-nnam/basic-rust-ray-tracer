use std::error::Error;
use std::fs::File;
use std::io::{LineWriter, Write};

// log modules
use env_logger;
use log::debug;

mod vectors;
use vectors::Vec3;
use vectors::color::Color;
use vectors::ray::{Ray, Point3};

/// Creates a linear scale for blue and white based on blendedValue function
/// 
/// `blendedValue = startValue * (1 - a) + endValue * a`
fn ray_color(r: &Ray) -> Color {
	let unit_direction = Vec3::unit_vector(r.direction());
	let a = (unit_direction.y() + 1.0) * 0.5;
	Color(1.0, 1.0, 1.0) * (1.0 - a) + Color(0.5, 0.7, 1.0) * a 
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // image dimensions and aspect ratio, in this case 16:9
		let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

		// Determine height based on given width and aspect ratio
		// and assert that it's greater than 1
		let mut image_height = (image_width as f64 / aspect_ratio) as i32;
		image_height = if image_height < 1 { 1 } else { image_height };

		// Set viewport dimensions and camera
		let focal_length = 1.0;
		let viewport_height = 2.0;
		let viewport_width = viewport_height
		* (image_width as f64 / image_height as f64);
		let camera_center: Point3 = Vec3(0.0, 0.0, 0.0);

		// Calculate vectors across viewport rows and down the vertical edge
		let viewport_u = Vec3(viewport_width, 0.0, 0.0);
		let viewport_v = Vec3(0.0, -viewport_height, 0.0);

		// Calculate horizontal and vertical delta vectors between pixels
		let pixel_delta_u = viewport_u / image_width as f64;
		let pixel_delta_v = viewport_v / image_height as f64;

		// Set location of top left pixel in viewport
		let viewport_upper_left = camera_center
			- Vec3(0.0,0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
		let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // render
		// TODO: Add file path in a config file?
    let img_file = File::create("images/image.ppm")?;
    let mut img_file = LineWriter::new(img_file);

    img_file.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in 0..image_height {
        debug!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
						let ray_direction = pixel_center - camera_center;
						let r = Ray::new(&camera_center, &ray_direction);

						let pixel_color = ray_color(&r);

            img_file.write_all(pixel_color.to_string().as_bytes())?;
        }
    }
    img_file.flush()?;
		debug!("\rDone.\t");
    Ok(())
}
