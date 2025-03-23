use uuid::Uuid;
pub use derive::*;

// use crate::record::Record;

// // // #[namespace]
// // #[derive(Namespace)]
// impl<T> Namespace for Record<T>
//   where T: UuidEncode {
// 	const NS: Uuid = Uuid::max();
// 		// Uuid::new_v5(&Uuid::max(), Record::serialize());
// }

pub trait UuidEncode {
	fn encode(&self) -> Uuid;
}

pub trait UuidDecode {} // TODO: actually define this

// schema/derive/
pub trait Identify {}

#[cfg(test)]
mod tests {
	// #[macro_use(crate::*)]
	use crate::*;
	use derive::*;

	#[namespace]
	struct Test {
		foo: u8,
		bar: String,
	}

	#[namespace(Test)]
	struct Test2 {
		foo: u8,
		bar: String,
		baz: f64,
	}

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
