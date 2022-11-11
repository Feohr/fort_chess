//! possible paths module.
//!
//! Handles the logic for piece possible paths and their movements.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod pawn;
mod rook;

use crate::{listener::spawn_square_sprite, ZAxisLevel, RESOLUTION};
use bevy::prelude::{Color, Commands, Component, Entity, Query, ResMut, Vec3, With};
use fort_builders::{
    board::{position_in_q1_bounds, position_in_q2_bounds, position_in_q3_bounds, Quadrant},
    game::Game,
    pieces::PieceType,
};
use pawn::analyse_pawn_paths;
use rook::analyse_rook_paths;

const PPATHS_COLOR_EMPTY: Color = Color::rgb(0.9, 0.9, 0.6);
const PPATHS_COLOR_PIECE: Color = Color::ORANGE_RED;
const STEP              : f32   = 1.0;

#[derive(Debug, Component)]
pub struct PossiblePaths {
    pub(crate) paths: Vec<(f32, f32)>,
}

#[derive(Component)]
pub struct Paths;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

pub fn possible_piece_paths(
    x:          f32,
    y:          f32,
    piece_type: PieceType,
    game:       &Game,
) -> Vec<(f32, f32)> {

    let path_analysis = possible_paths_closure_from_piece_type(piece_type);

    match Quadrant::from_xy(x, y).unwrap() {

        Quadrant::Q1 =>
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q1_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>(),

        Quadrant::Q2 =>
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q2_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>(),

        Quadrant::Q3 =>
            path_analysis(x, y, game)
                .into_iter()
                .filter(|(x, y)| position_in_q3_bounds(*x, *y))
                .collect::<Vec<(f32, f32)>>(),

    }

}

fn possible_paths_closure_from_piece_type(
    piece_type: PieceType,
) -> Box<dyn Fn(f32, f32, &Game) -> Vec<(f32, f32)>> {

    match piece_type {
        PieceType::Rook => Box::from(analyse_rook_paths),
        PieceType::Pawn => Box::from(analyse_pawn_paths),
        // Empty closure placeholder.
        _ => Box::new(|_x: f32, _y: f32, _g: &Game| Vec::new()),
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

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.get().contains(&(x, y))
    }

}

pub fn clear_possible_piece_paths(
    commands: &mut Commands,
    paths: &Query<Entity, With<Paths>>,
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

pub fn update_possible_piece_paths(
    game: &Game,
    paths: &mut ResMut<PossiblePaths>,
) {

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
