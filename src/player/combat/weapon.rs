use std::time::Duration;

use crate::{player::{combat::projectile_type::ProjectileType}, common::Velocity};
use bevy::prelude::*;

use super::weapon_type::WeaponType;

#[derive(Clone)]
pub struct Speed(pub f32);

#[derive(Clone)]
pub struct ShootingBehavior {
    pub shots: Vec<(Transform, Speed)>, //TODO: add transform
    pub delay_between_shots: Option<Duration>,
}

pub struct Weapon {
    pub projectile_type: ProjectileType,
    pub attack_timer: Timer,
    pub weapon_type: WeaponType,
    pub shooting_behavior: ShootingBehavior,
}

impl Weapon {
    pub fn try_shoot(
        &mut self,
        delta_time: Duration,
        commands: &mut Box<Commands>,
        asset_server: &mut Box<Res<AssetServer>>,
        player_transform: Transform,
        player_velocity: Velocity
    ) {
        self.attack_timer.tick(delta_time);

        if self.attack_timer.finished() {

            let projectile_bundle = self.projectile_type.get_projectile_bundle(asset_server);

            for (mut projectile_offset, projectile_speed) in self.shooting_behavior.shots.clone() {
                let mut projectile = projectile_bundle.clone();

                projectile.sprite_bundle.transform = player_transform;

                // Add rotation offset
                projectile.sprite_bundle.transform.rotation *= projectile_offset.rotation;

                // Add translation offset
                projectile_offset.rotate_around(Vec3::ZERO, player_transform.rotation);
                projectile.sprite_bundle.transform.translation += projectile_offset.translation;

                // Set velocity
                projectile.velocity.0 = (projectile.sprite_bundle.transform.rotation * Vec3::Y).truncate();
                projectile.velocity.0 *= projectile_speed.0;
                projectile.velocity.0 += player_velocity.0;

                commands.spawn_bundle(projectile);
            }
        }
    }

    pub fn upgrade() {}
}


