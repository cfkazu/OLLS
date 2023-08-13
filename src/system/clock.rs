use crate::prelude::*;
pub fn time_management(
    mut commands: Commands,
    mut turn_queue: ResMut<TurnQueue>,
    mut current_time: ResMut<CurrentTime>,
    get_aturn: Query<(Entity, &GetATurn)>,
) {
    for (_, get_aturn) in get_aturn.iter() {
        if get_aturn.current_time == current_time.time {
            // println!("not removed yet time:{:?}",current_time.time);
            return;
        }
    }
    //want a turnのOrdは逆順にしてあるので，popで取り出すと最も時間が小さいのが取り出される
    let next = turn_queue.queue.pop();
    if let Some(next) = next {
        current_time.time = next.time;
        let entity_candidate = commands.get_entity(next.character);
        if let Some(mut entity) = entity_candidate {
            entity.insert(GetATurn {
                current_time: current_time.time,
                before_time: next.before_time,
            });
        }

        loop {
            // println!("time:{:?}",current_time.time);
            let seek_next = turn_queue.queue.peek();
            if let Some(seek_next) = seek_next {
                if seek_next.time == current_time.time {
                    let next = turn_queue.queue.pop();
                    if let Some(next) = next {
                        let entity_candidate = commands.get_entity(next.character);
        if let Some(mut entity) = entity_candidate {
            entity.insert(GetATurn {
                current_time: current_time.time,
                before_time: next.before_time,
            });
        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
