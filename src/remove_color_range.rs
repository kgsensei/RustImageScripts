use image::{Rgb, RgbImage, DynamicImage, ImageBuffer};
use palette::{FromColor, Hsl, Srgb};

fn is_in_range(real_hue: f32, min: f32, max: f32) -> bool {
	if real_hue > 0.0 {
		(min..max).contains(&real_hue)
	} else {
		(min..max).contains(&(real_hue + 360.0))
	}
}

pub fn init(contents: DynamicImage, args: Vec<String>) {
	if args.len() < 5 {
		println!("This mode requires a 3rd and 4th launch parameter for hue rotation and threshold");
		return;
	}
	println!("Removing Color Range {}deg to {}deg", args[3], args[4]);
	let min: f32 = args[3].parse::<f32>().unwrap();
	let max: f32 = args[4].parse::<f32>().unwrap();
	let width: u32 = contents.width();
	let height: u32 = contents.height();
	let mut image: RgbImage = ImageBuffer::new(width, height);

	for (x, y, pixel) in contents.to_rgb8().enumerate_pixels() {
		let rgb = Srgb::new(
			pixel[0] as f32 / 255.0,
			pixel[1] as f32 / 255.0,
			pixel[2] as f32 / 255.0
		);
		let hsl: Hsl = Hsl::from_color(rgb);
		if !is_in_range(hsl.hue.into_degrees(), min, max) {
			image.put_pixel(x, y, *pixel);
		} else {
			let grey = (
				0.299 * pixel[0] as f32 +
				0.587 * pixel[1] as f32 +
				0.114 * pixel[2] as f32
			) as u8;
			image.put_pixel(x, y, Rgb([grey, grey, grey]));
		}
	}

	let _ = image.save("res.png");
}
