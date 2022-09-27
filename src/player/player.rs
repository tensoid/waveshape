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
    asset_server: Res<AssetServer>,
    asset_server2: Res<AssetServer>
    //TODO: fix what ever this is???
) {
    commands.spawn_bundle(Class::TRIANGLE.get_player_bundle(asset_server))
    .with_children(|parent| {
        parent.spawn_bundle(Class::TRIANGLE.get_booster_bundle(asset_server2));
    });
}   