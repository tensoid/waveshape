use bevy::prelude::*;

use crate::layers::Layer;

#[derive(Component)]
pub struct Grid;

pub struct GridPlugin;

//const GRID_SQUARE_LENGTH: f32 = 73.0;
const GRID_SQUARE_LENGTH: f32 = 100.0;
const GRID_SCALE: f32 = 2.0;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid).add_system(align_grid);
    }
}

fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid_texture: Handle<Image> = asset_server.load("grid2.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture: grid_texture,
            sprite: Sprite {
                color: Color::rgba(87.0 / 255.0, 2.0 / 255.0, 150.0 / 255.0, 0.1),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Layer::GRID.get_z()),
                scale: Vec3::new(GRID_SCALE, GRID_SCALE, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Grid);
}

fn align_grid(
    mut grid_query: Query<&mut Transform, (With<Grid>, Without<Camera>)>,
    camera_query: Query<&Transform, (With<Camera>, Without<Grid>)>,
) {
    let mut grid_transform = grid_query.single_mut();
    let real_qrid_square_length: f32 = GRID_SQUARE_LENGTH * grid_transform.scale.x;

    let camera_translation = camera_query.single().translation;

    let player_idx_square_x: f32 = (camera_translation.x / real_qrid_square_length).floor();
    let player_idx_square_y: f32 = (camera_translation.y / real_qrid_square_length).floor();

    grid_transform.translation.x = player_idx_square_x * real_qrid_square_length;
    grid_transform.translation.y = player_idx_square_y * real_qrid_square_length;
}
