use crate::prelude::*;
pub fn time_management(
    mut commands:Commands,
    mut turn_queue:ResMut<TurnQueue>,
    mut current_time:ResMut<CurrentTime>,
){
    //want a turnのOrdは逆順にしてあるので，popで取り出すと最も時間が小さいのが取り出される
    let next = turn_queue.queue.pop();
    if let Some(next) =next{
        current_time.time = next.time;
        commands.entity(next.character).insert(GetATurn{
            current_time:current_time.time,
            before_time:next.before_time,
        });
        loop{
            let seek_next = turn_queue.queue.peek();
            if let Some(seek_next) = seek_next{
                if seek_next.time == current_time.time{
                    let next = turn_queue.queue.pop();
                    if let Some(next) = next{
                        commands.entity(next.character).insert(GetATurn{
                            current_time:current_time.time,
                            before_time:next.before_time,
                        });
                    }
                }
            }else{
                break;
            }
        }     
    }
}