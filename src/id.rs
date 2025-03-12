use uuid::Uuid;

pub(crate) trait UuidEncode {
	fn encode(&self) -> Uuid;
}

pub(crate) fn UuidEncode() -> Uuid {
}
