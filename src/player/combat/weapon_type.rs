use bevy::prelude::*;

use crate::player::combat::projectile_type::ProjectileType;
use crate::player::combat::weapon::Weapon;

use crate::player::combat::weapon::ShootingBehavior;

use super::weapon::Speed;

pub enum WeaponType {
    LASER,
}

impl WeaponType {
    pub fn get_weapon(self) -> Weapon {
        match self {
            WeaponType::LASER => {
                Weapon {
                    projectile_type: ProjectileType::LASER,
                    attack_timer: Timer::from_seconds(1.0, true),
                    weapon_type: self,
                    shooting_behavior: ShootingBehavior {
                        shots: vec![
                            (
                                Transform {
                                    rotation: Quat::from_rotation_z(0.2),
                                    translation: Vec3::new(0.0, 25.0, 0.0),
                                    ..default()
                                }, 
                                Speed(10.0)
                            ),
                            (
                                Transform {
                                    rotation: Quat::from_rotation_z(0.0),
                                    translation: Vec3::new(0.0, 25.0, 0.0),
                                    ..default()
                                }, 
                                Speed(10.0)
                            ),
                            (
                                Transform {
                                    rotation: Quat::from_rotation_z(-0.2),
                                    translation: Vec3::new(0.0, 25.0, 0.0),
                                    ..default()
                                }, 
                                Speed(10.0)
                            )
                        ],
                        delay_between_shots: None, //TODO: implement, maybe with custom ShotTimer?? that keeps track of shots shot etc.
                    },
                }
            }
        }
    }
}
