use bevy::prelude::*;
use crate::utils::lerp_f32;

use crate::player::player_bundles::Player;

const BOOSTER_ZOOM: f32 = 1.1;
const BOOSTER_ZOOM_SMOOTH: f32 = 0.1;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_camera)
        .add_system_to_stage(CoreStage::PostUpdate, move_camera)
        .add_system(camera_effects);
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

fn camera_effects(
    mut camera: Query<(&mut OrthographicProjection), With<Camera>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut camera_projection = camera.single_mut();

    if mouse.pressed(MouseButton::Left) {
        let mut camera_projection = camera.single_mut();
        camera_projection.scale = lerp_f32(camera_projection.scale, BOOSTER_ZOOM, BOOSTER_ZOOM_SMOOTH);
    }
    else {
        camera_projection.scale = lerp_f32(camera_projection.scale, 1.0, BOOSTER_ZOOM_SMOOTH);
    }
}