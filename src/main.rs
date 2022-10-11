// Getting the board presets from the resources library.
extern crate fort_builders;

mod tiles;

use bevy::prelude::*;
// use bevy::window::WindowMode;
use bevy::render::camera::{ ScalingMode, WindowOrigin };

use fort_builders::{ dice_roll, exit, PlayerLW };
use fort_builders::player::{ Player, PlayerAction, Team };
use fort_builders::game::{ Game, GameAction };
use fort_builders::board::{ TOP, BTM, RGT, LFT };
use thiserror::Error;

pub const RESOLUTION: f32 = (16.0 / 9.0) * 20.0;
pub const TILESIZE: (f32, f32) = (1.0, 1.0);

use tiles::{ TilePlugin, load_tile };

#[derive(Error, Debug)]
enum Error {
    #[error("Error while loading graphics ({0}).")]
    GraphicalError(String),
    #[error("Error withing the fort builder module ({0}).")]
    InternalErrorLibError(#[from] fort_builders::Error),
    // SoundError(sound_lib::Error),
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.top       =   (TOP as f32) * RESOLUTION;
    camera.projection.bottom    =   (BTM as f32) * RESOLUTION;
    camera.projection.right     =   (RGT as f32) * RESOLUTION;
    camera.projection.left      =   (LFT as f32) * RESOLUTION;
    camera.projection.scaling_mode =    ScalingMode::None;
    camera.projection.window_origin =   WindowOrigin::Center;
    commands.spawn_bundle(camera);
}


// Players are different colors based on their choice team.
// In future, make this semi-automated.
// When no option, the program will automatically assign a team.
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor{
            // resizable: false,
            // decorations: true,
            title: "Fort Chess".to_string(),
            .. Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, load_tile)
        .add_startup_system(spawn_camera)
        .add_plugin(TilePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
