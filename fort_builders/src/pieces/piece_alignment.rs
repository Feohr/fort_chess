//! # piece alignment module
//!
//! Holds the chess piece alignments and their type for each player.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::board::Quadrant;
use crate::pieces::ENEMY_COUNT;

/// Type to abstract the position tuple.
type PositionVectori32 = Vec<(i32, i32)>;
/// Type to abstract the type *u8* value.
type PieceTypeVectoru8 = Vec<u8>;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To return the defender position inside a board based on the number of players.
fn defender_position(quadrant_active: usize) -> PositionVectori32 {

    (1..quadrant_active)
        .into_iter()
        .flat_map(|index| match index {
            1_usize => vec![
                (-3_i32, -2_i32), (-3_i32, -1_i32), (-3_i32,  0_i32), (-3_i32,  1_i32),
                (-4_i32, -2_i32), (-4_i32, -1_i32), (-4_i32,  0_i32), (-4_i32,  1_i32),
            ],
            2_usize => vec![
                (-2_i32,  2_i32), (-1_i32,  2_i32), ( 0_i32,  2_i32), ( 1_i32,  2_i32),
                (-2_i32,  3_i32), (-1_i32,  3_i32), ( 0_i32,  3_i32), ( 1_i32,  3_i32),
            ],
            3_usize => vec![
                ( 2_i32,  1_i32), ( 2_i32,  0_i32), ( 2_i32, -1_i32), ( 2_i32, -2_i32),
                ( 3_i32,  1_i32), ( 3_i32,  0_i32), ( 3_i32, -1_i32), ( 3_i32, -2_i32),
            ],
            _ => panic!("There can't be more than 4 players. index: {}.", index),
        })
        .collect::<PositionVectori32>()

}

// The enemies.
/// Returns the non-defender player positions in [`Quadrant::Q1`].
#[inline]
fn enemy_position_q1() -> PositionVectori32 {
    vec![
        (-7_i32, -2_i32), (-7_i32, -1_i32), (-7_i32,  0_i32), (-7_i32,  1_i32),
        (-8_i32, -2_i32), (-8_i32, -1_i32), (-8_i32,  0_i32), (-8_i32,  1_i32),
    ]
}

/// Returns the non-defender player positions in [`Quadrant::Q2`].
#[inline]
fn enemy_position_q2() -> PositionVectori32 {
    vec![
        (-2_i32,  6_i32), (-1_i32,  6_i32), ( 0_i32,  6_i32), ( 1_i32,  6_i32),
        (-2_i32,  7_i32), (-1_i32,  7_i32), ( 0_i32,  7_i32), ( 1_i32,  7_i32),
    ]
}

/// Returns the non-defender player positions in [`Quadrant::Q3`].
#[inline]
fn enemy_position_q3() -> PositionVectori32 {
    vec![
        ( 6_i32,  1_i32), ( 6_i32,  0_i32), ( 6_i32, -1_i32), ( 6_i32, -2_i32),
        ( 7_i32,  1_i32), ( 7_i32,  0_i32), ( 7_i32, -1_i32), ( 7_i32, -2_i32),
    ]
}

// piece type index
/// Returns `u8` value vector that corresponds to the [`PieceType`] value for non-defender type.
///
/// [`PieceType`]: crate::pieces::PieceType
#[inline]
fn enemy_type() -> PieceTypeVectoru8 {
    vec![3_u8, 3_u8, 3_u8, 3_u8, 4_u8, 3_u8, 3_u8, 4_u8]
}

/// Returns `u8` value vector that corresponds to the [`PieceType`] value for defender type.
///
/// [`PieceType`]: crate::pieces::PieceType
fn defender_type(quadrant_active: usize) -> PieceTypeVectoru8 {

    if quadrant_active > 4_usize {
        panic!("There can't be more than 4 players. index: {quadrant_active}.")
    }

    vec![4_u8, 1_u8, 2_u8, 0_u8, 3_u8, 3_u8, 3_u8, 3_u8]
        .into_iter()
        .cycle()
        .take(ENEMY_COUNT * quadrant_active)
        .collect::<PieceTypeVectoru8>()

}

/*-----------------------------------------------------------------------------------------------*/
/*████Public functions████*/
/*-----------------------------------------------------------------------------------------------*/
/// Returns the [`PieceType`] vector depending on wether the player is a a defender or not and what
/// qudrant the player resides in.
///
/// [`PieceType`]: crate::pieces::PieceType
#[inline]
pub(crate) fn piece_type(is_defender: bool, quadrant_active: usize) -> PieceTypeVectoru8 {
    match is_defender {
        true  => defender_type(quadrant_active),
        false => enemy_type(),
    }
}

/// Returns the [`Position`] vector values depending on if it is a defender or not and what
/// quadrant the player resides in.
///
/// [`Position`]: crate::pieces::Position
pub(crate) fn position_from_quadrant(
    quadrant:           &Quadrant,
    quadrant_active:    usize,
) -> PositionVectori32 {
        match quadrant {
            Quadrant::Q1     => enemy_position_q1(),
            Quadrant::Q2     => enemy_position_q2(),
            Quadrant::Q3     => enemy_position_q3(),
            Quadrant::NoQuad => defender_position(quadrant_active),
        }
}
/*-----------------------------------------------------------------------------------------------*/
