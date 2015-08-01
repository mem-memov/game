use monster::monster::Monster;
use hero::hero::Hero;

#[derive(Debug)]
pub struct Alien {
  health: u32,
  damage: u32
}

impl Alien {
	pub fn new(health: u32, damage: u32) -> Alien {
		Alien{
			health: health, 
			damage: damage
		}
	}
}

impl Monster for Alien {

	fn attack<T: Hero>(&mut self, enemy: &mut T) {
		println!("I'm an alien. I'm attacking.");
		let health = enemy.suffer(self.damage);
		self.health = self.health + health;
		println!("I'm the alien. I've become stronger by {} points.", health);
	}
	
	fn suffer(&mut self, damage: u32) -> u32 {
	
		println!("I'm an alien. I'm being atacked. Losing health");
		
		if self.health > damage {
		
			self.health = self.health - damage;
			damage
			
		} else {
		
			let real_damage = self.health;
			self.health = 0;
			real_damage
		
		}
		
	}
	
}

