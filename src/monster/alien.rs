use monster::monster::Monster;
use hero::hero::Hero;

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
		println!("Alien: I'm attacking.");
		let health = enemy.suffer(self.damage);
		self.health = self.health + health;
		println!("Alien: I've become stronger by {} points.", health);
	}
	
	fn suffer(&mut self, damage: u32) -> u32 {
	
		println!("Alien: I'm being atacked. Losing health");
		
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

