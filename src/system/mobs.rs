use crate::prelude::*;
use serde::Deserialize;
#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Copy, Deserialize)]
pub enum MobType {
    #[default]
    Neutral,
    Hostile,
    Ally,
    Item,
    Plant,
}
pub fn move_random(&pos: &Position) -> Position {
    let mut rng = RandomNumberGenerator::new();

    match rng.range(0, 4) {
        0 => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        1 => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        2 => Position {
            x: pos.x + 1,
            y: pos.y,
        },
        3 => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        _ => Position { x: pos.x, y: pos.y },
    }
}
pub fn move_chase(pos: &Position, target_pos: &Position) -> Position {
    if target_pos.x > pos.x {
        Position {
            x: pos.x + 1,
            y: pos.y,
        }
    } else if target_pos.x < pos.x {
        Position {
            x: pos.x - 1,
            y: pos.y,
        }
    } else if target_pos.y > pos.y {
        Position {
            x: pos.x,
            y: pos.y + 1,
        }
    } else if target_pos.y < pos.y {
        Position {
            x: pos.x,
            y: pos.y - 1,
        }
    } else {
        Position { x: pos.x, y: pos.y }
    }
}

pub fn mobs_move_by_time(
    mut commands: Commands,
    map: Res<Map>,
    mut mob_positions: Query<
        (Entity, &mut Position, &RequiredTime, Option<&MoveType>),
        (With<Mob>, With<GetATurn>),
    >,
    player_position: Query<&Position, (With<Player>, Without<Mob>)>,
    current_time: Res<CurrentTime>,
    mut turn_queue: ResMut<TurnQueue>,
) {
    let player_position = player_position.single();
    for (entity, position, required_time, move_type) in mob_positions.iter_mut() {
        let new_position: Position;
        if let Some(move_type) = move_type {
            let id = move_type.move_id;
            match id {
                MoveStrategy::Chase => {
                    new_position = move_chase(&position, player_position);
                }
                MoveStrategy::Random => {
                    new_position = move_random(&position);
                }
                _ => {
                    todo!("Move type not implemented")
                }
            }
        } else {
            new_position = move_random(&position);
        }
        if new_position != *position {
            commands.spawn(WantsToMove {
                entity: entity,
                destination: new_position,
            });
        }
        let mut next_time = current_time.clone();
        next_time.time.second += required_time.time;
        next_time.time.resolve_time();
        turn_queue.queue.push(WantATurn {
            time: next_time.time,
            character: entity,
            before_time: current_time.time,
        });

        commands.entity(entity).remove::<GetATurn>();
    }
}

pub fn spawn_mobs(
    mut map: ResMut<Map>,
    mut commands: Commands,
    atlas: Res<MobAsset>,
    current_time: Res<CurrentTime>,
) {
    let mut entities = Vec::new();
    let mut positions = Vec::new();
    for (position, mob) in map.mob_starts.iter() {
        let mut sprite = TextureAtlasSprite::new(mob.index);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        let entity = commands
            .spawn((
                SpriteSheetBundle {
                    sprite: sprite,
                    texture_atlas: atlas.atlas.clone(),
                    //transform: (*position).into(),
                    transform: (*position).to_transform(11.0),
                    ..Default::default()
                },
                *mob,
                Position {
                    x: position.x,
                    y: position.y,
                },
                GetATurn {
                    current_time: current_time.time,
                    before_time: current_time.time,
                },
            ))
            .id();
        entities.push(entity);
        positions.push(*position);
    }
    map.entities_occupy_tiles(entities, positions)
}
