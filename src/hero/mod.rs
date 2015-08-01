mod superman;
mod knight;
pub mod hero;

use self::superman::Superman;
use self::knight::Knight;
use count::Counting;

pub struct Builder<'a, T: 'a> {
	count: &'a mut T
}

impl <'a, T: 'a> Builder<'a, T> where T: Counting {

	pub fn new(count: &'a mut T) -> Builder<'a, T>  {
		Builder{
			count: count
		}
	}
	
	pub fn superman(&mut self, health: u32, damage: u32) -> Option<Superman> {
		
		match self.count.next() {
			None => None,
			Some(_) => {
				Some(Superman::new(health, damage))
			}
		}

	}
	
	pub fn knight(&mut self, health: u32, damage: u32) -> Option<Knight> {
	
		match self.count.next() {
			None => None,
			Some(_) => {
				Some(Knight::new(health, damage))
			}
		}

	}
	
}