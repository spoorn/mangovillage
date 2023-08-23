use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

pub struct LightingPlugin;
impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, directional_light);
    }
}

fn directional_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { color: Color::BEIGE, shadows_enabled: true, illuminance: 40_000.0, ..default() },
        frusta: Default::default(),
        cascades: Default::default(),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder { minimum_distance: 1.0, maximum_distance: 100000.0, ..default() }.into(),
        visible_entities: Default::default(),
        transform: Transform::from_xyz(-500.0, -500.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Z),
        global_transform: Default::default(),
        visibility: Default::default(),
        computed_visibility: Default::default(),
    });
}
