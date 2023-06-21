use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct list_accesses {
	accesses: Vec<usize>,
}

impl list_accesses {
    pub fn new() -> Self {
        list_accesses { accesses: Vec::new() }
    }

	pub fn add(&mut self, value: usize) {
		self.accesses.push(value);
	}
}

