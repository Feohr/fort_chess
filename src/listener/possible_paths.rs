//! possible paths module.
//!
//! Handles the logic for piece possible paths and their movements.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::{
        Color, Commands, Component, Entity, Query, ResMut, Vec3, With,
};
use fort_builders::{
    pieces::PieceType,
    board::{
       Quadrant, position_in_q1_bounds, position_in_q2_bounds,
       position_in_q3_bounds, position_in_board_bounds,
    },
    game::Game,
    player::PlayerAction,
};
use crate::{
    RESOLUTION, ZAxisLevel,
    listener::spawn_square_sprite,
};

const PPATHS_COLOR_EMPTY: Color = Color::rgb(0.9, 0.9, 0.6);
const PPATHS_COLOR_PIECE: Color = Color::ORANGE_RED;

#[derive(Debug, Component)]
pub struct PossiblePaths {
    pub(crate) paths: Vec<(f32, f32)>,
}

#[derive(Component)]
pub struct Paths;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

pub fn possible_piece_paths(
    x:                  f32,
    y:                  f32,
    _piece_type:         PieceType,
    game:               &Game,
) -> Vec<(f32, f32)> {

    let path_analysis: fn(f32, f32, &Game) -> Vec<(f32, f32)> = analyse_rook_paths;

    match Quadrant::from_xy(x, y).unwrap() {
        Quadrant::Q1 => {
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q1_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>()
        },
        Quadrant::Q2 => {
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q2_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>()
        },
        Quadrant::Q3 => {
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q3_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>()
        },
    }

}

fn analyse_rook_paths(x: f32, y: f32, game: &Game) -> Vec<(f32, f32)> {

    let mut _possiblepaths: Vec<(f32, f32)> = Vec::new();

    // Steps Along +ve X-axis.
    iter_rook_path_step_analysis(x, y, | x, _y| { *x += 1.0 }, game, &mut _possiblepaths);
    // Steps Along -ve X-axis.
    iter_rook_path_step_analysis(x, y, | x, _y| { *x -= 1.0 }, game, &mut _possiblepaths);
    // Steps Along +ve Y-axis.
    iter_rook_path_step_analysis(x, y, |_x,  y| { *y += 1.0 }, game, &mut _possiblepaths);
    // Steps Along -ve Y-axis.
    iter_rook_path_step_analysis(x, y, |_x,  y| { *y -= 1.0 }, game, &mut _possiblepaths);

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

/*████PossiblePaths████*/
/*-----------------------------------------------------------------------------------------------*/
impl PossiblePaths {

    fn update_paths(&mut self, mut paths: Vec<(f32, f32)>) {
        self.clear();
        self.paths = paths.drain(0..).collect::<Vec<(f32, f32)>>();
    }

    pub fn clear(&mut self) { self.paths.clear() }

    pub fn get(&self) -> &Vec<(f32, f32)> { &self.paths }

}

pub fn clear_possible_piece_paths(
    commands:   &mut Commands,
    paths:      &Query<Entity, With<Paths>>,
) {

    for path in paths {
        commands.entity(path).despawn();
    }

}

pub fn draw_possible_piece_paths(
    commands:       &mut Commands,
    paths:          &PossiblePaths,
    paths_query:    &Query<Entity, With<Paths>>,
    game:           &Game,
) {

    clear_possible_piece_paths(commands, paths_query);

    for step in paths.get().iter() {

        let (step_x, step_y) = (step.0 * RESOLUTION, step.1 * RESOLUTION);

        let step_block = spawn_square_sprite(
            commands,
            piece_in_step_detection(step.0, step.1, game),
            Vec3::new(step_x, step_y, ZAxisLevel::Seventh.as_f32()),
        );

        commands.entity(step_block).insert(Paths);

    }

}

pub fn update_possible_piece_paths(game: &Game, paths: &mut ResMut<PossiblePaths>) {

    let (piece_pos_x, piece_pos_y, piece_type) = {

        let piece = game.current_player().current_chosen_piece();

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

fn piece_in_step_detection(x: f32, y: f32, game: &Game) -> Color {

    match game.check_piece_in_pos(x, y) {
        true  => PPATHS_COLOR_PIECE,
        false => PPATHS_COLOR_EMPTY,
    }

}
/*-----------------------------------------------------------------------------------------------*/
