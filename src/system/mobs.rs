use crate::prelude::*;
use serde::Deserialize;
#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Copy,Deserialize)]
pub enum MobType{
    #[default]Neutral,
    Hostile,
    Ally,
    Item
}
pub fn mobs_move_by_time(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut mob_positions: Query<(Entity, &mut Position,&RequiredTime),(With<Mob>,With<GetATurn>)>,
    current_time: Res<CurrentTime>,
    mut turn_queue:ResMut<TurnQueue>,
){
    
    for (entity,position,required_time) in mob_positions.iter_mut(){
        
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

        let mut next_time = current_time.clone();
        next_time.time.second += required_time.time;
        next_time.time.resolve_time();
        turn_queue.queue.push(WantATurn {
             time: next_time.time, 
             character: entity, 
             before_time: current_time.time.clone() });
        
        commands.entity(entity).remove::<GetATurn>();
        
    }

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
    current_time:Res<CurrentTime>,
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
                GetATurn{current_time:current_time.time.clone(),before_time:current_time.time.clone()},
            )).id();
        entities.push(entity);
        positions.push(*position);
    }
    map.entities_occupy_tiles(entities, positions)
}