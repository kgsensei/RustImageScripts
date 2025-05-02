pub mod remove_color_channel;
pub mod isolate_color_range;

use std::fs;
use std::env;

use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() < 3 {
		println!("Requires at least two parameters [<img> <mode> <extra?>]\n");
		println!("[0]> Remove Color Channel  extra:<channel [0=r, 1=g, 2=b]>");
		println!("[1]> Isolate Color Range   extra:<from_deg> <to_deg>");
		return Ok(());
	}

	let file_path = &args[1];

	if fs::metadata(file_path).is_err() {
		println!("Error: image file '{}' does not exist or you do not have permission to use it", file_path);
		return Ok(());
	}

	let contents = ImageReader::open(file_path)?.decode()?;

	match args[2].as_ref() {
		"0" => remove_color_channel::init(contents, args),
		"1" => isolate_color_range::init(contents, args),
		_ => println!("Chosen option doesn't exist")
	}
	Ok(())
}
