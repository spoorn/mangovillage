use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_ecs_ldtk::prelude::FieldValue;

#[derive(Component)]
pub struct WorldComponent {
    pub level_iid: String
}

// Player Spawn portal entities
// TODO: Fix coordinates
#[derive(Component, Default)]
pub struct PlayerSpawn;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    pub player_spawn: PlayerSpawn
}

#[derive(Component, Default)]
pub struct Portal {
    // Destination Level iid
    pub destination: String,
    // Destination's portal ID linked to this portal
    pub link: i32,
    pub ldtk_coords: (i32, i32),
    pub width: f32,
    pub height: f32
}

impl From<EntityInstance> for Portal {
    fn from(entity_instance: EntityInstance) -> Self {
        let mut destination = None;
        let mut link = None;
        for field in &entity_instance.field_instances {
            match field.identifier.as_str() {
                "Destination" => if let FieldValue::String(Some(dst)) = &field.value {
                    destination = Some(dst);
                },
                "Link" => if let FieldValue::Int(Some(lnk)) = &field.value {
                    link = Some(lnk);
                },
                _ => {}
            }
        }
        if destination.is_some() && link.is_some() {
            return Portal {
                destination: destination.unwrap().clone(),
                link: *link.unwrap(),
                ldtk_coords: (entity_instance.px.x, entity_instance.px.y),
                width: entity_instance.width as f32,
                height: entity_instance.height as f32
            };
        }
        panic!("Could not create Portal Component from EntityInstance: {}", entity_instance.identifier);
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PortalBundle {
    #[from_entity_instance]
    pub portal: Portal
}