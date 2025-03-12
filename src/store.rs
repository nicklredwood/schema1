// fn add(store: Store, record: Record) -> Result<()> {
// 	// take the argument
// 	// add it to the store
// 	Ok(())
// }

/// Represents the to-be-realized store. Writes to disk ("realizes") upon [ Drop | Deref | idk ]
struct Store;

trait Realize {
    fn realize(&self);
}

impl Realize for Store {
    fn realize(&self) {
        todo!()
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        Store::realize(&self);
    }
}

// use std::fs::File;

