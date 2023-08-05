use crate::prelude::*;
use std::collections::HashSet;
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob{
   pub mob_type: MobType,
   pub index: usize
}
#[derive(Component, Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}
#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty:bool
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }
}