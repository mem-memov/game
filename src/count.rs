#[derive(Debug)]
pub struct Count {
	limit: u32,
	current: u32
}

pub trait Counting {
	fn next(&mut self) -> Option<u32>;
}


impl Count {
	pub fn new(limit: u32) -> Count {
		Count{
			limit: limit,
			current: 0
		}
	}
}

impl Counting for Count {
	fn next(&mut self) -> Option<u32> {
		if self.current == self.limit {
			None
		} else {
			self.current = self.current + 1;
			Some(self.current)
		}
	}
}

impl <'a> Counting for &'a mut Count {
	fn next(&mut self) -> Option<u32> {
		Count::next(*self)
	}
}

