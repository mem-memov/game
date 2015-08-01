use hero::hero::Hero;
use monster::monster::Monster;

#[derive(Debug)]
pub struct Zomby {
  health: u32,
  damage: u32
}

impl Zomby {
	pub fn new(health: u32, damage: u32) -> Zomby {
		Zomby{
			health: health, 
			damage: damage
		}
	}
}

impl Monster for Zomby {

	fn attack<T: Hero>(&mut self, enemy: &mut T) {
		println!("I'm a zomby. I'm attacking.");
		let health = enemy.suffer(self.damage);
		self.health = self.health + health;
		println!("I'm the zomby. I've become stronger by {} points.", health);
	}
	
	fn suffer(&mut self, damage: u32) -> u32 {
	
		println!("I'm a zomby. I'm being atacked. Losing health");
		
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