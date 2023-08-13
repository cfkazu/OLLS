use crate::prelude::*;
use ron::de::from_reader;
use std::fs::File;
impl MobStatusList {
    pub fn load(filename: &str) -> Self {
        let file = File::open(format!("assets/mob/template_{}.ron", filename))
            .expect("Failed to open spawn templates file");
        from_reader(file).expect("Failed to load spawn templates")
    }
}
pub fn setup(mut commands: Commands) {
    let mob_status_list = MobStatusList::load("default");
    println!("mob_status setup:{:?}", mob_status_list);
    commands.insert_resource(mob_status_list);
}
