use std::fmt::Display;

use health::{Health, HealthError, HealthLog};

pub mod health;

/// Entity is a unit in game. A teapot is an entity if the user can break the teapot.
/// The user is an Entity, enemies are entities...
/// If there is a reason to give health points to anything, this is an Entity.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    hp: Health,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EntityError {
    HeathError(HealthError),
}

impl Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            EntityError::HeathError(err) => err.to_string(),
        };

        write!(f, "{}", error)
    }
}

#[derive(Debug, PartialEq)]
pub enum EntityLog {
    Hp(HealthLog),
}

impl Display for EntityLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log = match self {
            EntityLog::Hp(hp_log) => hp_log.to_string(),
        };

        write!(f, "{}", log)
    }
}

impl Entity {
    pub fn new(max_health: u32, health: Option<u32>) -> Result<Entity, EntityError> {
        let hp = Health::new(max_health, health).map_err(EntityError::HeathError)?;

        Ok(Entity { hp })
    }

    // Status

    pub fn get_current_hp(&self) -> &u32 {
        self.hp.get_level()
    }

    pub fn get_max_hp(&self) -> &u32 {
        self.hp.get_max()
    }

    // Interactions

    pub fn gain_hp(&mut self, amount: u32) -> Option<EntityLog> {
        self.hp.increase(amount).map(EntityLog::Hp)
    }

    pub fn lose_hp(&mut self, amount: u32) {
        self.hp.decrease(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let teapot = Entity::new(1u32, None).unwrap();

        let hero = Entity::new(10_000u32, Some(8_000u32)).unwrap();

        let buged_entity = Entity::new(10u32, Some(12u32)).unwrap_err();

        assert_eq!(teapot.get_current_hp(), &1u32);
        assert_eq!(hero.get_current_hp(), &8_000u32);
        assert_eq!(
            buged_entity,
            EntityError::HeathError(HealthError::LevelTooHigh)
        )
    }
}
