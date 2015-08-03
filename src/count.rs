use std::cell::Cell;

pub struct Count {
	limit: u32,
	current: Cell<u32>
}

pub trait Counting {
	fn next(&self) -> Option<u32>;
}


impl Count {
	pub fn new(limit: u32) -> Count {
		Count{
			limit: limit,
			current: Cell::new(0)
		}
	}
}

impl Counting for Count {

	fn next(&self) -> Option<u32> {
	
		let mut current = self.current.get();
	
		if current == self.limit {
			None
		} else {
			current = current + 1;
			self.current.set(current);
			Some(current)
		}
		
	}
}





