use crate::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MobType{
    #[default]Neutral,
    Hostile,
    Ally
}