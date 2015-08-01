use hero::hero::Hero;

pub trait Monster {
    fn attack<T: Hero>(&mut self, enemy: &mut T);
	fn suffer(&mut self, damage: u32) -> u32;
}