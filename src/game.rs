//! game module.
//!
//! To handle game functionalities like start, update and win/lose/draw.
//! Holds the GameAsset which is the instance of the current game data object.
//! It also is responsible for drawing the pieces to the board.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, SPRITESIZE, TILESIZE, ZAxisLevel};
use bevy::prelude::*;
use fort_builders::{
    board::Quadrant,
    dice_roll,
    game::Game,
    player::{Player, Team},
};

// Temporary holder for number of players.
const PLAYERS:                  usize = 3;
/// The width of the pieces sprite sheet.
const PIECES_SPRITESHEET_WIDTH: usize = 5_usize;
/// Highlight color to display the current player pieces.
const HILITE_COLOR:             Color = Color::rgba(0.6, 0.6, 0.6, 0.3);

/// The game Plugin that holds piece drawing information.
pub(crate) struct GamePlugin;

/// To handle [`Player`] texture.
#[derive(Component)]
struct PlayerSheet(Handle<TextureAtlas>);

/// To distinguish piece entity.
#[derive(Component)]
struct Piece;

/// To distinguish highlight entity.
#[derive(Component)]
struct Highlight;

/// To hold [`Game`] resource.
#[derive(Debug, Component)]
pub(crate) struct GameAsset(pub(crate) Game);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████GameAsset████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAsset {

    /// To get a reference to the inner game tuple element,
    pub(crate) fn get(&self) -> &Game { &self.0 }

    /// To get a mutable reference to the inner game tuple element,
    pub(crate) fn get_mut(&mut self) -> &mut Game { &mut self.0 }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Plugin for GamePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for GamePlugin {

    /// [`Plugin`] implementation for [`GamePlugin`].
    fn build(&self, app: &mut App) {
        app .add_startup_system_to_stage(StartupStage::Startup, init_game   )
            .add_startup_system_to_stage(StartupStage::Startup, load_sprite )
            .add_system(                                        gametick    );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
/// Initial game creation. In future, this will be handled a bit differently to facilitate variable
/// game players.
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

/// Runs every frame of the game to check if the board needs to update graphics. Draws pieces as
/// well as highlights.
fn gametick(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
    sprite:         Res<PlayerSheet>,
    dquery:         Query<Entity, With<Piece>>,
    hquery:         Query<Entity, With<Highlight>>,
) {

    // If no need for update, return.
    if !game.get().update { return }

    draw_pieces(&mut commands, &sprite, &game, &dquery);
    highlight(  &mut commands,          &game, &hquery);

    // Setting the update as false to put latch back.
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
/// To clear all the pieces in a scene. Iterate over entity with [`Piece`] component and despawn
/// them.
fn clear_pieces(
    commands:   &mut Commands,
    query:      &Query<Entity, With<Piece>>,
) {

    for pieces in query.iter() {
        commands.entity(pieces).despawn();
    }

}

/// call to draw the player [`Piece`]s.
///
/// Iterating over each player and drawing all the pieces once again. *row* and *col* correspond
/// to the player sheet resource. Hence each position along the columns correspond to the piece
/// type which is added to offset to it. The team corresponds to the rows and it is multiplied
/// with the spritesheet width to jump between the rows. The constant PIECE_SPRITESHEET_WIDTH is
/// nothing but the number of chess piece types i.e. 5.
fn draw_pieces(
    commands:   &mut Commands,
    sprite:     &Res<PlayerSheet>,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Piece>>,
) {

    // Clean up.
    clear_pieces(commands, query);

    // For each player.
    game.get().players.iter().for_each(|player| {

        // For each piece.
        player.pieces.iter().for_each(|piece| {

            let sprite = spawn_piece(
                commands,
                sprite,
                (
                        // Row.
                        player.team.as_usize()  * PIECES_SPRITESHEET_WIDTH
                        // Column.
                )   +   piece.piece_type.as_usize(),
                Vec3::new(
                    //piece_pos_x.
                    piece.position.x as f32     *               RESOLUTION,
                    //piece_pos_y.
                    piece.position.y as f32     *               RESOLUTION,
                    //Z level.
                    ZAxisLevel::Eight.as_f32(),
                ),
            );

            // Spawn.
            commands.entity(sprite).insert(Name::from("Piece")).insert(Piece);

        })

    });

}
/*-----------------------------------------------------------------------------------------------*/

/*████Highlight████*/
/*-----------------------------------------------------------------------------------------------*/
/// To clear all the highlight entities from the scene.
fn clear_highlight(
    commands:   &mut Commands,
    query:      &Query<Entity, With<Highlight>>,
) {

    // Iterates over Highlight entities and despawns them.
    for blocks in query.iter() {
        commands.entity(blocks).despawn();
    }

}

/// To Draw highlight over the current player [`Piece`].
///
/// Iterating over the current active player and highlighting. The highlight size is [`TILESIZE`].
fn highlight(
    commands:   &mut Commands,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Highlight>>,
) {

    // Clean up.
    clear_highlight(commands, query);

    for piece in game.get().current_player().pieces() {

        // Spawn.
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: HILITE_COLOR,
                    custom_size: Some(Vec2::new(
                            //width.
                            TILESIZE.0          * RESOLUTION,
                            //height.
                            TILESIZE.1          * RESOLUTION,
                    )),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        //piece_pos_x.
                        piece.position.x as f32 * RESOLUTION,
                        //piece_pos_y.
                        piece.position.y as f32 * RESOLUTION,
                        // Z Level.
                        ZAxisLevel::Fifth.as_f32(),
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
/// To load the [`Player`] sprites.
///
/// To insert the chess piece sprite resource. There needs to be an asset folder at runtime for
/// the binary to load the player pieces asset. Otherwise it won't be possible for the pieces to
/// load.
fn load_sprite(
    mut commands:           Commands,
    mut texture_atlases:    ResMut<Assets<TextureAtlas>>,
    asset:                  Res<AssetServer>,
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

/// Simple helper function to spawn [`Piece`] sprites. Sprite size is [`TILESIZE`].
fn spawn_piece(
    commands:       &mut Commands,
    tile:           &PlayerSheet,
    index:          usize,
    translation:    Vec3,
) -> Entity {

    // Spawn.
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(
                        // width.
                        TILESIZE.0 * RESOLUTION,
                        // height.
                        TILESIZE.1 * RESOLUTION,
                )),
                ..default()
            },
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()

}
/*-----------------------------------------------------------------------------------------------*/
