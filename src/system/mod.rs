use crate::prelude::*;
mod camera;
mod clock;
mod combat;
mod end_turn;
mod mobs;
mod movement;
mod time_lapse;
pub use mobs::*;
pub struct UserPlugin;

/*impl Plugin for UserPlugin{

}*/

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharacterAsset>,
    mut map: ResMut<Map>,
    current_time: Res<CurrentTime>,
) {
    let player_start = map.player_start;
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    let entity = commands
        .spawn((
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: atlas.atlas.clone(),
                transform: player_start.into(),
                ..Default::default()
            },
            Player,
            Naming("Player".to_string()),
            Health {
                max: 10,
                current: 10,
            },
            Hunger {
                max: 100,
                current: 100,
            },
            Thirth {
                max: 100,
                current: 100,
            },
            SleepDesire {
                max: 100,
                current: 100,
            },
            Damage(5),
            Position {
                x: player_start.x,
                y: player_start.y,
            },
            GetATurn {
                current_time: current_time.time,
                before_time: current_time.time,
            },
        ))
        .id();
    map.entity_occupy_tile(entity, player_start);
}
pub fn player_take_a_turn(
    mut commands: Commands,
    does_player_turn: Query<Entity, (With<Player>, With<GetATurn>)>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if let Ok(entity) = does_player_turn.get_single() {
        next_state.set(TurnState::AwaitingInput);
        commands.entity(entity).remove::<GetATurn>();
    }
}
pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_postion: Query<(Entity, &mut Position, &mut Transform), With<Player>>,
    mobs: Query<(Entity, &Position), (With<Health>, Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
    current_time: Res<CurrentTime>,
    mut turn_queue: ResMut<TurnQueue>,
) {
    // println!("current_time: {:?}",current_time.time);
    let (player_entity, pos, transform) = player_postion.single_mut();
    let mut action = true;
    let mut wait = false;
    let mut new_position = *pos;
    let key = keyboard_input.get_pressed().next().cloned();

    if let Some(key) = key {
        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Up => new_position.y += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Space => wait = true,
            _ => action = false,
        }

        if new_position != *pos {
            let mut hit_something = false;
            mobs.iter()
                .filter(|(_, mob_pos)| **mob_pos == new_position)
                .for_each(|(mob_entity, _)| {
                    hit_something = true;
                    commands.spawn(WantsToAttack {
                        attacker: player_entity,
                        victim: mob_entity,
                    });
                });
            if !hit_something {
                commands.spawn(WantsToMove {
                    entity: player_entity,
                    destination: new_position,
                });
            }
            next_state.set(TurnState::PlayerTurn);
        }
        if wait {
            next_state.set(TurnState::PlayerTurn);
        }

        keyboard_input.reset(key);

        let mut next_time = current_time.clone();
        next_time.time.second += 5;
        next_time.time.resolve_time();
        turn_queue.queue.push(WantATurn {
            time: next_time.time,
            character: player_entity,
            before_time: current_time.time,
        })
    }
}
pub struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_input,
                //movement::try_move,
                // movement::try_move,
                // camera::camera_move,
            )
                .chain()
                .run_if(in_state(TurnState::AwaitingInput)),
        );
    }
}
pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // listening to user input on inventory screen
            .add_systems(
                Update,
                (
                    //player_input,
                    //mobs::mobs_move,
                    //movement::try_move,
                    //camera::camera_move,
                    combat::combat,
                    //end_turn::end_turn,

                    //equip_first_weapon,
                    //equip_weapon_log
                )
                    .chain()
                    .run_if(in_state(TurnState::PlayerTurn)),
            );

        app.add_systems(
            Update,
            (player_take_a_turn,).run_if(not(in_state(TurnState::AwaitingInput))),
        );
    }
}

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                //mobs::mobs_move_by_time,
                mobs::allies_move_by_time,
                mobs::enemies_move_by_time,
                mobs::neutrals_move_by_time,
            )
                //.chain()
                .run_if(not(in_state(TurnState::AwaitingInput))),
        );
    }
}

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                clock::time_management,
                //time_lapse::time_lapse,
            )
                .chain()
                .run_if(not(in_state(TurnState::AwaitingInput))),
        );
        app.add_systems(Update, (movement::try_move, camera::camera_move).chain());
    }
}
