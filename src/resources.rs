use crate::prelude::*;

#[derive(Resource)]
pub struct MapAsset{
    pub atlas:Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct MobAsset{
    pub atlas:Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct CharacterAsset{
    pub atlas:Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    //StartScreen,
    AwaitingInput,
    //InMenus,
    PlayerTurn,
    MonsterTurn,
    //GameOver,
    //Victory,
    //NextLevel,
}