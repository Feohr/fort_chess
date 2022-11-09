#![feature(panic_info_message)]

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

extern crate fort_builders;

mod game;
mod listener;
mod tiles;

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

pub(crate) const RESOLUTION: f32       = 4.0 / 3.0;
pub(crate) const SPRITESIZE: f32       = 32.0;
pub(crate) const TILESIZE: (f32, f32)  = (0.99, 0.99);
pub(crate) const TILEDRAW: (f32, f32)  = (0.89, 0.89);
           const BKGRND_COLOR: Color   = Color::rgb(0.8, 0.8, 0.7);

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
            ZAxisLevel::Fifteenth  => 14.0_f32,
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/

fn close_window_listener(
    input:      Res<Input<KeyCode>>,
    mut window: ResMut<Windows>,
) {

    if      input.pressed(KeyCode::LControl)
        &&  input.pressed(KeyCode::Q) {

        let id = window.primary().id();
        window.remove(id);

    }

}

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

// To set custom panic statement format.
//
// Format:
// ```
// $ERROR: thread panicked at: ['panic_location'] called 'error_source' on an 'error_type' with
//  value: 'error_value'
// ```
fn set_panic_hook_fmt() {

    std::panic::set_hook(Box::new(|info| {
        println!(
            "{RED}ERROR:{RST} [{}] {}",
            info.location().expect("No error location found."),
            info.message().expect("No error message found."),
        );
    }));

}

fn main() {

    set_panic_hook_fmt();

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
