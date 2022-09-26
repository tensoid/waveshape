use bevy::prelude::*;


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub current_health: f32
}

#[derive(Component)]
pub struct Level(pub f32);

#[derive(Component)]
pub struct MoveSpeed(pub f32);


#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub level: Level,
    pub move_speed: MoveSpeed,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}