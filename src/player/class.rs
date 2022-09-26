use bevy::prelude::*;

use crate::player::player_bundle::*;
use crate::layers::Layer;

pub enum Class {
    SCOUT
}

impl Class {
    pub fn get_bundle(&self, asset_server: Res<AssetServer>) -> PlayerBundle {
        match self {
            Class::SCOUT => {

                let player_texture: Handle<Image> = asset_server.load("player.png");

                PlayerBundle {
                    player: Player,
                    health: Health {
                        max_health: 100.0,
                        current_health: 100.0
                    },
                    level: Level(0.0),
                    move_speed: MoveSpeed(5.0),
                    sprite_bundle: SpriteBundle {
                        texture: player_texture,
                        transform: Transform {
                            translation: Vec3::new(0.0,0.0,Layer::PLAYER.get_z()),
                            scale: Vec3::new(0.05, 0.05, 1.0),
                            ..default()
                        },
                        ..default()
                    }
                }
            }
        }
    }
}