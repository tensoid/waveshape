use bevy::prelude::*;

use crate::layers::Layer;
use crate::player::player_bundles::*;

pub enum Class {
    TRIANGLE,
}

impl Class {
    pub fn get_player_bundle(&self, asset_server: Res<AssetServer>) -> PlayerBundle {
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
                    acceleration: Acceleration(0.2),
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

    //TODO: maybe texture atlas

    pub fn get_booster_bundle(&self, asset_server: Res<AssetServer>) -> BoosterBundle {
        match self {
            Class::TRIANGLE => {
                let booster_texture: Handle<Image> = asset_server.load("booster.png");

                BoosterBundle {
                    booster: Booster,
                    sprite_bundle: SpriteBundle {
                        texture: booster_texture,
                        sprite: Sprite {
                            flip_y: true, 
                            color: Color::rgb(1.0, 0.05, 0.0), 
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0.0, -45.0, Layer::PLAYER.get_z()),
                            scale: Vec3::new(0.6, 0.6, 1.0),
                            ..default()
                        },
                        ..default()
                    }
                }
            }
        }
    }
}
