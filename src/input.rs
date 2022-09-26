use bevy::prelude::*;

use crate::player::player_bundle::*;


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player);
    }
}


fn move_player (
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<(&Acceleration, &Deceleration, &RotationSpeed, &MaxVelocity, &mut Velocity, &mut Transform), With<Player>>,
) {
    let (
        player_acceleration, 
        player_deceleration, 
        player_rotation_speed, 
        player_max_velocity, 
        mut player_velocity, 
        mut player_transform
    ) = query.single_mut();

    //TODO: change pivot point
    let window = windows.get_primary().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        let player_rotation_z: f32 = player_transform.translation.truncate().angle_between(cursor_position);
        println!("{}", player_transform.translation);
        //println!("{}", cursor_position);
        player_transform.rotation = Quat::from_rotation_z(player_rotation_z);
    }

    if keyboard_input.pressed(KeyCode::W){
        let player_direction = (player_transform.rotation * Vec3::Y).truncate();
        player_velocity.0 += player_direction * player_acceleration.0;
    }
    else {
        player_velocity.0 *= player_deceleration.0;
    } 
    
    player_transform.translation += player_velocity.0.clamp_length_max(player_max_velocity.0).extend(0.0);
}