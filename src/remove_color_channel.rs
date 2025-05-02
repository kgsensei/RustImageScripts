use image::{RgbaImage, DynamicImage};

pub fn init(contents: DynamicImage, args: Vec<String>) {
	if args.len() < 4 {
		println!("This mode requires a 3rd launch parameter for channel to remove");
		return;
	}

	let channel: usize = match args[3].as_ref() {
		"0" => 0,
		"1" => 1,
		"2" => 2,
		"3" => 3,
		_ => 0
	};
	println!("Removing Color Channel {}", channel);

	let mut image = RgbaImage::from(contents);

	image
		.pixels_mut()
		.for_each(|p| p[channel] = 0);
	
	let _ = image.save("res.png");
}
