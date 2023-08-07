use bevy::{window::WindowTheme, transform::commands};

use crate::prelude::*;
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Glass1,
    Glass2,
    Glass3,
    Glass4,
    Glass5,
    Glass6,
    Glass7,
    Glass8,
    Glass9,
    Glass10,
    Glass11,
    GlassRock,
    Ground1,
    Ground2,
    Void1,
    Void2,
    Sand1,
    Sand2,
    Sand3,
    Sand4,
    SandGlass1,
    SandGlass2,
    SandGlass3,
    WhiteSand1,
    WhiteSand2,
    WhiteSand3,
    WhiteSand4,
    WhiteSand5,
    WhiteSand6,
}
#[derive(Resource)]
pub struct Map{
    pub width:i32,
    pub height:i32,
    pub tiles:Vec<TileType>,
    pub occupation: Vec<Option<Entity>>,
    pub player_start:Position,
    pub mob_starts:Vec<(Position,Mob)>,
   // pub mob_positions:Vec<Position>,
    pub file_name:String,
}
impl Map{
    pub fn new(width:i32,height:i32) -> Self{
        Self{
            width,
            height,
            tiles:vec![TileType::Glass1;(width*height) as usize],
            occupation:vec![None;(width*height) as usize],
            player_start:Position::new(1,2),
            mob_starts:Vec::new(),
           // mob_positions:Vec::new(),
            file_name:String::new(),
        }
    }

    pub fn in_bounds<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0 && position.x < self.width as i32
            && position.y >= 0 && position.y < self.height as i32
    }
    pub fn map_idx(&self,x: i32, y: i32) -> usize {
        ((y*self.width) + x) as usize
    }
    pub fn is_tile_occupied<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position)
            && self.occupation[self.map_idx(position.x, position.y)] == None
    }
    pub fn try_idx(&self,position:Position)->Option<usize>{
        if self.in_bounds(position){
            Some(self.map_idx(position.x,position.y))
        }else{
            None
        }
    }
    pub fn testmap() -> Self{
        let my_tile:Vec<TileType> =  vec![TileType::GlassRock,TileType::GlassRock,TileType::GlassRock,TileType::GlassRock,TileType::GlassRock,
                                          TileType::GlassRock,TileType::Glass1,TileType::Glass1,TileType::Glass1,TileType::GlassRock,
                                          TileType::GlassRock,TileType::Glass1,TileType::Glass1,TileType::Glass1,TileType::GlassRock,
                                          TileType::GlassRock,TileType::Glass1,TileType::Glass1,TileType::Glass1,TileType::GlassRock,
                                          TileType::GlassRock,TileType::GlassRock,TileType::GlassRock,TileType::GlassRock,TileType::GlassRock];
        Self{
            width:5,
            height:5,
            tiles:my_tile,
            occupation:vec![None;25],
            player_start:Position::new(2,1),
            mob_starts:vec![(Position::new(2,3),Mob{mob_type:MobType::Neutral,index:5}),
                            (Position::new(3,3),Mob{mob_type:MobType::Neutral,index:6}),
                            (Position::new(2,2),Mob{mob_type:MobType::Neutral,index:7}),
                            (Position::new(3,2),Mob{mob_type:MobType::Neutral,index:8})
                            ],
            //mob_positions:vec![Position::new(2,3),Position::new(3,3)],
            file_name:String::from("test"),
        }
    }
    pub fn can_enter_tile<T:Into<Position>>(&self,position: T)->bool{
        let position = position.into();
        self.in_bounds(position) 
        && self.tiles[self.map_idx(position.x, position.y)] == TileType::Glass1 
        && self.occupation[self.map_idx(position.x, position.y)] == None
    }

    pub fn entity_occupy_tile(&mut self,entity:Entity,position:Position){
        if let Some(idx) = self.try_idx(position){
            self.occupation[idx] = Some(entity);
        }
    }
    pub fn entities_occupy_tiles(&mut self,entities:Vec<Entity>,positions:Vec<Position>){
        for (entity,position) in entities.iter().zip(positions.iter()){
            self.entity_occupy_tile(*entity,*position);
        }
    }
    pub fn move_entity_occupation(&mut self, entity: Entity, old_p: Position, new_p: Position) {
        let old_idx = self.map_idx(old_p.x, old_p.y);
        let new_idx = self.map_idx(new_p.x, new_p.y);
        self.occupation[old_idx] = None;
        self.occupation[new_idx] = Some(entity);
    } 

}
pub fn spawn_map_tiles(
    map: Res<Map>,
    mut commands: Commands,
    atlas: Res<MapAsset>,
){
    for y in 0..map.height{
        for x in 0..map.width{
            let tile = map.tiles[map.map_idx(x,y)];
            let position = Position::new(x,y);
            let mut sprite = TextureAtlasSprite::new(tile as usize);
            sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
            //sprite.color = Color::OLIVE;
            commands.spawn((
                SpriteSheetBundle{
                    sprite: sprite,
                    texture_atlas: atlas.atlas.clone(),
                    transform: position.into(),
                    ..Default::default()
                },
            ));
        }
    }
}