//! knight module.
//!
//! Handles the knight's possible paths analysis.
//!
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::{
    board::{position_in_q1_bounds, position_in_q2_bounds, position_in_q3_bounds, Quadrant},
    game::Game,
    player::PlayerAction,
};
use crate::listener::possible_paths::{STEP, PositionVectorf32};

/// To hold the value of `360`.
const FULL_CIRCLE:  usize   = 360;
/// To get the circle scan increment step.
const CIRCLE_STEP:  usize   = 30;
/// To get the radius of the circle scanner.
const RADIUS:       f32     = STEP * 2.0;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To analyse paths for the knight from a given position.
///
/// when drawn on a graph paper, I noticed that the positions seem to be along a circumference of a
/// circle of 2 steps/units radius with the knight in the middle.
///
/// The function scans the paths in a circle. We start by looping over `360` degrees and
/// incrementing the angle at a step of `30` degrees with each iteration. We then check at each
/// iteration the x and y value on the circumference. This is achieved by computing the formulas
/// `circle_x = radius * sin(t)` and `circle_y = radius * cos(t)` where t is theta and radius is
/// the radius of the circle that needs to be scanned.
///
/// These x and y values are then offset by the position of the center(knight's position) to get
/// the resultant circumference positions.
///
/// To ignore the middle "cross" path like the knights in chess, we simply check if either x or y
/// is same as the `circle_x` or `circle_y` respectively. Which means that the the angle is either
/// `90` or '270' degress or '180' or '360'/'0' degrees. These are the angles we are supposed to
/// skip, hence we do.
///
/// Any positions with the pieces of the same team are also skipped.
pub(crate) fn analyse_knight_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {

    let mut _possiblepaths: PositionVectorf32 = Vec::new();

    for theta in (0..FULL_CIRCLE).step_by(CIRCLE_STEP) {

        let path_x = ((theta as f32).to_radians().sin() * RADIUS).round() + x;
        let path_y = ((theta as f32).to_radians().cos() * RADIUS).round() + y;

        if path_x == x.round()
        || path_y == y.round()
        || game.current_player().piece_index_from_xy_f32(path_x, path_y).is_ok() { continue }

        _possiblepaths.push((path_x, path_y));

    }

    let filter_function = match Quadrant::from_xy(x, y).unwrap() {
        Quadrant::Q1 => position_in_q1_bounds,
        Quadrant::Q2 => position_in_q2_bounds,
        Quadrant::Q3 => position_in_q3_bounds,
    };

    _possiblepaths  .into_iter()
                    .filter(|(x, y)| filter_function(*x, *y))
                    .collect::<PositionVectorf32>()

}
