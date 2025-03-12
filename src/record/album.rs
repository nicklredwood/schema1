use serde::{Serialize, Deserialize};

use super::track::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Album {
	tracks: Vec<Track>,
}
