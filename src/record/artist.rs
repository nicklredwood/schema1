use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::Record;
use schema::{Namespace, UuidEncode};

// TODO: write a type-derive macro. Something like:
// #[namespace(Record)]
// #[derive(Namespace)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Artist {
	name: String,
}

impl Artist {
	pub(crate) fn from(name: &str) -> Artist {
		Artist { name: name.to_owned() }
	}
}

impl Namespace for Artist {
	const NS: Uuid = Uuid::new_v5(&Record::NS, Artist::serialize());
}

impl UuidEncode for Artist {
	fn encode(&self) -> Uuid {
		Uuid::new_v5(&Self::NS, self.name)
	}
}
