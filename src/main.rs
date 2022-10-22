// Getting the board presets from the resources library.
extern crate fort_builders;

mod tiles;

use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, ScalingMode, WindowOrigin};

use fort_builders::board::{Quadrant, BTM, LFT, RGT, TOP};
use fort_builders::game::{Game, GameAction};
use fort_builders::player::{Player, PlayerAction, Team};
use fort_builders::{dice_roll, exit, PlayerLW};
use tiles::TilePlugin;

pub const RESOLUTION: f32 = (16.0 / 9.0) * 20.0;
pub const TILESIZE: (f32, f32) = (0.99, 0.99);
const PLAYERS: usize = 4;

#[derive(Debug, Component)]
struct GameAsset(Game);

fn setup(mut commands: Commands) {
    let roll = dice_roll() % 3;
    let players = (0..PLAYERS)
        .into_iter()
        .map(|i| {
            Player::from(
                format!("player {}", i + 1),
                Team::from_index(i).unwrap(),
                roll == i,
                Quadrant::from_index(i % 3).unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<Player>>();
    commands.spawn().insert(GameAsset(Game::init(players)));
    commands.spawn().insert_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            top: (TOP as f32) * RESOLUTION,
            bottom: (BTM as f32) * RESOLUTION,
            right: (RGT as f32) * RESOLUTION,
            left: (LFT as f32) * RESOLUTION,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::Center,
            ..default()
        },
        ..default()
    });
}

fn print_game_info(query: Query<&GameAsset>) {
    query.iter().for_each(|game| {
        println!("{:#?}", game);
    })
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
        .add_startup_system(setup)
        .add_system(print_game_info)
        .add_plugin(TilePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
