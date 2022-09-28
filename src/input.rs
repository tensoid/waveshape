use bevy::prelude::*;

use crate::player::player_bundles::*;

const MIN_SPEED_BEFORE_STOP: f32 = 0.1;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player);
    }
}


fn move_player (
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
    mut player_query: Query<(&Acceleration, &Deceleration, &MaxVelocity, &mut Velocity, &mut Transform), With<Player>>,
    mut booster_query: Query<&mut Visibility, With<Booster>>
) {
    let (
        player_acceleration, 
        player_deceleration, 
        player_max_velocity, 
        mut player_velocity, 
        mut player_transform
    ) = player_query.single_mut();

    let mut booster_visibility = booster_query.single_mut();

    //TODO: change pivot point
    let window = windows.get_primary().unwrap();
    if let Some(mut cursor_position) = window.cursor_position() {

        let window_width: f32 = window.width();
        let window_height: f32 = window.height();

        // Offset to center of screen aka player
        cursor_position.x -= window_width / 2.0;
        cursor_position.y -= window_height / 2.0;

        // Set the players rotation to the direction of the mouse cursor
        let player_rotation_z: f32 = Vec2::new(0.0, 1.0).angle_between(cursor_position);
        player_transform.rotation = Quat::from_rotation_z(player_rotation_z);
    }

    if mouse.pressed(MouseButton::Left){
        let player_direction = (player_transform.rotation * Vec3::Y).truncate();
        player_velocity.0 += player_direction * player_acceleration.0;
        booster_visibility.is_visible = true;
    }
    else {
        booster_visibility.is_visible = false;
        player_velocity.0 *= player_deceleration.0;
        if player_velocity.0.length() < MIN_SPEED_BEFORE_STOP {
            player_velocity.0 = Vec2::ZERO;
        }
    } 

    // Clamp to max velocity
    player_velocity.0 = player_velocity.0.clamp_length_max(player_max_velocity.0);
    
    player_transform.translation += player_velocity.0.extend(0.0);
}