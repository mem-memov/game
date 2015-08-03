mod alien;
mod zomby;
pub mod monster;

use self::alien::Alien;
use self::zomby::Zomby;
use count::Counting;

pub struct Builder<'a, T: 'a> {
	count: &'a T
}

impl <'a, T: 'a> Builder<'a, T> where T: Counting {

	pub fn new(count: &'a T) -> Builder<'a, T>  {
		Builder{
			count: count
		}
	}
	
	pub fn alien(&mut self, health: u32, damage: u32) -> Option<Alien> {

		match self.count.next() {
			None => None,
			Some(_) => {
				Some(Alien::new(health, damage))
			}
		}

	}

	pub fn zomby(&mut self, health: u32, damage: u32) -> Option<Zomby> {

		match self.count.next() {
			None => None,
			Some(_) => {
				Some(Zomby::new(health, damage))
			}
		}

	}

}

