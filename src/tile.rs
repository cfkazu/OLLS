use bevy::render::color;

use crate::prelude::*;
pub const TILE_SIZE: f32 = 16.0;
pub const SIDE_LENGTH: usize = 4;

#[derive(Component)]
struct Tile(u64);
#[derive(Debug, Clone, PartialEq, Eq, Component,Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn to_transform(&self,z:f32)->Transform{
        let x = (self.x as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
        let y = (self.y as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
        Transform::from_xyz(x, y, z)
    }
}
impl From<Position> for Transform{
    fn from(pos:Position)->Self{
        pos.to_transform(10.0)
    }
}
pub fn create_tile(commands:&mut Commands,num:u64,position:Position,atlas:&Res<MapAsset>){
    let mut sprite = TextureAtlasSprite::new(37);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    sprite.color = Color::OLIVE;
    commands.spawn((
        SpriteSheetBundle{
            sprite: sprite,
            texture_atlas: atlas.atlas.clone(),
            transform: position.into(),
            ..Default::default()
        },
    ));
   

}
