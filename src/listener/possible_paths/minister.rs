//! minister module.
//!
//! Handles the minister paths analysis.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::listener::possible_paths::PositionVectorf32;
use fort_builders::{
    board::{position_in_q1_bounds, position_in_q2_bounds, position_in_q3_bounds, Quadrant},
    game::{Game, GameAction},
    player::PlayerAction,
    BREADTH,
};

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// Minister traverses diagonally.
///
/// These traversals are easy to calculate by taking a breadth of the square and checking it's
/// corner. I have decided to put the limit at 4 blocks as there are no positions possible beyond
/// for at a given time inside a quadrant bound.
///
/// There remaining positions are then filtered out based on the qudrant that the piece lies
/// inside.
pub(crate) fn analyse_minister_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {
    let mut _possiblepaths: PositionVectorf32 = Vec::new();
    minister_step_analysis(
        x,
        y,
        |x, y, breadth| (x + breadth as f32, y + breadth as f32),
        game,
        &mut _possiblepaths,
    );
    minister_step_analysis(
        x,
        y,
        |x, y, breadth| (x + breadth as f32, y - breadth as f32),
        game,
        &mut _possiblepaths,
    );
    minister_step_analysis(
        x,
        y,
        |x, y, breadth| (x - breadth as f32, y + breadth as f32),
        game,
        &mut _possiblepaths,
    );
    minister_step_analysis(
        x,
        y,
        |x, y, breadth| (x - breadth as f32, y - breadth as f32),
        game,
        &mut _possiblepaths,
    );
    _possiblepaths
        .into_iter()
        .filter(|(_x, _y)| {
            (match Quadrant::from_xy(x, y).unwrap() {
                Quadrant::Q1 => position_in_q1_bounds,
                Quadrant::Q2 => position_in_q2_bounds,
                Quadrant::Q3 => position_in_q3_bounds,
                _ => panic!("Cannot analyse minister paths for pieces in \'NoQuad\' Quadrant."),
            })(*_x, *_y)
        })
        .collect::<PositionVectorf32>()
}

fn minister_step_analysis<F>(
    x: f32,
    y: f32,
    step: F,
    game: &Game,
    _possiblepaths: &mut PositionVectorf32,
) where
    F: Fn(f32, f32, i32) -> (f32, f32),
{
    for breadth in 1..(BREADTH * 2_i32) {
        let (x, y) = step(x, y, breadth);
        if game.current_player().piece_index_from_xy_f32(x, y).is_ok() {
            break;
        }
        _possiblepaths.push((x, y));
        if game.check_piece_in_pos(x, y) {
            break;
        }
    }
}
