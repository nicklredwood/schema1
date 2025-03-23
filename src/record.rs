use uuid::Uuid;
use serde::{Serialize, Deserialize};

// TODO: wrap `Uuid::new_v5()` in a higher-level function. Perhaps a macro?

use schema::{Namespace, UuidEncode};

pub mod artist;
pub mod track;
pub mod album;

// TODO: write a match-case macro to denote "no namespace"; derived from
// `uuid::Uuid::max()`
// #[namespace]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Record<T>
  where T: UuidEncode {
	uuid: Uuid, // derived from T.encode()
	content: T,
}

impl<T> Record<T>
  where T: UuidEncode {
	pub(crate) fn from(record: T) -> Record<T> {
		Record { uuid: record.encode(), content: record }
	}
}

// /// Still not sure how this is going to work
// pub(crate) trait UuidDecode {
// 	fn decode(uuid: Uuid) -> Self;
// }

// TODO: auto-implement traits
// pub(crate) trait UuidTranscode: UuidEncode + UuidDecode
