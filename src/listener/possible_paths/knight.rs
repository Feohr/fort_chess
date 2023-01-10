//! knight module.
//!
//! Handles the knight's possible paths analysis.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::{
    board::{position_in_q1_bounds, position_in_q2_bounds, position_in_q3_bounds, Quadrant},
    game::Game,
    player::PlayerAction,
};
use crate::listener::possible_paths::PositionVectorf32;

mod circle {
    /// The circle's min angle value that holds `0`.
    pub(crate) const ANGLE_START    : usize = 0_usize;
    /// To hold the value of `360`.
    pub(crate) const ANGLE_END      : usize = 360_usize;
    /// To get the circle scan increment step.
    pub(crate) const ANGLE_STEP     : usize = 30_usize;
    /// To get the radius of the circle scanner.
    pub(crate) const RADIUS         : f32   = 2_f32;
}

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
/// `90`, `270`, `180` or `360`/`0` degrees. These are the angles we are supposed to skip, hence we
/// do.
///
/// Any positions with the pieces of the same team are also skipped.
pub(crate) fn analyse_knight_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {
    let mut _possiblepaths: PositionVectorf32 = Vec::new();
    // Looping from 0 to 360 with 30 as the step.
    for theta in (circle::ANGLE_START..circle::ANGLE_END).step_by(circle::ANGLE_STEP) {
        // Getting the circumeference x and y values.
        let path_x = ((theta as f32).to_radians().sin() * circle::RADIUS).round() + x;
        let path_y = ((theta as f32).to_radians().cos() * circle::RADIUS).round() + y;
        // Checking if they are either equal to the piece's x and y value or if the position
        // consists of a piece from the same team.
        if path_x == x.round()
        || path_y == y.round()
        || game.current_player().piece_index_from_xy_f32(path_x, path_y).is_ok() { continue }
        // Pushing to the _possiblepaths vec.
        _possiblepaths.push((path_x, path_y));
    }
    // Return.
    _possiblepaths  .into_iter()
                    .filter(|(_x, _y)| (
                    // Fetching the appropriate filter function based on the x and y location.
                    match Quadrant::from_xy(x, y).unwrap() {
                        Quadrant::Q1 => position_in_q1_bounds,
                        Quadrant::Q2 => position_in_q2_bounds,
                        Quadrant::Q3 => position_in_q3_bounds,
                        _            => panic!(
                            "Cannot analyse paths for a piece in \'NoQuad\' Quadrant."
                        ),
                    })(*_x, *_y))
                    .collect::<PositionVectorf32>()
}
