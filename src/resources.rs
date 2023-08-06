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