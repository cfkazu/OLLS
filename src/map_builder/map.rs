use bevy::{window::WindowTheme, transform::commands};

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use crate::prelude::*;
use tiled::{Loader, TileLayer, FiniteTileLayer, FiniteTileLayerData};
#[derive(Copy, Clone, PartialEq,FromPrimitive)]
#[repr(usize)]
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
    Road1,
    HayTate,
    HayYoko,
    Void3,

    DarkGrass1,
    DarkGrass2,
    DarkGrass3,
    DarkGrass4,
    Wasteland,
    DraySoil1,
    DraySoil2,
    DraySoil3,
    DraySoil4,
    DraySoil5,
    Concrete,
    Green1,
    Snow1,
    Snow2,
    Snow3,
    Snow4,
    Snow5,
    Snow6,
    Snow7,
    Snow8,
    SnowHayTate,
    SnowHayYoko,
    SnowLake,
    Snow9,
    Snow10,
    SnowTree1,
    SnowTree2,
    SnowBlueTile,
    SnowRock1,
    SnowRock2,
    SnowRock3,
    SnowRock4,
    SnowStair,

    Heat1,
    Heat2,
    Heat3,
    IronTable,
    WoodenFloor1,
    WoodenFloor2,
    WoodenFloor3,
    HayFloor1,
    HayFloor2,
    HayFloor3,
    BrownConcrete,
    Wara,
    HibiTile,
    BrueTile,
    Icetile,
    RoadTile,
    SnowFlower1,
    SnowFlower2,
    SnowFlower3,
    SnowGrass1,
    SnowGrass2,
    SnowGrass3,
    SnowGrass4,
    SnowRock5,
    SnowRock6,
    Snow11,
    Snow12,
    Snow13,
    FlowerGarden1,
    FlowerGarden2,
    FlowerGarden3,
    Void4,
    SnowPillar,

    BlownFloor,
    SilverFloor,
    BronzeFloor,
    BlueFloor,
    RockFloor1,
    RockFloor2,
    RockFloor3,
    RockFloor4,
    RockFloor5,
    RockFloor6,
    RockFloor7,
    RockFloor8,
    RockFloor9,
    RockFloor10,
    RockFloor11,
    Ido,
    Roof1,
    WhiteFloor,
    BronzeFloor2,
    BronzeFloor3,
    RockFloor12,
    RockFloor13,
    RockFloor14,
    RockFloor15,
    KasekiFloor1,
    KasekiFloor2,
    KasekiFloor3,
    KasekiFloor4,
    KasekiFloor5,
    KasekiFloor6,
    KasekiFloor7,
    KasekiFloor8,
    KasekiFloor9,


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
        //&& self.tiles[self.map_idx(position.x, position.y)] == TileType::Glass1 
        && self.tiles[self.map_idx(position.x, position.y)] != TileType::Heat3
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
    pub fn free_occupy_tile(&mut self,position:Position){
        if let Some(idx) = self.try_idx(position){
            self.occupation[idx] = None;
        }
    }
    pub fn load(filename:&str)->Self{
        let mut loader = Loader::new();
        let map = loader.load_tmx_map(format!("assets/map/{}.tmx",filename));
        //println!("map:{:?}",map);
       

        if let Ok(map) = map{
            let mut my_tile:Vec<TileType> = Vec::new();
            let layer = map.get_layer(0).unwrap().as_tile_layer().unwrap();
            if let tiled::TileLayer::Finite(tiles) = layer{
                for y in (0..layer.height().unwrap()).rev(){
                    for x in (0..layer.width().unwrap()){
                        if let Some(t) = tiles.get_tile(x as i32,y as i32) {
                            my_tile.push(FromPrimitive::from_u32(t.id()).unwrap());
                        }
                    }
                }
            }

            Self{
                width:map.width as i32,
                height:map.height as i32,
                tiles:my_tile,
                occupation:vec![None;(map.width*map.height) as usize],
                player_start:Position::new(15,15),
                mob_starts:Vec::new(),
                //mob_positions:Vec::new(),
                file_name:filename.to_string(),

            }
        }else{
            panic!("Map not found");
        }
        
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