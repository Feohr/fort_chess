use fort_builders::{
    board::position_in_board_bounds,
    game::Game,
    player::PlayerAction,
};
use crate::listener::possible_paths::STEP;

pub fn analyse_rook_paths(x: f32, y: f32, game: &Game) -> Vec<(f32, f32)> {

    let mut _possiblepaths: Vec<(f32, f32)> = Vec::new();

    // Steps Along +ve X-axis.
    iter_rook_path_step_analysis(x, y, |x, _y| *x += STEP, game, &mut _possiblepaths);
    // Steps Along -ve X-axis.
    iter_rook_path_step_analysis(x, y, |x, _y| *x -= STEP, game, &mut _possiblepaths);
    // Steps Along +ve Y-axis.
    iter_rook_path_step_analysis(x, y, |_x, y| *y += STEP, game, &mut _possiblepaths);
    // Steps Along -ve Y-axis.
    iter_rook_path_step_analysis(x, y, |_x, y| *y -= STEP, game, &mut _possiblepaths);

    return _possiblepaths;

}

fn iter_rook_path_step_analysis<F>(
    mut _x:         f32,
    mut _y:         f32,
    step:           F,
    game:           &Game,
    _possiblepaths: &mut Vec<(f32, f32)>,
) where
    F: Fn(&mut f32, &mut f32),
{

    loop {

        step(&mut _x, &mut _y);

        if game.current_player().piece_index_from_xy_f32(_x, _y).is_ok()        { break }

        _possiblepaths.push((_x, _y));

        if !position_in_board_bounds(_x, _y) || game.check_piece_in_pos(_x, _y) { break }

    }

}
