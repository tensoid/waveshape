use bevy::prelude::*;

use crate::layers::Layer;
use crate::player::player_bundle::*;

pub enum Class {
    TRIANGLE,
}

impl Class {
    pub fn get_bundle(&self, asset_server: Res<AssetServer>) -> PlayerBundle {
        match self {
            Class::TRIANGLE => {
                let player_texture: Handle<Image> = asset_server.load("triangle.png");

                PlayerBundle {
                    player: Player,
                    health: Health {
                        max_health: 100.0,
                        current_health: 100.0,
                    },
                    level: Level(0.0),
                    acceleration: Acceleration(0.15),
                    deceleration: Deceleration(0.95),
                    velocity: Velocity(Vec2::new(0.0, 0.0)),
                    rotation_speed: RotationSpeed(0.1),
                    max_velocity: MaxVelocity(8.0),
                    sprite_bundle: SpriteBundle {
                        texture: player_texture,
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, Layer::PLAYER.get_z()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                }
            }
        }
    }
}