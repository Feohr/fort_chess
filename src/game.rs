//! game module.
//!
//! To handle game functionalities like start, update and win/lose/draw.
//! Holds the GameAsset which is the instance of the current game data object.
//! It also is responsible for drawing the pieces to the board.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

//-----------//
pub(crate) mod draw_piece;
pub(crate) mod highlight;
pub(crate) mod player_name;
pub(crate) mod game_end;
//-----------//

use crate::{
    SPRITESIZE,
    font::BoldFontHandle,
    state::FortChessState,
    startscreen::NameEntryValue,
};
use bevy::prelude::{
    Entity, With, Commands, Res, ResMut, Component, Vec2, Handle, TextureAtlas, StartupStage, App,
    Assets, AssetServer, Plugin, Query, SystemSet, State,
};
use fort_builders::{
    dice_roll,
    board::{Quadrant, q1_outer_bound_pos, q2_outer_bound_pos, q3_outer_bound_pos},
    game::{Game, GameAction},
    player::{Player, Team},
};
use draw_piece::{draw_pieces, Piece};
use highlight::{highlight_active_pieces, Highlight};
use player_name::{
    display_player_names, highlight_player_name, PlayerName, PlayerNameBoxVec, PlayerNameOutline,
};
use game_end::GameEndPlugin;

/// To hold the number of types of pieces.
const PIECE_TYPE_COUNT: usize = 5_usize;
/// To hold the number of types of teams.
const TEAM_TYPE_COUNT: usize = 3_usize;

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
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite)
            .add_system_set(
                SystemSet::on_enter(FortChessState::GameBuild)
                .with_system(init_game)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::GameBuild)
                .with_system(set_state_boardscreen)
            )
            .add_system_set(
                SystemSet::on_enter(FortChessState::BoardScreen)
                .with_system(init_player_name_box_vec)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::BoardScreen)
                .with_system(game_update_tick)
            )
           .add_system_set(
               SystemSet::on_exit(FortChessState::BoardScreen)
               .with_system(dealloc_player_name_box_vec)
            )
            .add_plugin(GameEndPlugin);
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Player Name Box████*/
/*-----------------------------------------------------------------------------------------------*/
/// Simple function to initialize player name struct vec.
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
    game
        .get()
        .players
        .iter()
        .for_each(|player| {
            let (x, y) = if player.is_defender {
                (-1_i32, 0_i32)
            } else {
                (outer_check_fn_iter.next().unwrap())()
            };
            player_name.push(player.name.clone(), player.team, x, y);
        });
    commands.insert_resource(player_name);
}

/// To dealloc [`PlayerNameBoxVec`] when leaving board screen.
fn dealloc_player_name_box_vec(mut commands: Commands) {
    commands.remove_resource::<PlayerNameBoxVec>();
}
/*-----------------------------------------------------------------------------------------------*/

/*████GameAsset████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAsset {
    /// To get a reference to the inner game tuple element,
    #[inline]
    pub(crate) fn get(&self) -> &Game {
        &self.0
    }
    /// To get a mutable reference to the inner game tuple element,
    #[inline]
    pub(crate) fn get_mut(&mut self) -> &mut Game {
        &mut self.0
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
/// Initial game creation. In future, this will be handled a bit differently to facilitate variable
/// game players.
fn init_game(
    mut commands:           Commands,
    name_entry_value_res:   Res<NameEntryValue>,
) {
    let count = name_entry_value_res.count();
    if count < 2_usize { panic!("Less than two players") }
    let dice_roll = (dice_roll() % TEAM_TYPE_COUNT) % count;
    let mut quadrant = [Quadrant::Q1, Quadrant::Q2, Quadrant::Q3].into_iter();
    commands.insert_resource(GameAsset(Game::init(
        (usize::MIN..count)
            .into_iter()
            .map(|index| {
                let is_defender = dice_roll == index;
                Player::from(
                    name_entry_value_res.as_string(index).unwrap(),
                    Team::from_index(index).unwrap(),
                    is_defender,
                    count,
                    if is_defender {
                        Quadrant::NoQuad
                    } else {
                        quadrant.next().unwrap()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<Player>>()
    )));
}

/// To set the state to [`BoardScreen`].
///
/// [`BoardScreen`]: FortChessState::BoardScreen
fn set_state_boardscreen(mut state: ResMut<State<FortChessState>>) {
    state.set(FortChessState::BoardScreen).unwrap();
}

/// Runs every frame of the game to check if the board needs to update graphics. Draws pieces as
/// well as highlights.
fn game_update_tick(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
    font:           Res<BoldFontHandle>,
    sprite:         Res<PlayerSheet>,
    mut pname:      ResMut<PlayerNameBoxVec>,
    mut state:      ResMut<State<FortChessState>>,
    dquery:         Query<Entity, With<Piece>>,
    hquery:         Query<Entity, With<Highlight>>,
    pnquery:        Query<Entity, With<PlayerName>>,
    pnhquery:       Query<Entity, With<PlayerNameOutline>>,
) {
    if !game.get().update { return }
    clean_up_lost_players(game.get_mut(), &mut pname);
    if game.get().players.len().eq(&1_usize) {
        game
            .get_mut()
            .next_player()
            .set_play_false()
            .current_player_mut()
            .set_winner();
    }
    if !game.get().play {
        let _throw = state.set(FortChessState::ResultScreen);
        return;
    }
    game.get_mut().set_update_false();
    draw_pieces(            &mut commands, &sprite, &game, &dquery);
    highlight_active_pieces(&mut commands, &game,          &hquery);
    display_player_names(   &mut commands, &pname,         &pnquery, &font);
    highlight_player_name(  &mut commands, &pname,  &game, &pnhquery);
}

/// Looks for players and kills them at every iteration.
fn clean_up_lost_players(
    game:   &mut Game,
    pname:  &mut ResMut<PlayerNameBoxVec>,
) {
    let _dead = game.hunt();
    if !_dead.is_empty() { dbg!(&_dead); }
    _dead
        .into_iter()
        .for_each(|player| pname.pop(player.team))
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
            Vec2::splat(SPRITESIZE * 2_f32),
            PIECE_TYPE_COUNT, // Rows.
            PIECE_TYPE_COUNT, // Columns.
            Vec2::splat(0_f32),
            Vec2::splat(0_f32),
        ),
    )));
}
/*-----------------------------------------------------------------------------------------------*/
