use crate::prelude::*;
use serde::Deserialize;
use std::fs::File;
use ron::de::from_reader;
mod mob_status;
#[derive(Clone, Deserialize, Debug,Default)]
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
    pub required_time:Option<i32>,
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
    pub fn load_by_vec(entities:Vec<SpawnTemplate>)->Self{
        Self{
            entities
        }
    }
    pub fn spawn_entities(
        &self,
        commands: &mut Commands,
        atlas:Res<MapAsset>,
        mut map:&mut ResMut<Map>,
        current_time:Res<CurrentTime>,
        mob_status_list:&Res<MobStatusList>,
    ){
        let mut rng = RandomNumberGenerator::new();
        for entity in self.entities.iter(){
            if rng.range(0,100) < entity.probability{
                if let Some(position) = entity.position{
                    self.spawn_entity(&position, &entity, commands, atlas.atlas.clone(), &mut map,&current_time,&mob_status_list);
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
        current_time:&Res<CurrentTime>,
        mob_status_list:&Res<MobStatusList>,
    ){
        let mob_status =mob_status_list.mob_status_list.get(&template.index);
        let mut sprite = TextureAtlasSprite::new(template.index);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        let mut entity: bevy::ecs::system::EntityCommands<'_, '_, '_>;
        if let Some(mob_status) = mob_status{
            entity = commands
            .spawn((
                SpriteSheetBundle{
                    sprite:sprite,
                    texture_atlas: atlas,
                    transform:(*position).to_transform(11.0),
                    ..Default::default()
                },
                Naming(mob_status.name.clone()),
                Mob{
                    mob_type: mob_status.mob_type.clone(),
                    index: mob_status.index,
                },
                Position { x: position.x, y: position.y },
            ));
            if let Some(hp) = mob_status.hp{
                entity.insert(Health{current:hp,max:hp});
            }
            if let Some(damage) = mob_status.base_damage{
                entity.insert(Damage(damage));
            }
            if let Some(hunger) = mob_status.hunger{
                entity.insert(Hunger{current:hunger,max:hunger});
            }
            if let Some(sleep) = mob_status.sleep{
                entity.insert(SleepDesire{current:sleep,max:sleep});
            }
            if let Some(required_time) = mob_status.required_time{
                entity.insert(RequiredTime{time:required_time});
            }else{
                entity.insert(RequiredTime{time:15});
            }
            if mob_status.mob_type != MobType::Item{
                entity.insert(GetATurn{current_time:current_time.time.clone(),before_time:current_time.time.clone()});
            }
        }else{
            entity = commands
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
            if let Some(required_time) = template.required_time{
                entity.insert(RequiredTime{time:required_time});
            }else{
                entity.insert(RequiredTime{time:15});
            }
            if template.mob_type != MobType::Item{
                entity.insert(GetATurn{current_time:current_time.time.clone(),before_time:current_time.time.clone()});
            }
        }
        
        
        /* 
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
        }*/
        map.entity_occupy_tile(entity.id(), *position);
    }
}

pub fn spawn_map_templates(
    mut commands: Commands,
    mut map: ResMut<Map>,
    atlas: Res<MapAsset>,
    current_time:Res<CurrentTime>,
    mob_status_list:&Res<MobStatusList>,
){
    let template = SpawnTemplates::load(&map.file_name);
    template.spawn_entities(&mut commands, atlas, &mut map,current_time,mob_status_list);
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Startup,
            (
                mob_status::setup,
                //time_lapse::time_lapse,
            )
            .chain()
        );

    }
}

