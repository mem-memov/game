use monster::monster::Monster;

pub trait Hero {
    fn attack<T: Monster>(&mut self, enemy: &mut T);
	fn suffer(&mut self, damage: u32) -> u32;
}