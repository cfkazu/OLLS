use crate::prelude::*;

pub fn combat(
    mut commands:Commands<'_, '_>,
    mut map:ResMut<Map>,
    attacker_messages:Query<(Entity,&WantsToAttack)>,
    player:Query<Entity,With<Player>>,
    names_query:Query<&Naming>,
    mut health_query:Query<(&mut Health,&Position,&Naming)>,
    damage_query:Query<&Damage>,
){
    let victims:Vec<(Entity,Entity,Entity)> = attacker_messages.iter()
    .map(|(entity, attack)| (entity, attack.attacker, attack.victim))
    .collect();

    victims.iter().for_each(|(message,attacker,victim)|{
        let base_damage = if let Ok((d)) = damage_query.get(*attacker) {
            d.0
        } else {
            0
        };
     /*    let w_damage: i32 = damage_query.iter()
            .filter(|(_, c, e)| c.is_some() && e.is_some())
            .map(|(dmg, carried, _)| (dmg, carried.unwrap()))
            .filter(|(_, carried)| carried.0 == *attacker)
            .map(|(dmg, _)| dmg.0)
            .sum();*/
        let final_damage = base_damage;// + w_damage;
        if let Ok((mut hp, pos, name)) = health_query.get_mut(*victim) {
            hp.current -= final_damage;
            // add action to gamelog, first get name of attacker, then build message
            let attacker_char = names_query.get(*attacker).unwrap();
            let message = format!("\n{} attacks {} ({} damage).", attacker_char.0, name.0, final_damage);
            //gamelog.add_entry(message);
            /* 
            // less than 1 HP remove it
            if hp.current < 1 {
                if let Ok(_) = player.get(*victim) {
                    
                } else {
                    map.free_occupy_tile(*pos);
                    commands.entity(*victim).despawn();
                }
            }
            */
        }
        commands.entity(*message).despawn();
    });
}