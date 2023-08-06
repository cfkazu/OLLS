use crate::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Copy)]
pub enum MobType{
    #[default]Neutral,
    Hostile,
    Ally
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