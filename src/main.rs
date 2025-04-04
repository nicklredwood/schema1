use clap::Parser;

// #[macro_use]
// extern crate schema;

mod cli;
mod debug;

mod record;
mod store;

// use serde::{Serialize, Deserialize};

fn main() {
	use record::*;
	use artist::*;

	println!("Hello, world!");
	let cli = cli::Cli::parse();

	// for _ in 0..args.count {
	// 	println!("Hello {}!", args.name);
	// }
	println!("{:?}", cli);

	let pink_floyd = Artist::from("Pink Floyd");
	let record = Record::from(pink_floyd);
}
