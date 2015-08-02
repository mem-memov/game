extern crate game;

use game::count::Count;
use game::hero::hero::Hero;
use game::monster::monster::Monster;

fn main() {

	let mut count = Count::new(2);

	let mut monster_builder = game::monster::Builder::new(&mut count); // <-- Dependency Injection
	let mut monster = match monster_builder.alien(50, 10) {
		None => panic!("No monsters left"),
		Some(monster) => monster
	};
	
	let mut hero_builder = game::hero::Builder::new(&mut count); // <-- Dependency Injection
	let mut hero1 = match hero_builder.superman(150, 20) {
		None => panic!("No heros left"),
		Some(hero) => hero
	};

	let mut hero2 = match hero_builder.knight(50, 30) {
		None => panic!("No heros left"),
		Some(hero) => hero
	};
/*  
  //println!("This monster is an {:?}", monster);
  //println!("This hero is an {:?}", hero1);

	hero1.attack(&mut monster);
	monster.attack(&mut hero1);
	
	hero2.attack(&mut monster);
	*/
	
}