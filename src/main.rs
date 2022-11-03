#![feature(panic_info_message)]

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

// Getting the board presets from the resources library.
extern crate fort_builders;

mod game;
mod listener;
mod tiles;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::{
        default, App, Camera2dBundle, ClearColor, Color, Commands, Component, DefaultPlugins, Msaa,
        Res, ResMut, WindowDescriptor, Windows,
    },
    render::camera::{OrthographicProjection, ScalingMode, WindowOrigin},
    window::WindowMode,
};
use fort_builders::{
    board::{BTM, LFT, RGT, TOP},
    RED, RST,
};
use game::GamePlugin;
use listener::listener;
use tiles::TilePlugin;

pub const RESOLUTION: f32 = 4.0 / 3.0;
pub const SPRITESIZE: f32 = 32.0;
pub const TILESIZE: (f32, f32) = (0.99, 0.99);
pub const TILEDRAW: (f32, f32) = (0.89, 0.89);

// To store player's position.
#[derive(Debug, Component)]
pub struct PlayerPosition(f32, f32);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

fn window_close(input: Res<Input<KeyCode>>, mut window: ResMut<Windows>) {
    if input.pressed(KeyCode::LControl) && input.pressed(KeyCode::Q) {
        let id = window.primary().id();
        window.remove(id);
    }
}

fn setup(mut commands: Commands) {
    // Default player position.
    commands.insert_resource(PlayerPosition(0.0, 0.0));
    commands.spawn().insert_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            left: (LFT as f32) * RESOLUTION,
            top: (TOP as f32) * RESOLUTION,
            right: (RGT as f32) * RESOLUTION,
            bottom: (BTM as f32) * RESOLUTION,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::Center,
            ..default()
        },
        ..default()
    });
}

// Setting up panic hook.
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        println!(
            "{RED}ERROR:{RST} [{}] {}",
            info.location().expect("No error location found."),
            info.message().expect("No error message found."),
        );
    }));
}

// Players are different colors based on their choice team.
// In future, make this semi-automated.
// When no option, the program will automatically assign a team.
fn main() {
    // Setting up panic hook.
    set_panic_hook();
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Fort Chess".to_string(),
            resizable: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.7)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilePlugin)
        .add_plugin(GamePlugin)
        .add_system(listener)
        .add_system(window_close)
        .run();
}
