//! # piece alignment module
//!
//! Holds the chess piece alignments and their type for each player.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::board::Quadrant;

type PosInfo = Vec<(i32, i32)>;
type TypeInfo = Vec<u8>;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

// The defender.
fn get_def_pos(quadrant_active: usize) -> PosInfo {
(1..quadrant_active)
        .into_iter()
        .flat_map(|i| {
            match i {
                1 => vec![
                    (-3, -2), (-3, -1), (-3,  0), (-3,  1), (-4, -2), (-4, -1), (-4,  0), (-4,  1),
                ],
                2 => vec![
                    (-2,  2), (-1,  2), ( 0,  2), ( 1,  2), (-2,  3), (-1,  3), ( 0,  3), ( 1,  3),
                ],
                3 => vec![
                    ( 2,  1), ( 2,  0), ( 2, -1), ( 2, -2), ( 3,  1), ( 3,  0), ( 3, -1), ( 3, -2),
                ],
                _ => panic!("There can't be more than 4 players. index: {i}."),
            }
        })
        .collect::<PosInfo>()
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

fn get_def_type(quadrant_active: usize) -> TypeInfo {
    (1..quadrant_active)
        .into_iter()
        .flat_map(|i| {
            match i {
                1 => vec![4, 1, 2, 0, 3, 3, 3, 3],
                2 => vec![4, 2, 1, 4, 3, 3, 3, 3],
                3 => vec![0, 1, 2, 4, 3, 3, 3, 3],
                _ => panic!("There can't be more than 4 players. index: {i}."),
            }
        })
        .collect::<TypeInfo>()
}

/*████Public functions████*/
/*-----------------------------------------------------------------------------------------------*/
pub(crate) fn get_piece_type(is_defender: bool, quadrant_active: usize) -> TypeInfo {
    match is_defender {
        true  => get_def_type(quadrant_active),
        false => get_enm_type(),
    }
}

pub(crate) fn get_pos_from_quadrant(
    is_defender: bool,
    quadrant: &Quadrant,
    quadrant_active: usize,
) -> PosInfo {
    match is_defender {
        true => get_def_pos(quadrant_active),
        false => match quadrant {
            Quadrant::Q1 => get_enm_pos_q1(),
            Quadrant::Q2 => get_enm_pos_q2(),
            Quadrant::Q3 => get_enm_pos_q3(),
        },
    }
}
/*-----------------------------------------------------------------------------------------------*/
