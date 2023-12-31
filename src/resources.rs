use crate::prelude::*;

#[derive(Resource)]
pub struct MapAsset {
    pub atlas: Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct MobAsset {
    pub atlas: Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct CharacterAsset {
    pub atlas: Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct ItemAsset {
    pub atlas: Handle<TextureAtlas>,
}
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    //StartScreen,
    AwaitingInput,
    //InMenus,
    Timelapsing,
    MonsterTurn,
    MoveTime,
    BulletMoving,
    //GameOver,
    //Victory,
    //NextLevel,
}

#[derive(Resource)]
pub struct GameLog {
    pub entries: Vec<String>,
}
impl GameLog {
    pub fn new() -> Self {
        let mut log: Vec<String> = Vec::with_capacity(4);
        log.push("Log...".to_string());
        log.push("\nUse the arrow keys to move.".to_string());
        log.push("\nBump into the enemies to attack them.".to_string());
        log.push("\nFind the amulet to win the game.".to_string());

        let entries_log = GameLog { entries: log };
        entries_log
    }

    pub fn add_entry(&mut self, message: String) {
        self.entries.push(message);
        self.entries.remove(0);
    }
}
