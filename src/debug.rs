use std::fmt::Debug;

#[derive(Debug)]
struct FormatContainer<T>
  where T: Debug {
	data: T,
	format: String,
}

impl<T> FormatContainer<T>
  where T: Debug + Copy {
	fn from(&data: &T) -> FormatContainer<T> {
		FormatContainer {
			data: data.clone(),
			format: format!("{:?}", data)
		}
	}
}

impl<T> Drop for FormatContainer<T>
  where T: Debug {
	fn drop(&mut self) {
		println!("{:?}", self);
	}
}
