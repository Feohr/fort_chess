//! A simple chess-inspired board game made using rust and bevy game engine. For instructions to
//! play please read the "README.md" file: [README](../../../README.md).
//!
//! main module.
//! The outer most entry module of the game. Everything starts here.
//!
//! ## Conventions followed:
//! > [Rust Conventions](https://rust-lang.github.io/api-guidelines/naming.html).

#![feature(let_else)]
#![feature(panic_info_message)]
#![feature(generic_arg_infer)]

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

extern crate fort_builders;

/*------------*/
mod despawn_entity;
mod font;
mod game;
mod listener;
mod startscreen;
mod state;
mod tiles;
/*------------*/

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::{
        default, App, Camera2dBundle, ClearColor, Color, Commands, DefaultPlugins, Res, ResMut,
        State, WindowDescriptor, Windows,
    },
    render::camera::{OrthographicProjection, ScalingMode, WindowOrigin},
    window::WindowMode,
};
use font::FontHandlePlugin;
use fort_builders::{
    board::{BTM, LFT, RGT, TOP},
    RED, RST,
};
use game::GamePlugin;
use listener::ListenerPlugin;
use startscreen::MainScreenPlugin;
use state::FortChessState;
use tiles::TilePlugin;

/// Size of a single sprite.
pub(crate) const SPRITESIZE: f32 = 32_f32;
/// The resolution of the game.
pub(crate) const RESOLUTION: f32 = (4_f32 / 3_f32) * SPRITESIZE;
/// Size of the tile/elements relative
/// to the spritesize.
pub(crate) const TILESIZE: (f32, f32) = (0.99_f32, 0.99_f32);
/// Size of highlighting block.
pub(crate) const TILEDRAW: (f32, f32) = (0.89_f32, 0.89_f32);
/// The background screen color.
const BKGRND_COLOR: Color = Color::SILVER;

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
            ZAxisLevel::First => 1_f32,
            ZAxisLevel::Second => 2_f32,
            ZAxisLevel::Third => 3_f32,
            ZAxisLevel::Fourth => 4_f32,
            ZAxisLevel::Fifth => 5_f32,
            ZAxisLevel::Sixth => 6_f32,
            ZAxisLevel::Seventh => 7_f32,
            ZAxisLevel::Eight => 8_f32,
            ZAxisLevel::Ninth => 9_f32,
            ZAxisLevel::Tenth => 10_f32,
            ZAxisLevel::Eleventh => 11_f32,
            ZAxisLevel::Twelfth => 12_f32,
            ZAxisLevel::Thirteenth => 13_f32,
            ZAxisLevel::Fourteenth => 14_f32,
            ZAxisLevel::Fifteenth => 15_f32,
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/// Function to close the game window. Press `Ctrl + q` to quit.
#[inline]
fn close_window_listener(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.pressed(KeyCode::LControl) && input.pressed(KeyCode::Q) {
        close_window(&mut windows)
    }
}

#[inline]
pub(crate) fn close_window(window: &mut ResMut<Windows>) {
    if let Some(window) = window.get_primary_mut() {
        window.close()
    }
}

/// Initial setup.
///
/// Fetches the bounds constraints from the [`fort_builders`] library and sets up the camera
/// according to the [`RESOLUTION`]. The scaling mode is set to `None`.
fn setup(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            left: LFT as f32 * RESOLUTION,
            top: TOP as f32 * RESOLUTION,
            right: RGT as f32 * RESOLUTION,
            bottom: BTM as f32 * RESOLUTION,
            far: 10000_f32,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::Center,
            ..default()
        },
        ..default()
    });
}

/// To set a custom panic statement format.
///
/// ## Format:
/// ```text
/// $ERROR: thread panicked at: [panic_location] called `error_source' on an 'error_type' with
///  value: 'error_value'
/// ```
fn set_panic_hook_fmt() {
    std::panic::set_hook(Box::new(|info| {
        println!(
            "{RED}ERROR:{RST} [{}] {:?}",
            info.location()
                .expect("Fatal error, cannot find the panic location."),
            info.message().expect("No error message produced."),
        );
    }));
}

fn tmp_state_change(mut state: ResMut<State<FortChessState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Return) {
        match state.current() {
            FortChessState::StartScreen => {
                state.set(FortChessState::GameBuild).unwrap_or_default();
            }
            FortChessState::GameBuild => {
                state.set(FortChessState::BoardScreen).unwrap_or_default();
            }
            _ => {}
        }
    }
}

/// Main entry function.
fn main() {
    set_panic_hook_fmt();
    //
    //
    //
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Fort Chess".to_string(),
            resizable: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(ClearColor(BKGRND_COLOR))
        .add_state(FortChessState::new())
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainScreenPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(ListenerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(FontHandlePlugin)
        .add_system(close_window_listener)
        .add_system(tmp_state_change)
        .run();
}
