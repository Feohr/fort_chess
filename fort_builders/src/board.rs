//! # board module
//!
//! module to hold board specific values like dimensions etc.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{Error, BREADTH};

/// Board's right most `x axis` length.
pub const X_MAX: i32 = 8_i32;
/// Board's left most `x axis` length.
pub const X_MIN: i32 = -8_i32;
/// Board's top most `y axis` length.
pub const Y_MAX: i32 = 8_i32;
/// Board's down most `y axis` length.
pub const Y_MIN: i32 = -2_i32;

/// Board's right most length in view.
pub const RGT: i32 = 12_i32;
/// Board's left most length in view.
pub const LFT: i32 = -13_i32;
/// Board's top most length in view.
pub const TOP: i32 = 10_i32;
/// Board's bottom most length in view.
pub const BTM: i32 = -4_i32;

/*████Local Constants████*/
/*-----------------------------------------------------------------------------------------------*/
/// Holds the [`X_MIN`]  value as `f32`.
pub const XMINF: f32 = X_MIN as f32;
/// Holds the [`X_MAX`]  value as `f32`.
pub const XMAXF: f32 = X_MAX as f32;
/// Holds the [`Y_MIN`]  value as `f32`.
pub const YMINF: f32 = Y_MIN as f32;
/// Holds the[`Y_MAX`]  value as `f32`.
pub const YMAXF: f32 = Y_MAX as f32;
/// Holds the empty value that needs to be deleted from each size to form the board shape.
const EMPTY: f32 = 6_f32;
/// to get the actual board bounds excluding the border.
const BOARD: f32 = EMPTY - 1_f32;
/*-----------------------------------------------------------------------------------------------*/

/// Quadrants inside the game. Each value corresponds to a side of the board.
#[derive(Debug)]
pub enum Quadrant {
    /// Block 1.
    Q1,
    /// Block 2.
    Q2,
    /// Block 3.
    Q3,
    /// Defender Quadrant.
    NoQuad,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Quadrant Outer Bounds████*/
/*-----------------------------------------------------------------------------------------------*/
/// Gets the outer bound of the board q1.
#[inline]
pub fn q1_outer_bound_pos() -> (i32, i32) {
    (X_MIN - 4_i32, 0_i32)
}

/// Gets the outer bound of the board q2.
#[inline]
pub fn q2_outer_bound_pos() -> (i32, i32) {
    (-1_i32, Y_MAX + 1_i32)
}

/// Gets the outer bound of the board q3.
#[inline]
pub fn q3_outer_bound_pos() -> (i32, i32) {
    (X_MAX + 1_i32, 0_i32)
}
/*-----------------------------------------------------------------------------------------------*/

/*████Quadrant████*/
/*-----------------------------------------------------------------------------------------------*/
impl Quadrant {
    /// To get a [`Quadrant`] value from index of usize.
    #[inline]
    pub fn from_index(index: usize) -> Result<Self, Error> {
        match index {
            0_usize => Ok(Quadrant::Q1),
            1_usize => Ok(Quadrant::Q2),
            2_usize => Ok(Quadrant::Q3),
            _ => Err(Error::InvalidQuadrantIndex(index)),
        }
    }
    /// To get a [`Quadrant`] value from x and y values of `f32` type.
    #[inline]
    pub fn from_xy(x: f32, y: f32) -> Result<Self, Error> {
        if position_in_q1_bounds(x, y) {
            return Ok(Quadrant::Q1);
        }
        if position_in_q2_bounds(x, y) {
            return Ok(Quadrant::Q2);
        }
        if position_in_q3_bounds(x, y) {
            return Ok(Quadrant::Q3);
        }
        Err(Error::PositionNotInQuadrant(x as i32, y as i32))
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Board Bounds Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// Returns the bool if the x and y values are inside [`Q1`] bounds.
///
/// [`Q1`]: Quadrant::Q1
#[inline]
pub fn position_in_q1_bounds(x: f32, y: f32) -> bool {
    (x < XMAXF - (BOARD * 2_f32) && x >= XMINF) && (y <= (YMAXF - 1_f32) - EMPTY && y >= YMINF)
}

/// Returns the bool if the x and y values are inside [`Q2`] bounds.
///
/// [`Q2`]: Quadrant::Q2
#[inline]
pub fn position_in_q2_bounds(x: f32, y: f32) -> bool {
    (x <= (XMAXF - 1_f32) - EMPTY && x >= XMAXF - (BOARD * 2_f32))
        && (y < YMAXF && y >= (YMINF + 4_f32))
}

/// Returns the bool if the x and y values are inside [`Q3`] bounds.
///
/// [`Q3`]: Quadrant::Q3
#[inline]
pub fn position_in_q3_bounds(x: f32, y: f32) -> bool {
    (x >= XMAXF - EMPTY && x < XMAXF) && (y <= (YMAXF - 1_f32) - EMPTY && y >= YMINF)
}

/// Checks if position inside [`Q1`], [`Q2`] or [`Q3`]. Returns false
/// if the piece not inside these three bounds.
///
/// [`Q1`]: Quadrant::Q1
/// [`Q2`]: Quadrant::Q2
/// [`Q3`]: Quadrant::Q3
#[inline]
pub fn position_in_board_bounds(x: f32, y: f32) -> bool {
    position_in_q1_bounds(x, y) || position_in_q2_bounds(x, y) || position_in_q3_bounds(x, y)
}

/*████Cursor Position Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// To get the screen width as `f32`.
#[inline]
fn full_width() -> f32 {
    (LFT.abs() + RGT) as f32
}

/// To get the screen height as `f32`.
#[inline]
fn full_height() -> f32 {
    (BTM.abs() + TOP) as f32
}

/// To get the cursor position relative to the camera screen.
#[inline]
pub fn cursor_in_window(c_x: f32, c_y: f32, height: f32, width: f32) -> (f32, f32) {
    (
        (((c_x / width) * full_width()) + (LFT as f32)).round(),
        (((c_y / height) * full_height()) + (BTM as f32)).round(),
    )
}
/*-----------------------------------------------------------------------------------------------*/

/*████Position in Opposite Side Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// To check wether the defender piece is in the opposite side.
#[inline]
pub(crate) fn check_in_opposite_defender(x: i32, y: i32) -> bool {
    x <= X_MIN || x >= X_MAX - 1_i32 || y >= Y_MAX - 1_i32
}

/// To check wether the non-defender piece is in the opposite side.
#[inline]
pub(crate) fn check_in_opposite_enemy(x: i32, y: i32) -> bool {
    (x >= -BREADTH - 1_i32 && x < BREADTH + 1_i32) && y.abs() <= BREADTH
}
/*-----------------------------------------------------------------------------------------------*/
