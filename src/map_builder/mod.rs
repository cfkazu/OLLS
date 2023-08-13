mod map;
pub use map::*;
mod tile_status;
use crate::prelude::*;
pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(Startup,tile_status::setup)
        ;
    }
}