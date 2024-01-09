// Uses and includes
use bevy::prelude::*;

// Local modules
mod constants;
mod components;

/// Main function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}

/// Spawn players and bases
fn setup() {
    // TODO
}