//! rook module
//!
//! To handle the rook possible paths analysis.
//!
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::{
    board::position_in_board_bounds,
    game::{Game, GameAction},
    player::PlayerAction,
};
use crate::listener::possible_paths::{STEP, PositionVectorf32};

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// Takes the given position and draws the possible rook path from there.
///
/// Checks for steps along `+ve X-axis`, `-ve X-axis`, `+ve Y-axis` and `-ve Y-axis`.
pub(crate) fn analyse_rook_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {
    let mut _possiblepaths: PositionVectorf32 = Vec::new();
    iter_rook_path_step_analysis(x, y, |_x, _y| *_x += STEP, game, &mut _possiblepaths);
    iter_rook_path_step_analysis(x, y, |_x, _y| *_x -= STEP, game, &mut _possiblepaths);
    iter_rook_path_step_analysis(x, y, |_x, _y| *_y += STEP, game, &mut _possiblepaths);
    iter_rook_path_step_analysis(x, y, |_x, _y| *_y -= STEP, game, &mut _possiblepaths);
    _possiblepaths
}

/// To analyse the rook path in a given direction using the step function.
///
/// Step and move in a given direction until the position is either in the current player pieces,
/// out of bound of the board or another player piece present. Else push to possible paths.
fn iter_rook_path_step_analysis<F>(
    mut _x:         f32,
    mut _y:         f32,
    step:           F,
    game:           &Game,
    _possiblepaths: &mut PositionVectorf32,
) where
        F: Fn(&mut f32, &mut f32),
{
    loop {
        step(&mut _x, &mut _y);
        if !position_in_board_bounds(_x, _y)
        || game.current_player().piece_index_from_xy_f32(_x, _y).is_ok() { break }
        _possiblepaths.push((_x, _y));
        if game.check_piece_in_pos(_x, _y) { break }
    }
}
