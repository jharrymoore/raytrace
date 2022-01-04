use std::env;
use std::fs;

use raytrace::config::Config;
use raytrace::raytrace::render;


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		println!("Usage: {}, <config file>, <output_file>", args[0]);
		return
	};
	let json = fs::read(&args[1]).expect("Failed to read file");

	// the enetire scene is configured in the json
	let scene: Config = serde_json::from_slice(&json).expect("couldn't parse json config file");
	let filename = &args[2];
	println!("\nRendering {}", filename);
	render(&filename, scene)
}
