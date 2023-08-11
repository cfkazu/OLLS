use crate::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Player;

#[derive(Component,Clone, Copy)]
pub struct Mob{
   pub mob_type: MobType,
   pub index: usize
}
#[derive(Component, Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}
#[derive(Component, Clone)]
pub struct Naming(pub String);
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32
}
#[derive(Component, Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}
#[derive(Component)]
pub struct Damage(pub i32);

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

#[derive(Component)]
pub struct SleepDesire{
    pub current: i32,
    pub max: i32
}
#[derive(Component)]
pub struct Hunger{
    pub current: i32,
    pub max: i32
}

#[derive(Component)]
pub struct Thirth{
    pub current: i32,
    pub max: i32
}

#[derive(Component,Clone,Copy,Eq,PartialEq,PartialOrd,Debug)]
pub struct Time{
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}
impl Time{
    pub fn resolve_time(&mut self){
        if self.minute >= 60{
            self.minute = 0;
            self.hour += 1;
        }
        if self.hour >= 24{
            self.hour = 0;
            self.day += 1;
        }
        if self.day >= 30{
            self.day = 0;
            self.month += 1;
        }
        if self.month >= 12{
            self.month = 0;
            self.year += 1;
        }
    }
}
impl Ord for Time{
    fn cmp(&self, other:&Self) -> Ordering {
        if self.year > other.year{
            return Ordering::Greater;
        }else if self.year < other.year{
            return Ordering::Less;
        }
        if self.month > other.month{
            return Ordering::Greater;
        }else if self.month < other.month{
            return Ordering::Less;
        }
        if self.day > other.day{
            return Ordering::Greater;
        }else if self.day < other.day{
            return Ordering::Less;
        }
        if self.hour > other.hour{
            return Ordering::Greater;
        }else if self.hour < other.hour{
            return Ordering::Less;
        }
        if self.minute > other.minute{
            return Ordering::Greater;
        }else if self.minute < other.minute{
            return Ordering::Less;
        }
        return Ordering::Equal;

    }
}
#[derive(Component,Resource,Eq,PartialEq,Ord,PartialOrd,Clone,Copy)]
pub struct CurrentTime{
    pub time: Time
}
#[derive(Component,Eq,PartialEq)]
pub struct WantATurn{
    pub time:Time,
    pub character: Entity,
    pub before_time:Time,
}
impl PartialOrd for WantATurn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        
            if self.time > other.time{
               // return Ordering::Greater;
                return Some(Ordering::Less);
                //Binary_Heap用に逆順にする
            }else if self.time < other.time{
                //return Ordering::Less;
                return Some(Ordering::Greater);
            }
            if self.character > other.character{
               // return Ordering::Greater;
                return Some(Ordering::Less);
            }else if self.character < other.character{
                //return Ordering::Less;
                return Some(Ordering::Greater);
            }
            return Some(Ordering::Equal);
       
    }
}
impl Ord for WantATurn {
    fn cmp(&self, other:&Self) -> Ordering {
        if self.time > other.time{
           // return Ordering::Greater;
            return Ordering::Less;
            //Binary_Heap用に逆順にする
        }else if self.time < other.time{
            //return Ordering::Less;
            return Ordering::Greater;
        }
        if self.character > other.character{
           // return Ordering::Greater;
            return Ordering::Less;
        }else if self.character < other.character{
            //return Ordering::Less;
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}
#[derive(Resource,Component)]
pub struct TurnQueue{
    pub queue: BinaryHeap<WantATurn>
}
#[derive(Component)]
pub struct GetATurn{
    pub current_time: Time,
    pub before_time: Time,
}