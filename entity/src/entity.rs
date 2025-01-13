use crate::health::HealthLog;

/// Entity is a unit in game. A teapot is an entity if the user can break the teapot.
/// The user is an Entity, enemies are entities...
/// If there is a reason to give health points to anything, this is an Entity.
pub trait Entity {
    // Status
    fn get_current_hp(&self) -> &u32;

    fn get_max_hp(&self) -> &u32;

    // Interactions
    fn gain_hp(&mut self, amount: u32) -> Option<HealthLog>;

    fn loose_hp(&mut self, amount: u32);
}

pub trait EntityChangeMaxHealthCapacity {
    fn set_max_hp(&mut self, new_max_hp: u32);
}
