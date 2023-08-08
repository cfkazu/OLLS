use crate::{prelude::*};
mod camera;
mod movement;
mod end_turn;
mod mobs;
mod combat;
pub use mobs::*;
pub struct UserPlugin;


/*impl Plugin for UserPlugin{
    
}*/

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharacterAsset>,
    mut map: ResMut<Map>,
){
    let player_start = map.player_start;
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    let entity = commands
        .spawn((
            SpriteSheetBundle{
                sprite: sprite,
                texture_atlas: atlas.atlas.clone(),
                transform: player_start.into(),
                ..Default::default()
            },
            Player,
            Naming("Player".to_string()),
            Health{max: 10, current: 10},
            Damage(5),
            Position { x: player_start.x, y: player_start.y },
        )).id();
    map.entity_occupy_tile(entity,player_start);
    
}

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_postion:Query<(Entity,&mut Position,&mut Transform),With<Player>>,
    mut map: ResMut<Map>,
    mobs:Query<(Entity, &Position),(With<Health>,Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
){
    let(player_entity,mut pos,mut transform) = player_postion.single_mut();
    let mut action = true;
    let mut wait = false;
    let mut new_position = pos.clone();
    let key = keyboard_input.get_pressed().next().cloned();

    if let Some(key) = key{
        match key{
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Up => new_position.y += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Space => wait = true,
            _ => action = false,
        }
    /* 
    if map.can_enter_tile(new_position){
        map.move_entity_occupation(player_entity, pos.clone(), new_position);
        pos.x = new_position.x;
        pos.y = new_position.y;
        //transform.translation.x = (pos.x as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
        //transform.translation.y = (pos.y as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
    }*/
    

        if new_position != *pos{
            let mut hit_something = false;
            mobs.iter()
                .filter(|(_,mob_pos)|{
                    **mob_pos == new_position
                })
                .for_each(|(mob_entity,_)|{
                    hit_something = true;
                    commands.spawn(WantsToAttack{
                        attacker: player_entity,
                        victim: mob_entity
                    });
                });
            if !hit_something{
                commands.spawn(WantsToMove{entity: player_entity, destination: new_position});
            }
            next_state.set(TurnState::PlayerTurn);
        }

        keyboard_input.reset(key);
    }
}
pub struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (player_input,
                
                movement::try_move,
                camera::camera_move,
                
            ).chain().run_if(
                in_state(TurnState::AwaitingInput))
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
                end_turn::end_turn,
                
                //equip_first_weapon,
                //equip_weapon_log
            )
            .chain()
            .run_if(in_state(TurnState::PlayerTurn))
            );
    }
}

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
                mobs::mobs_move,
                movement::try_move,
                end_turn::end_turn
            )
            .chain()
            .run_if(in_state(TurnState::MonsterTurn))
            );
    }
}