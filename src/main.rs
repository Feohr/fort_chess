//! A simple chess-inspired board game made using rust and bevy game engine. For instructions to
//! play please read the "README.md" file: [README](../../../README.md).
//!
//! main module.
//! The outer most entry module of the game. Everything starts here.
//!
//! ## Conventions followed:
//! > [Rust Conventions](https://rust-lang.github.io/api-guidelines/naming.html).

#![feature(panic_info_message)]
#![feature(let_else)]

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

extern crate fort_builders;

/*------------*/
//   Modules  //
/*------------*/
mod game;
mod listener;
mod tiles;
/*------------*/

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::{
        default, App, Camera2dBundle, ClearColor, Color, Commands, DefaultPlugins, Res,
        ResMut, WindowDescriptor, Windows,
    },
    render::camera::{OrthographicProjection, ScalingMode, WindowOrigin},
    window::WindowMode,
};
use fort_builders::{
    board::{BTM, LFT, RGT, TOP},
    RED, RST,
};
use game::GamePlugin;
use listener::ListenerPlugin;
use tiles::TilePlugin;

/// Size of a single sprite.
pub(crate) const SPRITESIZE             : f32         = 32_f32;
/// The resolution of the game.
pub(crate) const RESOLUTION             : f32         = (4_f32 / 3_f32) * SPRITESIZE;
/// Size of the tile/elements relative
/// to the spritesize.
pub(crate) const TILESIZE               : (f32, f32)  = (0.99_f32, 0.99_f32);
/// Size of highlighting block.
pub(crate) const TILEDRAW               : (f32, f32)  = (0.89_f32, 0.89_f32);
/// The background screen color.
           const BKGRND_COLOR           : Color       = Color::SILVER;

/// To hold Z-axis layer values. Each name corresponds to it's value in `f32`.
#[allow(unused)]
pub(crate) enum ZAxisLevel {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    Thirteenth,
    Fourteenth,
    Fifteenth,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████ZAxisLevel████*/
/*-----------------------------------------------------------------------------------------------*/
impl ZAxisLevel {

    /// Each level corresponds to its value in `f32`. There are a total of _ levels.
    fn as_f32(&self) -> f32 {

        match self {
            ZAxisLevel::First       =>  1.0_f32,
            ZAxisLevel::Second      =>  2.0_f32,
            ZAxisLevel::Third       =>  3.0_f32,
            ZAxisLevel::Fourth      =>  4.0_f32,
            ZAxisLevel::Fifth       =>  5.0_f32,
            ZAxisLevel::Sixth       =>  6.0_f32,
            ZAxisLevel::Seventh     =>  7.0_f32,
            ZAxisLevel::Eight       =>  8.0_f32,
            ZAxisLevel::Ninth       =>  9.0_f32,
            ZAxisLevel::Tenth       => 10.0_f32,
            ZAxisLevel::Eleventh    => 11.0_f32,
            ZAxisLevel::Twelfth     => 12.0_f32,
            ZAxisLevel::Thirteenth  => 13.0_f32,
            ZAxisLevel::Fourteenth  => 14.0_f32,
            ZAxisLevel::Fifteenth   => 15.0_f32,
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/

/// Function to close the game window. Press `Ctrl + q` to quit.
fn close_window_listener(
    input:      Res<Input<KeyCode>>,
    mut window: ResMut<Windows>,
) {

    // If the Control and q key is pressed, find the primary window and close it instantly.
    if      input.pressed(KeyCode::LControl)
        &&  input.pressed(KeyCode::Q) {

        let id = window.primary().id();
        window.remove(id);

    }

}

/// Initial setup.
///
/// Fetches the bounds constraints from the [`fort_builders`] library and sets up the camera
/// according to the [`RESOLUTION`]. The scaling mode is set to `None`.
fn setup(mut commands: Commands) {

   commands.spawn()
            .insert_bundle(Camera2dBundle {
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

/// To set a custom panic statement format.
/// ## Format:
/// ```text
/// $ERROR: thread panicked at: [panic_location] called `error_source' on an 'error_type' with
///  value: 'error_value'
/// ```
fn set_panic_hook_fmt() {

    std::panic::set_hook(Box::new(|info| {
        println!(
            "{RED}ERROR:{RST} [{}] {}",
            info.location().expect("No error location found."),
            info.message().expect("No error message found."),
        );
    }));

}

/// Main entry function.
fn main() {

    // Setting up the panic hook before the program begins.
    set_panic_hook_fmt();

    // Main loop of the game.
    //
    // __WindowDescriptor__ for setting up the window.
    // -    Name:       fort_chess.
    // -    Reszable:   no.
    // -    Mode:       fullscreen.
    //
    // __ClearColor__ is used to fill the background color,
    // -    Valie:      BKGRND_COLOR.
    //
    //  Plugins:
    //  -   Default:    for default bevy functionalities.
    //  -   Tile:       to draw the fort.
    //  -   Game:       to handle the game object.
    //  -   Listener:   to handle input.
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Fort Chess".to_string(),
            resizable: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(ClearColor(BKGRND_COLOR))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ListenerPlugin)
        .add_system(close_window_listener)
        .run();

}
