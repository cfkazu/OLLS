use crate::prelude::*;
pub fn position_translation(primary_query: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    if let Ok(primary) = primary_query.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert_pos(pos.x as f32),
                convert_pos(pos.y as f32),
                11 as f32,
            );
        }
    }
}
pub fn convert_pos(pos: f32) -> f32 {
    (pos - (SIDE_LENGTH -1) as f32 /2.)*TILE_SIZE
}
