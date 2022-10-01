use bevy::prelude::*;

use crate::common::Velocity;

use super::projectile_type::ProjectileType;


#[derive(Component, Clone)]
pub struct Projectile(pub ProjectileType);

#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}