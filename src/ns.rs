use uuid::Uuid;

use crate::id;
use crate::record::Record;

pub(crate) trait Namespace {
	const NS: Uuid;
}

impl<T> Namespace for Record<T>
  where T: id::UuidEncode {
	const NS: Uuid = Uuid::max();
		// Uuid::new_v5(&Uuid::max(), Record::serialize());
}
