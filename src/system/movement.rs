use ron::de;

use crate::prelude::*;
pub fn try_move(
    mut commands: Commands,
    mut map: ResMut<Map>,
    move_messages: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &mut Position)>
){
    for (entity,move_signal) in move_messages.iter(){
        let destination = move_signal.destination;
        
        if map.can_enter_tile(destination){
            if map.is_tile_occupied(destination){
                
                if let Ok((mov_ent, mut position)) = movers.get_mut(move_signal.entity) {
                    // update occupation map
                    map.move_entity_occupation(mov_ent, *position, move_signal.destination);
                    // and execute the movement
                    position.x = move_signal.destination.x;
                    position.y = move_signal.destination.y;
                    // mark the fov to be updated
                }
            }
        }
        commands.entity(entity).despawn();
    }
}