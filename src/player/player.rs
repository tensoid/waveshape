use bevy::prelude::*;

use crate::player::player_bundle::*;
use crate::player::class::Class;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_input);
    }
}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(Class::SCOUT.get_bundle(asset_server));
}   


fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut player_transform, player_move_speed) = query.single_mut();

    //TODO: normalize move vector (diagonal bug)

    if keyboard_input.pressed(KeyCode::A){
        player_transform.translation.x -= player_move_speed.0;
    }

    if keyboard_input.pressed(KeyCode::D){
        player_transform.translation.x += player_move_speed.0;
    }

    if keyboard_input.pressed(KeyCode::W){
        player_transform.translation.y += player_move_speed.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        player_transform.translation.y -= player_move_speed.0;
    }

    //player_transform.rotate(Quat::from_rotation_z(-0.2));
}