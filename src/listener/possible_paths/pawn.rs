use fort_builders::{
    board::Quadrant,
    game::Game,
    player::PlayerAction,
};
use crate::listener::possible_paths::STEP;

type PawnClosure = Box<dyn Fn(&mut f32, &mut f32)>;

pub fn analyse_pawn_paths(x: f32, y: f32, game: &Game) -> Vec<(f32, f32)> {

    let mut possiblepaths: Vec<(f32, f32)> = Vec::new();

    let quadrant = Quadrant::from_xy(x, y).unwrap();
    let is_defender = game.current_player().is_defender;

    let closure: PawnClosure = match is_defender {
        true  =>    match quadrant {
                        Quadrant::Q1 => Box::new(| x, _y| *x -= STEP),
                        Quadrant::Q2 => Box::new(|_x,  y| *y += STEP),
                        Quadrant::Q3 => Box::new(| x, _y| *x += STEP),
                    },
        false =>    match quadrant {
                        Quadrant::Q1 => Box::new(| x, _y| *x += STEP),
                        Quadrant::Q2 => Box::new(|_x,  y| *y -= STEP),
                        Quadrant::Q3 => Box::new(| x, _y| *x -= STEP),
                    },
    };

    iter_pawn_path_step_analysis(
        x,
        y,
        is_defender,
        quadrant,
        closure,
        game,
        &mut possiblepaths,
    );

    return possiblepaths;

}

fn iter_pawn_path_step_analysis<F>(
    mut _x:         f32,
    mut _y:         f32,
    _is_defender:   bool,
    qudrant:        Quadrant,
    step:           F,
    game:           &Game,
    possiblepaths: &mut Vec<(f32, f32)>,
) where
    F: Fn(&mut f32, &mut f32),
{

    step(&mut _x, &mut _y);

    match qudrant {
       Quadrant::Q1 | Quadrant::Q3 => {
            // +ve Y-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x, _y + STEP, game, possiblepaths);
            // -ve Y-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x, _y - STEP, game, possiblepaths);
        },
        Quadrant::Q2               => {
            // +ve X-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x + STEP, _y, game, possiblepaths);
            // -ve X-axis Diagonal check.
            pawn_possible_path_if_piece_at_pos(_x - STEP, _y, game, possiblepaths);
        },
    }

    if game.check_piece_in_pos(_x, _y)                                  { return }
    if game.current_player().piece_index_from_xy_f32(_x, _y).is_ok()    { return }

    possiblepaths.push((_x, _y));

}

/// Used to detect pieces for mainly pawns.
fn pawn_possible_path_if_piece_at_pos(
    x:              f32,
    y:              f32,
    game:           &Game,
    possiblepaths: &mut Vec<(f32, f32)>,
) {

    if !game.check_piece_in_pos(x, y)                               { return }
    if game.current_player().piece_index_from_xy_f32(x, y).is_ok()  { return }

    possiblepaths.push((x, y));

}
