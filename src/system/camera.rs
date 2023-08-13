use crate::prelude::*;

pub fn camera_move(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    for player_position in player_query.iter() {
        if let Ok(window) = primary_query.get_single() {
            let mut camera_transform = camera_query.single_mut();
            // calculate new coordinates and update
            let cam_x = convert_pos(player_position.x as f32);
            let cam_y = convert_pos(player_position.y as f32);
            camera_transform.translation = Vec3::new(cam_x, cam_y as f32, 999.0);
        }
    }
}
