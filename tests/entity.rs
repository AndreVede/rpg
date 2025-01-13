use entity::{
    entity::{Entity, EntityChangeMaxHealthCapacity},
    health::Health,
};
use entity_proc_macro::{entity, Entity, EntityChangeMaxHealthCapacity};

use entity::health::HealthError;

#[derive(Entity, Debug, PartialEq)]
#[entity()]
struct Teapot {}

impl Teapot {
    pub fn new(hp_max: u32, hp: Option<u32>) -> Result<Teapot, HealthError> {
        Ok(Teapot {
            hp: Health::new(hp_max, hp)?,
        })
    }
}

#[derive(Entity, EntityChangeMaxHealthCapacity, Debug, PartialEq)]
#[entity()]
struct TeapotInvolved {}

impl TeapotInvolved {
    pub fn new(hp_max: u32, hp: Option<u32>) -> Result<TeapotInvolved, HealthError> {
        Ok(TeapotInvolved {
            hp: Health::new(hp_max, hp)?,
        })
    }
}

#[test]
fn test_init() {
    let teapot = Teapot::new(1u32, None).unwrap();

    let hero = Teapot::new(10_000u32, Some(8_000u32)).unwrap();

    let buged_entity = Teapot::new(10u32, Some(12u32)).unwrap_err();

    assert_eq!(teapot.get_current_hp(), &1u32);
    assert_eq!(hero.get_current_hp(), &8_000u32);
    assert_eq!(buged_entity, HealthError::LevelTooHigh)
}

#[test]
fn test_gain_hp() {
    let mut teapot = Teapot::new(5u32, Some(0u32)).unwrap();

    // more than the max
    teapot.gain_hp(6u32);
    assert_eq!(teapot.get_current_hp(), &5u32);
}

#[test]
fn test_loose_hp() {
    let mut teapot = Teapot::new(1u32, None).unwrap();

    // more than possible
    teapot.loose_hp(10u32);
    assert_eq!(teapot.get_current_hp(), &0u32);
}

#[test]
fn test_change_max() {
    let mut teapot_involved = TeapotInvolved::new(1u32, None).unwrap();

    assert_eq!(teapot_involved.get_current_hp(), &1u32);
    assert_eq!(teapot_involved.get_max_hp(), &1u32);

    teapot_involved.set_max_hp(8u32);

    assert_eq!(teapot_involved.get_current_hp(), &8u32);
    assert_eq!(teapot_involved.get_max_hp(), &8u32);

    teapot_involved.set_max_hp(7u32);

    assert_eq!(teapot_involved.get_current_hp(), &7u32);
    assert_eq!(teapot_involved.get_max_hp(), &7u32);
}
