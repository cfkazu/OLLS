pub struct UserPlugin;
use crate::prelude::*;
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
            Position { x: player_start.x, y: player_start.y },
        )).id();
    map.entity_occupy_tile(entity,player_start);
    
}

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_postion:Query<(Entity,&mut Position,&mut Transform),With<Player>>,
    mut map: ResMut<Map>,
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
    
    if map.can_enter_tile(new_position){
        pos.x = new_position.x;
        pos.y = new_position.y;
        transform.translation.x = (pos.x as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
        transform.translation.y = (pos.y as f32 - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE;
    }
    
    
    

/* 
        if new_position != *pos{
            commands.spawn(WantsToMove{entity: player_entity, destination: new_position});
        }*/

        keyboard_input.reset(key);
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
                player_input,
                //equip_first_weapon,
                //equip_weapon_log
            )
                //.run_if(in_state(TurnState::AwaitingInput))
            );
    }
}