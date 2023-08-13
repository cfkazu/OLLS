use crate::prelude::*;
pub fn die_despawn(
    mut commands:Commands,
    dead:Query<(Entity,&ToDie)>,
    mut gamelog:ResMut<GameLog>,
    mut map: ResMut<Map>,
    mut player: Query<(Entity, &mut Health), With<Player>>,
){
    for (entity,to_die) in dead.iter(){
        if let Ok((_, mut player_health)) = player.get_mut(to_die.entity){
            player_health.current = 10;
            gamelog.add_entry(format!("\n You died!:{}",to_die.death_reason));
        }else{
            //多分despawnはやりすぎで，死体とかを残したほうが良い
            //もちろんその場合は，MoveとかのComponentをはく奪する必要がある
            commands.entity(to_die.entity).despawn();
            gamelog.add_entry(format!("{}",to_die.death_reason));
            map.free_occupy_tile(to_die.position);
        }
        commands.entity(entity).despawn();
    }
}