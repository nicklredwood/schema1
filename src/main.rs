use uuid::Uuid;
use clap::Parser;
//use uuid::{uuid, Uuid};
//use serde::{Serialize, Deserialize};

mod cli;

fn main() {
	println!("Hello, world!");
	let cli = cli::Cli::parse();

	// for _ in 0..args.count {
	// 	println!("Hello {}!", args.name);
	// }
	println!("{:?}", cli);
}


struct Record {
    uuid: Uuid,
    content: RecordType,
}

enum RecordType {
    Artist(Artist),
    Track(Track),
    Album(Album),
}

struct Artist {
	name: String,
}

struct Track {
	artist: Artist,
	name: String,
}

struct Album {
	tracks: Vec<Track>,
}

impl Artist {
	fn from(name: String) -> Artist {
    Artist { name }
	}
}

