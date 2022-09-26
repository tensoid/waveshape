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
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Deceleration(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct MaxVelocity(pub f32);

#[derive(Component)]
pub struct RotationSpeed(pub f32);


#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub level: Level,
    pub acceleration: Acceleration,
    pub deceleration: Deceleration,
    pub velocity: Velocity,
    pub max_velocity: MaxVelocity,
    pub rotation_speed: RotationSpeed,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}