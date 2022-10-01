use bevy::{core::Stopwatch, prelude::*};

pub struct GameData {
    game_state: f32, //TODO: make gamestate enum
    round_start_stopwatch: Stopwatch 
}

//TODO: startup system init game data
    // - Deserialization
    // - Setting default
    

pub fn initialize_game_data(mut commands: Commands){
    commands.insert_resource(
        GameData {
            game_state: 0.0,
            round_start_stopwatch: Stopwatch::new()
        }
    )
}