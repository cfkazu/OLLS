use crate::prelude::*;
use rustc_hash::FxHasher;
type Hasher = BuildHasherDefault<FxHasher>;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Neutral;
#[derive(Component)]
pub struct Ally;

#[derive(Component, Clone, Copy)]
pub struct MoveType {
    pub move_id: MoveStrategy,
}
#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Copy, Deserialize)]
pub enum MoveStrategy {
    #[default]
    Random,
    Chase,
}
#[derive(Component, Clone, Copy)]
pub struct Mob {
    pub mob_type: MobType,
    pub index: usize,
}
#[derive(Component, Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}
#[derive(Component, Clone)]
pub struct Naming(pub String);
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
#[derive(Component, Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }
}

#[derive(Component)]
pub struct SleepDesire {
    pub current: i32,
    pub max: i32,
}
#[derive(Component)]
pub struct Hunger {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Thirth {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Copy, Eq, PartialEq, PartialOrd, Debug)]
pub struct Time {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}
impl Time {
    pub fn resolve_time(&mut self) {
        if self.second >= 60 {
            self.second = 0;
            self.minute += 1;
        }
        if self.minute >= 60 {
            self.minute = 0;
            self.hour += 1;
        }
        if self.hour >= 24 {
            self.hour = 0;
            self.day += 1;
        }
        if self.day >= 30 {
            self.day = 0;
            self.month += 1;
        }
        if self.month >= 12 {
            self.month = 0;
            self.year += 1;
        }
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year > other.year {
            return Ordering::Greater;
        } else if self.year < other.year {
            return Ordering::Less;
        }
        if self.month > other.month {
            return Ordering::Greater;
        } else if self.month < other.month {
            return Ordering::Less;
        }
        if self.day > other.day {
            return Ordering::Greater;
        } else if self.day < other.day {
            return Ordering::Less;
        }
        if self.hour > other.hour {
            return Ordering::Greater;
        } else if self.hour < other.hour {
            return Ordering::Less;
        }
        if self.minute > other.minute {
            return Ordering::Greater;
        } else if self.minute < other.minute {
            return Ordering::Less;
        }

        if self.second > other.second {
            return Ordering::Greater;
        } else if self.second < other.second {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}
#[derive(Component, Resource, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct CurrentTime {
    pub time: Time,
}
#[derive(Component, Eq, PartialEq)]
pub struct WantATurn {
    pub time: Time,
    pub character: Entity,
    pub before_time: Time,
}
impl PartialOrd for WantATurn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time > other.time {
            // return Ordering::Greater;
            return Some(Ordering::Less);
            //Binary_Heap用に逆順にする
        } else if self.time < other.time {
            //return Ordering::Less;
            return Some(Ordering::Greater);
        }
        if self.character > other.character {
            // return Ordering::Greater;
            return Some(Ordering::Less);
        } else if self.character < other.character {
            //return Ordering::Less;
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
    }
}
impl Ord for WantATurn {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time > other.time {
            // return Ordering::Greater;
            return Ordering::Less;
            //Binary_Heap用に逆順にする
        } else if self.time < other.time {
            //return Ordering::Less;
            return Ordering::Greater;
        }
        if self.character > other.character {
            // return Ordering::Greater;
            return Ordering::Less;
        } else if self.character < other.character {
            //return Ordering::Less;
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}
#[derive(Resource, Component)]
pub struct TurnQueue {
    pub queue: BinaryHeap<WantATurn>,
}
#[derive(Component)]
pub struct GetATurn {
    pub current_time: Time,
    pub before_time: Time,
}
#[derive(Clone, Deserialize, Debug, Default)]
pub struct TileStatus {
    pub name: String,
    pub can_pass: bool,
}
#[derive(Clone, Deserialize, Debug, Default, Resource)]
pub struct TileStatusList {
    pub tile_status_list: HashMap<usize, TileStatus, Hasher>,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct MobStatus {
    pub mob_type: MobType,
    pub name: String,
    pub drop_items: Option<Vec<(String, i32)>>,
    pub description: Option<String>,
    pub hp: Option<i32>,
    pub index: usize,
    pub base_damage: Option<i32>,
    pub hunger: Option<i32>,
    pub sleep: Option<i32>,
    pub required_time: Option<i32>,

    pub move_id: Option<MoveStrategy>,
    pub occupy_tile: Option<bool>,
    //それぞれのMobの種類に特徴的な値(植物の成長度など)
    pub additional_status: Option<Vec<i32>>,
}

#[derive(Clone, Deserialize, Debug, Default, Resource)]
pub struct MobStatusList {
    pub mob_status_list: HashMap<usize, MobStatus, Hasher>,
}

#[derive(Debug, Clone, PartialEq, Eq, Component, Copy, Deserialize)]
pub struct RequiredTime {
    pub time: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Component, Copy, Deserialize)]
pub struct Plant {
    pub growth: i32,
    pub growth_rate: i32,
}
#[derive(Clone,Component)]
pub struct ToDie{
    pub entity:Entity,
    pub position:Position,
    pub death_reason:String,
}

#[derive(Clone,Component)]
pub struct Bullet{
    pub damage:i32,
    pub transformed_transform:Transform,
    pub target_transform:Transform,
    pub angle:f32,
}