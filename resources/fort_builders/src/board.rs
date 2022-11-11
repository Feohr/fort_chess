//! # board module
//!
//! module to hold board specific values like dimensions etc.
//!
//! Quadrant 1: left block
//! Quadrant 2: top block
//! Quadrant 3: right block
//!
//! ## Contents:
//! -   X_MAX       (const)
//! -   X_MIN       (const)
//! -   Y_MAX       (const)
//! -   Y_MIN       (const)
//! -   RGT         (const)
//! -   LFT         (const)
//! -   TOP         (const)
//! -   BTM         (const)
//! -   Quadrant    (enum)

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::Error;
// use crate::pieces::{Piece, PieceType, Position};

// Board tile borders.
/// Board's right most x axis length.
pub const X_MAX: i32 =  8_i32;
/// Board's left most x acis length.
pub const X_MIN: i32 = -8_i32;
/// Board's top most y axis length.
pub const Y_MAX: i32 =  8_i32;
/// Board's down most y axis length.
pub const Y_MIN: i32 = -2_i32;

// Camera view over the board.
/// Board's right most length in view.
pub const RGT: i32 =  12_i32;
/// Board's left most length in view.
pub const LFT: i32 = -13_i32;
/// Board's top most length in view.
pub const TOP: i32 =  10_i32;
/// Board's bottom most length in view.
pub const BTM: i32 =  -4_i32;

/*████Local Constants████*/
/*-----------------------------------------------------------------------------------------------*/
const XMINF: f32 = X_MIN as f32;
const XMAXF: f32 = X_MAX as f32;
const YMINF: f32 = Y_MIN as f32;
const YMAXF: f32 = Y_MAX as f32;
const EMPTY: f32 = 6.0;
const BOARD: f32 = EMPTY - 1.0;
/*-----------------------------------------------------------------------------------------------*/

/// Quadrants inside the game.
///
/// ## Contents:
/// -   Block 1
/// -   Block 2
/// -   Block 3
#[derive(Debug)]
pub enum Quadrant {
    /// Block 1.
    Q1,
    /// Block 2.
    Q2,
    /// Block 3.
    Q3,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Quadrant████*/
/*-----------------------------------------------------------------------------------------------*/
impl Quadrant {
    pub fn from_index(index: usize) -> Result<Self, Error> {

        match index {
            0 => Ok(Quadrant::Q1),
            1 => Ok(Quadrant::Q2),
            2 => Ok(Quadrant::Q3),
            _ => Err(Error::InvalidQuadrantIndex(index)),
        }

    }

    pub fn from_xy(x: f32, y: f32) -> Result<Self, Error> {

        if position_in_q1_bounds(x, y) { return Ok(Quadrant::Q1) }
        if position_in_q2_bounds(x, y) { return Ok(Quadrant::Q2) }
        if position_in_q3_bounds(x, y) { return Ok(Quadrant::Q3) }

        Err(Error::PositionNotInQuadrant(x as i32, y as i32))

    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Board Bounds Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// Returns the bool if the x and y values are inside bounds.
pub fn position_in_q1_bounds(x: f32, y: f32) -> bool {
        (x < XMAXF - (BOARD * 2.0) && x >= XMINF)
    &&  (y <= (YMAXF - 1.0) - EMPTY && y >= YMINF)
}

pub fn position_in_q2_bounds(x: f32, y: f32) -> bool {
        (x <= (XMAXF - 1.0) - EMPTY && x >= XMAXF - (BOARD * 2.0))
    &&  (y < YMAXF && y >= (YMINF + 4.0))
}

pub fn position_in_q3_bounds(x: f32, y: f32) -> bool {
        (x >= XMAXF - EMPTY && x < XMAXF)
    &&  (y <= (YMAXF - 1.0) - EMPTY && y >= YMINF)
}

pub fn position_in_board_bounds(x: f32, y: f32) -> bool {
        position_in_q1_bounds(x, y)
    ||  position_in_q2_bounds(x, y)
    ||  position_in_q3_bounds(x, y)
}

/*████Cursor Position Logic████*/
/*-----------------------------------------------------------------------------------------------*/
fn full_width() -> f32 { (LFT.abs() + RGT) as f32 }

fn full_height() -> f32 { (BTM.abs() + TOP) as f32 }

pub fn cursor_in_window(c_x: f32, c_y: f32, height: f32, width: f32) -> (f32, f32) {
    (
        (((c_x / width) * full_width()) + (LFT as f32)).round(),
        (((c_y / height) * full_height()) + (BTM as f32)).round(),
    )
}
/*-----------------------------------------------------------------------------------------------*/
