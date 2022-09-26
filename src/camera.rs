use bevy::prelude::*;

use crate::player::player_bundle::Player;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_camera)
        .add_system_to_stage(CoreStage::PostUpdate, move_camera);
    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
){
    let mut player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}