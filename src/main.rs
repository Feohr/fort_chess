// Getting the board presets from the resources library.
extern crate fort_builders;

mod game;
mod tiles;

use bevy::{
    prelude::{
        App, Camera2dBundle, ClearColor, Commands, DefaultPlugins,Color, Msaa, WindowDescriptor,
        default,
    },
    render::camera::{OrthographicProjection, ScalingMode, WindowOrigin},
};
use fort_builders::board::{LFT, TOP, RGT, BTM};
use game::GamePlugin;
use tiles::TilePlugin;

pub const RESOLUTION: f32 = (16.0 / 9.0) * 20.0;
pub const TILESIZE: (f32, f32) = (0.99, 0.99);

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

// Players are different colors based on their choice team.
// In future, make this semi-automated.
// When no option, the program will automatically assign a team.
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.7)))
        .insert_resource(WindowDescriptor {
            title: "Fort Chess".to_string(),
            ..Default::default()
        })
        .add_plugin(TilePlugin)
        .add_plugin(GamePlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
