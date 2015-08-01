use hero::hero::Hero;
use monster::monster::Monster;

#[derive(Debug)]
pub struct Knight {
  health: u32,
  damage: u32
}

impl Knight {
	pub fn new(health: u32, damage: u32) -> Knight {
		Knight{
			health: health, 
			damage: damage
		}
	}
}

impl Hero for Knight {

	fn attack<T: Monster>(&mut self, enemy: &mut T) {
		println!("I'm a knight. I'm attacking a monster.");
		let health = enemy.suffer(self.damage);
		self.health = self.health + health;
		println!("I'm the knight. I've become stronger by {} points.", health);
	}
	
	fn suffer(&mut self, damage: u32) -> u32 {
	
		println!("I'm a knight. I'm being atacked. Losing health");
		
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