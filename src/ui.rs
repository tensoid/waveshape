use bevy::prelude::*;

use crate::layers::Layer;

#[derive(Component)]
struct RoundTimeText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui);
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Hello".to_string(),
            TextStyle {
                font: asset_server.load("fonts/VCR_OSD_MONO.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        visibility: Visibility {is_visible:true},
        ..default()
    })
    .insert(RoundTimeText);

    
}

fn update_ui() {

}
