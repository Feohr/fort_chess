//! game module.
//!
//! To handle game functionalities like start, update and win/lose/draw.
//! Holds the GameAsset which is the instance of the current game data object.
//! It also is responsible for drawing the pieces to the board.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod draw_piece;
mod highlight;
mod player_name;

use crate::SPRITESIZE;
use bevy::{
    prelude::{
        Entity, With, Commands, Res, ResMut, Component, Vec2, Handle, TextureAtlas, StartupStage,
        App, Assets, AssetServer, Plugin, Query,
    }
};
use fort_builders::{
    board::{Quadrant, q1_outer_bound_pos, q2_outer_bound_pos, q3_outer_bound_pos},
    dice_roll,
    game::{Game, GameAction},
    player::{Player, Team},
};
use draw_piece::draw_pieces;
use draw_piece::Piece;
use highlight::highlight_active_pieces;
use highlight::Highlight;
use player_name::display_player_names;
use player_name::PlayerName;
use player_name::PlayerNameBoxVec;

// Temporary holder for number of players.
const PLAYER_COUNT: usize = 4;

/// The game Plugin that holds piece drawing information.
pub(crate) struct GamePlugin;

/// To handle [`Player`] texture.
#[derive(Component)]
pub(crate) struct PlayerSheet(Handle<TextureAtlas>);

/// To hold [`Game`] resource.
#[derive(Debug, Component)]
pub(crate) struct GameAsset(pub(crate) Game);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for GamePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for GamePlugin {

    /// [`Plugin`] implementation for [`GamePlugin`].
    fn build(&self, app: &mut App) {
        app .add_startup_system_to_stage(StartupStage::PreStartup, init_game                )
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite              )
            .add_startup_system_to_stage(StartupStage::Startup,    init_player_name_box_vec )
            .add_system(                                           game_update_tick         );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Init Player Name Box████*/
/*-----------------------------------------------------------------------------------------------*/
fn init_player_name_box_vec(
    mut commands:   Commands,
    game:           Res<GameAsset>,
) {

    let mut player_name = PlayerNameBoxVec::new();
    let mut outer_check_fn_iter = [
        q1_outer_bound_pos,
        q2_outer_bound_pos,
        q3_outer_bound_pos,
    ].into_iter();

    for player in game.get().players.iter() {

        let (x, y) = if player.is_defender {
            (-1, 0)
        } else {
            (outer_check_fn_iter.next().unwrap())()
        };

        player_name.push(player.name.clone(), player.team, x, y);

    }

    commands.insert_resource(player_name);

}
/*-----------------------------------------------------------------------------------------------*/

/*████GameAsset████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAsset {

    /// To get a reference to the inner game tuple element,
    pub(crate) fn get(&self) -> &Game { &self.0 }

    /// To get a mutable reference to the inner game tuple element,
    pub(crate) fn get_mut(&mut self) -> &mut Game { &mut self.0 }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
/// Initial game creation. In future, this will be handled a bit differently to facilitate variable
/// game players.
fn init_game(mut commands: Commands) {

    let dice_roll = (dice_roll() % 3_usize) % PLAYER_COUNT;
    let mut quadrant = [Quadrant::Q1, Quadrant::Q2, Quadrant::Q3].into_iter();

    commands.insert_resource(dbg!(GameAsset(Game::init(
        (usize::MIN..PLAYER_COUNT)
            .into_iter()
            .map(|index| {
                let is_defender = dice_roll == index;
                Player::from(
                    format!("Mohd Rehaan{}", index + 1_usize),
                    Team::from_index(index).unwrap(),
                    is_defender,
                    PLAYER_COUNT,
                    if is_defender { Quadrant::NoQuad } else { quadrant.next().unwrap() },
                )
                .unwrap()
            })
            .collect::<Vec<Player>>()
    ))));

}

/// Runs every frame of the game to check if the board needs to update graphics. Draws pieces as
/// well as highlights.
fn game_update_tick(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
    asset:          Res<AssetServer>,
    sprite:         Res<PlayerSheet>,
    mut pname:      ResMut<PlayerNameBoxVec>,
    dquery:         Query<Entity, With<Piece>>,
    hquery:         Query<Entity, With<Highlight>>,
    pnquery:        Query<Entity, With<PlayerName>>,
) {

    // If no need for update, return.
    if !game.get().update { return }

    clean_up_lost_players(game.get_mut(), &mut pname);

    draw_pieces(            &mut commands, &sprite, &game, &dquery);
    highlight_active_pieces(&mut commands, &game,          &hquery);
    display_player_names(   &mut commands, &pname,         &pnquery, &asset);

    // Setting the update as false to put latch back.
    game.get_mut().set_update_false();

}

/// Looks for players and kills them at every iteration.
fn clean_up_lost_players(
    game:   &mut Game,
    pname:  &mut ResMut<PlayerNameBoxVec>,
) {

    let _dead = game.hunt();

    if !_dead.is_empty() { dbg!(&_dead); }

    for player in _dead.into_iter() {
        pname.pop(player.team);
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
/*-----------------------------------------------------------------------------------------------*/
