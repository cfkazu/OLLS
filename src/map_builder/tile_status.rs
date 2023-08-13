use crate::prelude::*;
use ron::de::from_reader;
use std::fs::File;
impl TileStatusList {
    pub fn load(filename: &str) -> Self {
        let file = File::open(format!("assets/map/tileinfo_{}.ron", filename))
            .expect("Failed to open spawn templates file");
        from_reader(file).expect("Failed to load spawn templates")
    }
}
pub fn setup(mut commands: Commands) {
    let tile_status_list = TileStatusList::load("default");
    println!("tile_status setup:{:?}", tile_status_list);
    commands.insert_resource(tile_status_list);
}
