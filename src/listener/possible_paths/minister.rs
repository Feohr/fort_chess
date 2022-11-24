//! minister module.
//!
//! Handles the minister paths analysis.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::{
    board::{position_in_q1_bounds, position_in_q2_bounds, position_in_q3_bounds, Quadrant},
    game::{Game, GameAction},
    player::PlayerAction,
};
use crate::{
    tiles::BREADTH,
    listener::possible_paths::PositionVectorf32,
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

    // Initializing possible paths vector.
    let mut _possiblepaths: PositionVectorf32 = Vec::new();

    // Along +ve X and +ve y diagonal.
    minister_step_analysis(
        x, y,   |x, y, breadth| {
                    (x + breadth as f32,
                     y + breadth as f32,)
                },
        game, &mut _possiblepaths,
    );
    // Along +ve X and -ve y diagonal.
    minister_step_analysis(
        x, y,   |x, y, breadth| {
                    (x + breadth as f32,
                     y - breadth as f32,)
                },
        game, &mut _possiblepaths,
    );
    // Along -ve X and +ve y diagonal.
    minister_step_analysis(
        x, y,   |x, y, breadth| {
                    (x - breadth as f32,
                     y + breadth as f32,)
                },
        game, &mut _possiblepaths,
    );
    // Along -ve X and -ve y diagonal.
    minister_step_analysis(
        x, y,   |x, y, breadth| {
                    (x - breadth as f32,
                     y - breadth as f32,)
                },
        game, &mut _possiblepaths,
    );

    // Return.
    _possiblepaths  .into_iter()
                    .filter(|(_x, _y)| (
                    // Fetching the appropriate filter function based on the x and y location.
                    match Quadrant::from_xy(x, y).unwrap() {
                        Quadrant::Q1 => position_in_q1_bounds,
                        Quadrant::Q2 => position_in_q2_bounds,
                        Quadrant::Q3 => position_in_q3_bounds,
                        _            => panic!(
                            "Cannot analyse minister paths for pieces in \'NoQuad\' Quadrant."
                        ),
                    })(*_x, *_y))
                    .collect::<PositionVectorf32>()

}

fn minister_step_analysis<F>(
    x:              f32,
    y:              f32,
    step:           F,
    game:           &Game,
    _possiblepaths: &mut PositionVectorf32,
) where
        F: Fn(f32, f32, i32) -> (f32, f32),
{

    // Looping over the breadth of a single quadrant.
    for breadth in 1..(BREADTH * 2_i32) {

        // To get the x and y for this step.
        let (x, y) = step(x, y, breadth);

        // If the same team piece exists at the step stop.
        if game.current_player().piece_index_from_xy_f32(x, y).is_ok()  { break }

        // Push.
        _possiblepaths.push((x, y));

        // If there was an enemy piece at the position then stop.
        if game.check_piece_in_pos(x, y)                                { break }

    }

}
