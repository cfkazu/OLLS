use crate::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::format;
use std::fs::File;
use ron::de::from_reader;
#[derive(Clone, Deserialize, Debug)]
pub struct SpawnTemplate {
    pub mob_type:MobType,
    pub probability: i32,
    pub name: String,
    pub drop_items: Option<Vec<(String, i32)>>,
    pub description: Option<String>,
    pub hp: Option<i32>,
    pub index: usize,
    pub base_damage: Option<i32>,
    pub position: Option<Position>,
    pub hunger: Option<i32>,
    pub sleep: Option<i32>,
} 


#[derive(Clone, Deserialize, Debug)]
pub struct SpawnTemplates {
    pub entities: Vec<SpawnTemplate>,
}

impl SpawnTemplates{
    pub fn load(filename: &str) -> Self {
        let file = File::open(format!("assets/map/template_{}.ron",filename))
            .expect("Failed to open spawn templates file");
        from_reader(file).expect("Failed to load spawn templates")
    }

    pub fn spawn_entities(
        &self,
        commands: &mut Commands,
        atlas:Res<MobAsset>,
        mut map:&mut ResMut<Map>,
    ){
        let mut rng = RandomNumberGenerator::new();
        for entity in self.entities.iter(){
            if rng.range(0,100) < entity.probability{
                if let Some(position) = entity.position{
                    self.spawn_entity(&position, &entity, commands, atlas.atlas.clone(), &mut map);
                }else{
                    todo!("Spawn entity at random position");
                }
            }
        }
    }

    fn spawn_entity(
        &self,
        position:&Position,
        template:&SpawnTemplate,
        commands:&mut Commands,
        atlas:Handle<TextureAtlas>,
        map:&mut ResMut<Map>,
    ){
        let mut sprite = TextureAtlasSprite::new(template.index);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        let mut entity = commands
            .spawn((
                SpriteSheetBundle{
                    sprite:sprite,
                    texture_atlas: atlas,
                    transform:(*position).to_transform(11.0),
                    ..Default::default()
                },
                Naming(template.name.clone()),
                Mob{
                    mob_type: template.mob_type.clone(),
                    index: template.index,
                },
                Position { x: position.x, y: position.y },
            ));
        if let Some(hp) = template.hp{
            entity.insert(Health{current:hp,max:hp});
        }
        if let Some(damage) = template.base_damage{
            entity.insert(Damage(damage));
        }
        if let Some(hunger) = template.hunger{
            entity.insert(Hunger{current:hunger,max:hunger});
        }
        if let Some(sleep) = template.sleep{
            entity.insert(SleepDesire{current:sleep,max:sleep});
        }
        map.entity_occupy_tile(entity.id(), *position);
    }
}

pub fn spawn_map_templates(
    mut commands: Commands,
    mut map: ResMut<Map>,
    atlas: Res<MobAsset>,
){
    let template = SpawnTemplates::load(&map.file_name);
    template.spawn_entities(&mut commands, atlas, &mut map);
}