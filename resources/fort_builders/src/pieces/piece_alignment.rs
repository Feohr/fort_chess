//! # piece alignment module
//!
//! Holds the chess piece alignments and their type for each player.

use crate::board::Quadrant;

type PosInfo = Vec<(i32, i32)>;
type TypeInfo = Vec<u8>;
type PieceInfo = (PosInfo, TypeInfo);

// The defender.
fn get_def_pos() -> PosInfo {
    vec![ 
        (-3, -2), (-3, -1), (-3,  0), (-3,  1), (-4, -2), (-4, -1), (-4,  0), (-4,  1),
        (-2,  2), (-1,  2), ( 0,  2), ( 1,  2), (-2,  3), (-1,  3), ( 0,  3), ( 1,  3),
        ( 2,  1), ( 2,  0), ( 2, -1), ( 2, -2), ( 3,  1), ( 3,  0), ( 3, -1), ( 3, -2),
    ]
}

// The enemies.
fn get_enm_pos_q1() -> PosInfo {
    vec![
        (-7, -2), (-7, -1), (-7,  0), (-7,  1), (-8, -2), (-8, -1), (-8,  0), (-8,  1),
    ]
}

fn get_enm_pos_q2() -> PosInfo {
    vec![
        (-2,  6), (-1,  6), ( 0,  6), ( 1,  6), (-2,  7), (-1,  7), ( 0,  7), ( 1,  7),
    ]
}

fn get_enm_pos_q3() -> PosInfo {
    vec![
        ( 6,  1), ( 6,  0), ( 6, -1), ( 6, -2), ( 7,  1), ( 7,  0), ( 7, -1), ( 7, -2),
    ]
}

// piece type index
fn get_enm_type() -> TypeInfo {
    vec![
        3, 3, 3, 3, 4, 3, 3, 4,
    ]
}

fn get_def_type() -> TypeInfo {
    vec![
        4, 1, 2, 0, 3, 3, 3, 3,
        4, 2, 1, 4, 3, 3, 3, 3,
        0, 1, 2, 4, 3, 3, 3, 3,
    ]
}

fn get_piece_type(is_defender: bool) -> TypeInfo {
    match is_defender {
        true  => get_def_type(),
        false => get_enm_type(),
    }
}

fn get_pos_from_quadrant(is_defender: bool, quadrant: &Quadrant) -> PosInfo {
    match is_defender {
        true => get_def_pos(),
        false => match quadrant {
            Quadrant::Q1 => get_enm_pos_q1(),
            Quadrant::Q2 => get_enm_pos_q2(),
            Quadrant::Q3 => get_enm_pos_q3(),
        },
    }
}

pub fn get_resp_info(is_defender: bool, quadrant: Quadrant) -> PieceInfo {
    (
        get_pos_from_quadrant(is_defender, &quadrant),
        get_piece_type(is_defender),
    )
}
