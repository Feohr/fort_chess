#![feature(panic_info_message)]

// Getting the board presets from the resources library.
extern crate fort_builders;

mod game;
mod tiles;
mod listener;

use bevy::{
    prelude::{
        App, Camera2dBundle, ClearColor, Color, Commands, CursorMoved, DefaultPlugins, EventReader,
        Msaa, WindowDescriptor, default, SpriteBundle, Sprite, Vec2, Vec3, Transform, Windows, Res,
        Query, Component, Entity, With,
    },
    render::camera::{OrthographicProjection, ScalingMode, WindowOrigin},
    window::WindowMode,
};
use fort_builders::{
    board::{LFT, TOP, RGT, BTM},
    RED, RST,
};
use game::GamePlugin;
use tiles::TilePlugin;
use listener::ListenerPlugin;

pub const RESOLUTION: f32 = 4.0 / 3.0;
pub const TILESIZE: (f32, f32) = (0.99, 0.99);
pub const SPRITESIZE: f32 = 32.0;

fn setup(mut commands: Commands) {
   commands.spawn().insert_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            left:   (LFT as f32) * RESOLUTION,
            top:    (TOP as f32) * RESOLUTION,
            right:  (RGT as f32) * RESOLUTION,
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
    std::panic::set_hook(Box::new(|info|{
        println!("{RED}ERROR:{RST} [{}] {}",
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
        .add_plugin(ListenerPlugin)
        .run();
}
