use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};

mod player;
mod gamedata;
mod map;
mod camera;
mod layers;
mod input;
mod utils;
mod ui;
mod common;


use player::player::PlayerPlugin;
use camera::CameraPlugin;
use map::grid::GridPlugin;
use input::InputPlugin;
use gamedata::gamedata::initialize_game_data;
use ui::UIPlugin;
use player::combat::combat::CombatPlugin;

fn main() {

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Hellwave".to_string(),
            //present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(display_fps)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(CombatPlugin)
        .add_startup_system(initialize_game_data)
        .run();
}


fn display_fps(diagnostics: Res<Diagnostics>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!(
        "NCA - {:.2}",
        diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.average()).unwrap_or(0.0)
    ));
}
