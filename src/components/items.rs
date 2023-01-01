use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amout: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

