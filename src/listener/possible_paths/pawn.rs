//! pawn module.
//!
//! Handles the pawn's possible paths analysis.
//!
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::{
    board::Quadrant,
    game::Game,
    player::PlayerAction,
};
use crate::listener::possible_paths::{STEP, PositionVectorf32};

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// Function to analyse possible pawn paths from a given position.
///
/// ### If the peice is:
/// #### Defender:
///
/// if inside [`Quadrant::Q1`]: Then we decrement the pawn steps in `x-axis` as it moves
/// towards the player along the `negative x-axis`.
///
/// if inside [`Quadrant::Q2`]: Then we increment the pawn steps in `y-axis` as it moves
/// towards the player along the `y-axis`.
///
/// if inside [`Quadrant::Q3`]: Then we increment steps in `x-axis` as it moves towards the
/// opponent along the `positive x-axis`.
///
/// #### Not defender:
///
/// if inside [`Quadrant::Q1`]: Then we increment the pawn steps in `x-axis` as it move
/// towards the defender along the `negative x-axis`.
///
/// if inside [`Quadrant::Q2`]: Then we decrement the pawn steps in `y-axis` as it moves
/// towards the defender along the `y-axis`.
///
/// if inside [`Quadrant::Q3`]: Then we decrement steps in `x-axis` as it moves towards
/// the defender along the `positive x-axis`.
pub(crate) fn analyse_pawn_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {

    // Initializing paths.
    let mut _possiblepaths: PositionVectorf32 = Vec::new();

    let quadrant    = Quadrant::from_xy(x, y).unwrap();
    let is_defender = game.current_player().is_defender;

    // Getting the quadrant information and mapping the appropriate closure to calculate.
    let pawn_closure = match is_defender {
        true  =>    match quadrant {
                        Quadrant::Q1 => | x: &mut f32, _y: &mut f32| *x -= STEP,
                        Quadrant::Q2 => |_x: &mut f32,  y: &mut f32| *y += STEP,
                        Quadrant::Q3 => | x: &mut f32, _y: &mut f32| *x += STEP,
                    },
        false =>    match quadrant {
                        Quadrant::Q1 => | x: &mut f32, _y: &mut f32| *x += STEP,
                        Quadrant::Q2 => |_x: &mut f32,  y: &mut f32| *y -= STEP,
                        Quadrant::Q3 => | x: &mut f32, _y: &mut f32| *x -= STEP,
                    },
    };

    // To calculate the steps after getting the appropriate steps.
    iter_pawn_path_step_analysis(
        x,
        y,
        is_defender,
        quadrant,
        pawn_closure,
        game,
        &mut _possiblepaths,
    );

    // Return.
    _possiblepaths

}

/// To handle the pawn step analysis and find the killable pieces.
///
/// The pawn possible killable pieces detection. After taking a step front, check for diagonal
/// pieces that can be killed.
fn iter_pawn_path_step_analysis<F>(
    mut _x:         f32,
    mut _y:         f32,
    _is_defender:   bool,
    qudrant:        Quadrant,
    step:           F,
    game:           &Game,
    _possiblepaths: &mut PositionVectorf32,
) where
    F: Fn(&mut f32, &mut f32),
{

    // Execute the closure to take a step.
    step(&mut _x, &mut _y);

    match qudrant {
       Quadrant::Q1 | Quadrant::Q3 => {
            // +ve Y-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x, _y + STEP, game, _possiblepaths);
            // -ve Y-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x, _y - STEP, game, _possiblepaths);
        },
        Quadrant::Q2               => {
            // +ve X-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x + STEP, _y, game, _possiblepaths);
            // -ve X-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x - STEP, _y, game, _possiblepaths);
        },
    }

    // Check if there is a piece in the current position and return of true.
    // This is the straight path part of the pawn.
    if game.check_piece_in_pos(_x, _y) { return }

    _possiblepaths.push((_x, _y));

}

/// Used to detect pawn's killable pieces.
///
/// check if there is piece in the given position or not. If true then register that piece.
/// OR check if there is a piece from the current palyer's pieces and return if true.
///
/// This is an arbitrary function written to reduce code clutter. As in there is nothingin
/// particular that this function does specific to pawn pieces.
fn pawn_possible_path_if_piece_at_pos(
    x:              f32,
    y:              f32,
    game:           &Game,
    _possiblepaths: &mut PositionVectorf32,
) {

    if !game.check_piece_in_pos(x, y)
    ||  game.current_player().piece_index_from_xy_f32(x, y).is_ok() { return }

    // Finally push the resultant position to PossiblePaths.
    _possiblepaths.push((x, y));

}
