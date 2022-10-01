use std::time::Duration;

use crate::player::{combat::projectile_type::ProjectileType};
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
        player_transform: Transform
    ) {
        self.attack_timer.tick(delta_time);

        if self.attack_timer.finished() {

            let projectile_bundle = self.projectile_type.get_projectile_bundle(asset_server);

            for (transform, speed) in self.shooting_behavior.shots.clone() {
                let mut projectile = projectile_bundle.clone();

                projectile.sprite_bundle.transform = player_transform;
                //TODO: add transform offset from shootingbehavior

                projectile.velocity.0 = (player_transform.rotation * Vec3::Y).truncate();
                projectile.velocity.0 *= speed.0;

                commands.spawn_bundle(projectile);
            }
        }
    }

    pub fn upgrade() {}
}
