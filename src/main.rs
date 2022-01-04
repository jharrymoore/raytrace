use std::env;
use std::fs;

use raytrace::config::Config;


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		println!("Usage: {}, <config file>, <output_file>", args[0]);
		return
	};
	let json = fs::read(&args[1]).expect("Failed to read file");

	let scene = serde_json::from_slice::<Config>(&json).expect("couldn't parse json config file");
}
