//! # board module
//!
//! module to hold board specific values like dimensions etc.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::Error;

// Board tile borders.
/// Board's right most `x axis` length.
pub const X_MAX: i32 =  8_i32;
/// Board's left most `x axis` length.
pub const X_MIN: i32 = -8_i32;
/// Board's top most `y axis` length.
pub const Y_MAX: i32 =  8_i32;
/// Board's down most `y axis` length.
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
/// Holds the [`X_MIN`]  value as `f32`.
const XMINF: f32 = X_MIN as f32;
/// Holds the [`X_MAX`]  value as `f32`.
const XMAXF: f32 = X_MAX as f32;
/// Holds the [`Y_MIN`]  value as `f32`.
const YMINF: f32 = Y_MIN as f32;
/// Holds the[`Y_MAX`]  value as `f32`.
const YMAXF: f32 = Y_MAX as f32;
/// Holds the empty value that needs to be deleted from each size to form the board shape.
const EMPTY: f32 = 6.0;
/// To get the actual board bounds excluding the border.
const BOARD: f32 = EMPTY - 1.0;
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
pub fn q1_outer_bound_pos() -> (i32, i32) { (X_MIN - 4, 0) }

pub fn q2_outer_bound_pos() -> (i32, i32) { (-1, Y_MAX + 1) }

pub fn q3_outer_bound_pos() -> (i32, i32) { (X_MAX + 1, 0) }
/*-----------------------------------------------------------------------------------------------*/

/*████Quadrant████*/
/*-----------------------------------------------------------------------------------------------*/
impl Quadrant {

    /// To get a [`Quadrant`] value from index of usize.
    pub fn from_index(index: usize) -> Result<Self, Error> {

        match index {
            0 => Ok(Quadrant::Q1),
            1 => Ok(Quadrant::Q2),
            2 => Ok(Quadrant::Q3),
            _ => Err(Error::InvalidQuadrantIndex(index)),
        }

    }

    /// To get a [`Quadrant`] value from x and y values of `f32` type.
    pub fn from_xy(x: f32, y: f32) -> Result<Self, Error> {

        if position_in_q1_bounds(x, y) { return Ok(Quadrant::Q1) }
        if position_in_q2_bounds(x, y) { return Ok(Quadrant::Q2) }
        if position_in_q3_bounds(x, y) { return Ok(Quadrant::Q3) }

        // If the position is out of bounds.
        Err(Error::PositionNotInQuadrant(x as i32, y as i32))

    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Board Bounds Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// Returns the bool if the x and y values are inside [`Q1`] bounds.
///
/// [`Q1`]: Quadrant::Q1
pub fn position_in_q1_bounds(x: f32, y: f32) -> bool {
        (x < XMAXF - (BOARD * 2.0) && x >= XMINF)
    &&  (y <= (YMAXF - 1.0) - EMPTY && y >= YMINF)
}

/// Returns the bool if the x and y values are inside [`Q2`] bounds.
///
/// [`Q2`]: Quadrant::Q2
pub fn position_in_q2_bounds(x: f32, y: f32) -> bool {
        (x <= (XMAXF - 1.0) - EMPTY && x >= XMAXF - (BOARD * 2.0))
    &&  (y < YMAXF && y >= (YMINF + 4.0))
}

/// Returns the bool if the x and y values are inside [`Q3`] bounds.
///
/// [`Q3`]: Quadrant::Q3
pub fn position_in_q3_bounds(x: f32, y: f32) -> bool {
        (x >= XMAXF - EMPTY && x < XMAXF)
    &&  (y <= (YMAXF - 1.0) - EMPTY && y >= YMINF)
}

/// Checks if position inside [`Q1`], [`Q2`] or [`Q3`]. Returns false
/// if the piece not inside these three bounds.
///
/// [`Q1`]: Quadrant::Q1
/// [`Q2`]: Quadrant::Q2
/// [`Q3`]: Quadrant::Q3
pub fn position_in_board_bounds(x: f32, y: f32) -> bool {
        position_in_q1_bounds(x, y)
    ||  position_in_q2_bounds(x, y)
    ||  position_in_q3_bounds(x, y)
}

/*████Cursor Position Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// To get the screen width as `f32`.
fn full_width() -> f32 { (LFT.abs() + RGT) as f32 }

/// To get the screen height as `f32`.
fn full_height() -> f32 { (BTM.abs() + TOP) as f32 }

/// To get the cursor position relative to the camera screen.
pub fn cursor_in_window(c_x: f32, c_y: f32, height: f32, width: f32) -> (f32, f32) {
    (
        (((c_x / width) * full_width()) + (LFT as f32)).round(),
        (((c_y / height) * full_height()) + (BTM as f32)).round(),
    )
}
/*-----------------------------------------------------------------------------------------------*/
