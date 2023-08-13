use crate::prelude::*;
#[derive(Component)]
pub struct TopUINode;
mod hud;
#[derive(Resource)]
pub(crate) struct FontManager {
    pub font: Handle<Font>,
}

fn setup(asset_server: ResMut<AssetServer>, mut commands: Commands) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");
    let manager = FontManager { font };
    commands.insert_resource(manager);

    let game_log = GameLog::new();
    commands.insert_resource(game_log);
}
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_plugins(hud::HudPlugin);
    }
}
