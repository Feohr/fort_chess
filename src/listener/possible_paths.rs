//! possible paths module.
//!
//! Handles the logic for piece possible paths and their movements.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

//----------//
mod knight;
mod minister;
mod pawn;
mod queen;
mod rook;
//----------//

use crate::{despawn_entity::DespawnEntity, listener::spawn_square_sprite, ZAxisLevel, RESOLUTION};
use bevy::prelude::{Color, Commands, Component, Entity, Query, ResMut, Vec3, With};
use fort_builders::{
    game::{Game, GameAction},
    pieces::PieceType,
};
use knight::analyse_knight_paths;
use minister::analyse_minister_paths;
use pawn::analyse_pawn_paths;
use queen::analyse_queen_paths;
use rook::analyse_rook_paths;

/// The color of the [`PossiblePaths`] that do not have a piece.
const PPATHS_COLOR_EMPTY: Color = Color::rgb(0.9_f32, 0.9_f32, 0.6_f32);
/// The color of [`PossiblePaths`] that have a piece.
const PPATHS_COLOR_PIECE: Color = Color::PURPLE;
/// The step size just holds 1.0 as the value. Not necessary but I do a lot of unnecessary stuff.
const STEP: f32 = 1_f32;

/// Type to hold a vector of tuple with `f32` x and y positions.
type PositionVectorf32 = Vec<(f32, f32)>;

/// A resource to hold a vector with PossiblePaths of each piece.
#[derive(Debug)]
pub struct PossiblePaths {
    pub(crate) paths: PositionVectorf32,
}
/// A component to denote enitity with Path.
#[derive(Component)]
pub struct Paths;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To detect possible paths of a piece.
pub(crate) fn possible_piece_paths(
    x: f32,
    y: f32,
    piece_type: PieceType,
    game: &Game,
) -> PositionVectorf32 {
    (match piece_type {
        PieceType::Rook => analyse_rook_paths,
        PieceType::Pawn => analyse_pawn_paths,
        PieceType::Knight => analyse_knight_paths,
        PieceType::Minister => analyse_minister_paths,
        PieceType::Queen => analyse_queen_paths,
    })(x, y, game)
}

/*████PossiblePaths████*/
/*-----------------------------------------------------------------------------------------------*/
impl PossiblePaths {
    /// Calculates and updates the paths value to render.
    #[inline]
    fn update_paths(&mut self, paths: PositionVectorf32) {
        self.paths = paths;
    }
    /// Empties the paths vector.
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.paths.clear();
    }
    /// Returns a reference to the internal vector.
    #[inline]
    pub(crate) fn get(&self) -> &PositionVectorf32 {
        &self.paths
    }
    /// searches the paths to see if the position exists.
    #[inline]
    pub(crate) fn contains(&self, x: f32, y: f32) -> bool {
        self.get().contains(&(x, y))
    }
}

/// To draw the paths whenever a piece is chosen.
pub(crate) fn draw_possible_piece_paths(
    commands: &mut Commands,
    paths: &PossiblePaths,
    paths_query: &Query<Entity, With<Paths>>,
    game: &Game,
) {
    commands.despawn_entity(paths_query);
    paths.get().iter().for_each(|step| {
        let step_block = spawn_square_sprite(
            commands,
            piece_in_step_detection(step, game),
            Vec3::new(
                step.0 * RESOLUTION,
                step.1 * RESOLUTION,
                ZAxisLevel::Seventh.as_f32(),
            ),
        );
        commands.entity(step_block).insert(Paths);
    });
}

/// To update the possible paths whenever a piece is chosen. The paths are derived from
/// `possible_piece_paths` function that returns a vector of position tuples.
pub(crate) fn update_possible_piece_paths(game: &Game, paths: &mut ResMut<PossiblePaths>) {
    let (piece_pos_x, piece_pos_y, piece_type) = {
        let piece = game.current_player().current_chosen_piece().unwrap();
        (
            piece.position.x as f32,
            piece.position.y as f32,
            piece.piece_type,
        )
    };
    paths.update_paths(possible_piece_paths(
        piece_pos_x,
        piece_pos_y,
        piece_type,
        game,
    ));
}

/// To detect if a position has a piece and return the appropriate color. For position with pieces
/// it returns Red else Yellow.
#[inline]
fn piece_in_step_detection(step: &(f32, f32), game: &Game) -> Color {
    match game.check_piece_in_pos(step.0, step.1) {
        true => PPATHS_COLOR_PIECE,
        false => PPATHS_COLOR_EMPTY,
    }
}
/*-----------------------------------------------------------------------------------------------*/
