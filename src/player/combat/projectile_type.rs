use bevy::prelude::*;

use crate::common::Velocity;

use super::{projectile_bundle::{ProjectileBundle, Projectile}};

#[derive(Clone, Copy)]
pub enum ProjectileType {
    LASER,
}

impl ProjectileType {
    
    pub fn get_projectile_bundle(self, asset_server: &mut Box<Res<AssetServer>>) -> ProjectileBundle {

        let projectile_texture = match self {
            ProjectileType::LASER => asset_server.load("laser.png")
        };

        ProjectileBundle {
            projectile: Projectile(self),
            velocity: Velocity(Vec2::ZERO),
            sprite_bundle: SpriteBundle {
                texture: projectile_texture,
                ..default()
            },
        }
    }
}
