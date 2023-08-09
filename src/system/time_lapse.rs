use crate::prelude::*;
pub fn time_lapse(
    mut commands:Commands<'_, '_>,
    mut map:ResMut<Map>,
    mut character_query:
    Query<(
        Entity,
        &mut Health,
        &Position,
        Option<&mut Hunger>,
        Option<&mut SleepDesire>,
        Option<&mut Thirth>
    )>
){
   for(
       entity,
       mut health,
       pos,
       hunger,
       sleep_desire,
       thirth
   ) in character_query.iter_mut(){
       
       if let Some(mut hunger) = hunger{
           hunger.current -= 1;
           if hunger.current <= 0{
                health.current = 0;
           }
       }
       if let Some(mut sleep_desire) = sleep_desire{
            sleep_desire.current -= 1;
            if sleep_desire.current <= 0{
                 health.current = 0;
            }
       }
       if let Some(mut thirth) = thirth{
           thirth.current -= 1;
            if thirth.current <= 0{
                 health.current = 0;
            }
       }
       if health.current <= 0{
            map.free_occupy_tile(*pos);
           commands.entity(entity).despawn();
       }
   }
}