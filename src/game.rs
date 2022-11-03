//! game module.
//!
//! To handle game functionalities like start, update and win/lose/draw.
//! Holds the GameAsset which is the instance of the current game data object.
//! It also is responsible for drawing the pieces to the board.
//!
//! ## Contents:
//! -   GamePlugin
//! -   GameAsset
//! -   Piece

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, SPRITESIZE, TILESIZE};
use bevy::prelude::*;
use fort_builders::{
    board::Quadrant,
    dice_roll,
    game::Game,
    player::{Player, Team},
};

// Temporary.
const PLAYERS: usize = 4;
// Highlight coior.
const HILITE_CLR: Color = Color::rgba(0.6, 0.6, 0.6, 0.3);

/// The GamePlugin that holds piece drawing information.
pub struct GamePlugin;

#[derive(Component)]
struct PlayerSheet(Handle<TextureAtlas>);

#[derive(Component)]
struct Piece;

#[derive(Component)]
struct Highlight;

#[derive(Debug, Component)]
pub struct GameAsset(pub Game);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████GameAsset████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAsset {
    /// To get a reference to the inner game tuple element,
    pub fn get(&self) -> &Game {
        &self.0
    }

    /// To get a mutable reference to the inner game tuple element,
    pub fn get_mut(&mut self) -> &mut Game {
        &mut self.0
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Plugin for GamePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, init_game)
            .add_startup_system_to_stage(StartupStage::Startup, load_sprite)
            .add_system(gametick);
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
/// Initial game creation.
fn init_game(mut commands: Commands) {
    let roll = (dice_roll() % 3_usize) % PLAYERS;
    commands.insert_resource(dbg!(GameAsset(Game::init(
        (0..PLAYERS)
            .into_iter()
            .map(|i| {
                Player::from(
                    format!("player {}", i + 1),
                    Team::from_index(i).unwrap(),
                    roll == i,
                    PLAYERS,
                    Quadrant::from_index(calcq(i, roll)).unwrap(),
                )
                .unwrap()
            })
            .collect::<Vec<Player>>()
    ))));
}

fn gametick(
    mut commands: Commands,
    sprite: Res<PlayerSheet>,
    mut game: ResMut<GameAsset>,
    dquery: Query<Entity, With<Piece>>,
    hquery: Query<Entity, With<Highlight>>,
) {
    if !game.get().update {
        return;
    }
    draw_pieces(&mut commands, &sprite, &game, &dquery);
    highlight(&mut commands, &game, &hquery);
    game.get_mut().set_update_false();
}

/// Function to map quadrants to the player correctly ignoring the defender quadrant.
// The whole recurring theme seems to be that I am not satisfied by the logic/implementation of
// the function as a whole. I really wish I could come up with a more clever way of working around
// this problem.
fn calcq(i: usize, roll: usize) -> usize {
    match i {
        i if i <= roll => i,
        i if i > roll => (i - 1) % 3,
        _ => panic!("Unexpected error when matching i and roll ({i}, {roll})."),
    }
}

/*-----------------------------------------------------------------------------------------------*/

/*████Piece████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_pieces(commands: &mut Commands, query: &Query<Entity, With<Piece>>) {
    for pieces in query.iter() {
        commands.entity(pieces).despawn();
    }
}

/// To draw the players.
fn draw_pieces(
    mut commands: &mut Commands,
    sprite: &Res<PlayerSheet>,
    game: &ResMut<GameAsset>,
    query: &Query<Entity, With<Piece>>,
) {
    if !game.get().update {
        return;
    }
    clear_pieces(&mut commands, &query);
    game.get().players.iter().for_each(|player| {
        player.pieces.iter().for_each(|piece| {
            let sprite = spawn_piece(
                &mut commands,
                &sprite,
                (player.team.to_index() * 5) + piece.piece_type.to_index(),
                Vec3::new(
                    piece.position.x as f32 * RESOLUTION,
                    piece.position.y as f32 * RESOLUTION,
                    8.0,
                ),
            );
            commands
                .entity(sprite)
                .insert(Name::from("Piece"))
                .insert(Piece);
        })
    });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Highlight████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_highlight(commands: &mut Commands, query: &Query<Entity, With<Highlight>>) {
    for blocks in query.iter() {
        commands.entity(blocks).despawn();
    }
}

fn highlight(
    mut commands: &mut Commands,
    game: &ResMut<GameAsset>,
    query: &Query<Entity, With<Highlight>>,
) {
    if !game.get().update {
        return;
    }
    clear_highlight(&mut commands, &query);
    for piece in game.get().player().pieces() {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: HILITE_CLR,
                    custom_size: Some(Vec2::new(TILESIZE.0 * RESOLUTION, TILESIZE.1 * RESOLUTION)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        piece.position.x as f32 * RESOLUTION,
                        piece.position.y as f32 * RESOLUTION,
                        5.0,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Highlight);
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerSheet████*/
/*-----------------------------------------------------------------------------------------------*/
/// To load the player sprites.
fn load_sprite(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(PlayerSheet(texture_atlases.add(
        TextureAtlas::from_grid_with_padding(
            asset.load("spritesheet/chess_pieces_sheet.png"),
            Vec2::splat(SPRITESIZE * 2.0),
            5, // Rows.
            5, // Columns.
            Vec2::splat(0.0),
            Vec2::splat(0.0),
        ),
    )));
}

fn spawn_piece(
    commands: &mut Commands,
    tile: &PlayerSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(TILESIZE.0 * RESOLUTION, TILESIZE.1 * RESOLUTION)),
                ..default()
            },
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation: translation,
                ..default()
            },
            ..default()
        })
        .id()
}
/*-----------------------------------------------------------------------------------------------*/
