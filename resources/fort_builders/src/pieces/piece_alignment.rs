//! # piece alignment module
//!
//! Holds the chess piece alignments and their type for each player.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::board::Quadrant;

/// Type to abstract the position tuple.
type PositionVectori32 = Vec<(i32, i32)>;
/// Type to abstract the type *u8* value.
type PieceTypeVectoru8 = Vec<u8>;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To return the defender position inside a board based on the number of players.
fn defender_position(quadrant_active: usize) -> PositionVectori32 {

    (1..quadrant_active)
        .into_iter()
        .flat_map(|i| match i {
            1 => vec![
                (-3, -2), (-3, -1), (-3,  0), (-3,  1), (-4, -2), (-4, -1), (-4,  0), (-4,  1),
            ],
            2 => vec![
                (-2,  2), (-1,  2), ( 0,  2), ( 1,  2), (-2,  3), (-1,  3), ( 0,  3), ( 1,  3),
            ],
            3 => vec![
                ( 2,  1), ( 2,  0), ( 2, -1), ( 2, -2), ( 3,  1), ( 3,  0), ( 3, -1), ( 3, -2),
            ],
            _ => panic!("There can't be more than 4 players. index: {}.", i),
        })
        .collect::<PositionVectori32>()

}

// The enemies.
/// Returns the non-defender player positions in [`Quadrant::Q1`].
fn enemy_position_q1() -> PositionVectori32 {

    vec![
        (-7, -2), (-7, -1), (-7,  0), (-7,  1), (-8, -2), (-8, -1), (-8,  0), (-8,  1),
    ]

}

/// Returns the non-defender player positions in [`Quadrant::Q2`].
fn enemy_position_q2() -> PositionVectori32 {

    vec![
        (-2,  6), (-1,  6), ( 0,  6), ( 1,  6), (-2,  7), (-1,  7), ( 0,  7), ( 1,  7),
    ]

}

/// Returns the non-defender player positions in [`Quadrant::Q3`].
fn enemy_position_q3() -> PositionVectori32 {

    vec![
        ( 6,  1), ( 6,  0), ( 6, -1), ( 6, -2), ( 7,  1), ( 7,  0), ( 7, -1), ( 7, -2),
    ]

}

// piece type index
/// Returns `u8` value vector that corresponds to the [`PieceType`] value for non-defender type.
///
/// [`PieceType`]: crate::pieces::PieceType
fn enemy_type() -> PieceTypeVectoru8 {

    vec![3, 3, 3, 3, 4, 3, 3, 4]

}

/// Returns `u8` value vector that corresponds to the [`PieceType`] value for defender type.
///
/// [`PieceType`]: crate::pieces::PieceType
fn defender_type(quadrant_active: usize) -> PieceTypeVectoru8 {

    (1..quadrant_active)
        .into_iter()
        .flat_map(|i| match i {
            1 => vec![4, 1, 2, 0, 3, 3, 3, 3],
            2 => vec![4, 2, 1, 4, 3, 3, 3, 3],
            3 => vec![0, 1, 2, 4, 3, 3, 3, 3],
            _ => panic!("There can't be more than 4 players. index: {i}."),
        })
        .collect::<PieceTypeVectoru8>()

}

/*-----------------------------------------------------------------------------------------------*/
/*████Public functions████*/
/*-----------------------------------------------------------------------------------------------*/
/// Returns the [`PieceType`] vector depending on wether the player is a a defender or not and what
/// qudrant the player resides in.
///
/// [`PieceType`]: crate::pieces::PieceType
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
    is_defender:        bool,
    quadrant:           &Quadrant,
    quadrant_active:    usize,
) -> PositionVectori32 {

    match is_defender {
        true  => defender_position(quadrant_active),
        false => match quadrant {
            Quadrant::Q1 => enemy_position_q1(),
            Quadrant::Q2 => enemy_position_q2(),
            Quadrant::Q3 => enemy_position_q3(),
        },
    }

}
/*-----------------------------------------------------------------------------------------------*/
