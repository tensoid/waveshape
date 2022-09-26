use bevy::prelude::*;

use crate::player::class::Class;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(Class::TRIANGLE.get_bundle(asset_server));
}   