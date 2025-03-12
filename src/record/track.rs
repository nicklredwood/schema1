use serde::{Serialize, Deserialize};

use super::artist::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Track {
	artist: Artist,
	name: String,
}
