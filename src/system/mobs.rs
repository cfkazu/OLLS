use crate::prelude::*;
use serde::Deserialize;
#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Copy,Deserialize)]
pub enum MobType{
    #[default]Neutral,
    Hostile,
    Ally
}
pub fn mobs_move(
    mut commands: Commands
    ,mut map: ResMut<Map>
    ,mut mob_positions: Query<(Entity, &mut Position),With<Mob>>
){
    for (entity,position) in mob_positions.iter_mut(){
        let mut rng = RandomNumberGenerator::new();
        let mut new_position = position.clone();
        new_position = match rng.range(0,4){
            0 => Position{x:new_position.x,y:new_position.y+1},
            1 => Position{x:new_position.x,y:new_position.y-1},
            2 => Position{x:new_position.x+1,y:new_position.y},
            3 => Position{x:new_position.x-1,y:new_position.y},
            _ => Position{x:new_position.x,y:new_position.y},
        };
        if new_position != *position{
            commands.spawn(WantsToMove{entity: entity, destination: new_position});
        }
    }
}
pub fn spawn_mobs(
    mut map: ResMut<Map>,
    mut commands: Commands,
    atlas: Res<MobAsset>,
){
    let mut entities = Vec::new();
    let mut positions = Vec::new();
    for (position,mob) in map.mob_starts.iter(){
        let mut sprite = TextureAtlasSprite::new(mob.index);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        let entity = commands
            .spawn((
                SpriteSheetBundle{
                    sprite: sprite,
                    texture_atlas: atlas.atlas.clone(),
                    //transform: (*position).into(),
                    transform:(*position).to_transform(11.0),
                    ..Default::default()
                },
                *mob,
                Position { x: position.x, y: position.y },
            )).id();
        entities.push(entity);
        positions.push(*position);
    }
    map.entities_occupy_tiles(entities, positions)
}