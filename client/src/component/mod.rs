use bevy::animation::AnimationClip;
use bevy::asset::Handle;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);
