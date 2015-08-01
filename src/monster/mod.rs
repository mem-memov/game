mod alien;
mod zomby;
pub mod monster;

pub struct Builder {
	boost: u32
}

use self::alien::Alien;
use self::zomby::Zomby;

impl Builder {

	pub fn new(boost: u32) -> Builder {
		Builder{
			boost: boost
		}
	}
	
	pub fn alien(&self, health: u32, damage: u32) -> Alien {
		Alien::new(
			health, 
			&self.boost * damage
		)
	}
	
	pub fn zomby(&self, health: u32, damage: u32) -> Zomby {
		Zomby::new(
			health, 
			&self.boost * damage
		)
	}
	
}

