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

// Board tile borders.
/// Board's right most x axis length.
pub const X_MAX: i32 = 8_i32;
/// Board's left most x acis length.
pub const X_MIN: i32 = -8_i32;
/// Board's top most y axis length.
pub const Y_MAX: i32 = 8_i32;
/// Board's down most y axis length.
pub const Y_MIN: i32 = -2_i32;

// Camera view over the board.
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
const PADDING: f32 = 0.5;
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

/*████Picker Logics████*/
/*-----------------------------------------------------------------------------------------------*/
fn yq1(y: f32, ymax: f32) -> bool { y <= ymax - EMPTY && y >= YMINF }

fn yq2(y: f32, ymax: f32) -> bool { y <= ymax && y >= (YMINF + 4.0) }

fn xq2(x: f32, xmax: f32) -> bool { x <= xmax - EMPTY }

fn xq12(x: f32, xmax: f32) -> bool { x >= xmax - (BOARD * 2.0) }

fn xq11(x: f32, xmax: f32) -> bool { x <= xmax - (BOARD * 2.0) }

fn xqabs(x: f32, xmax: f32) -> bool { x.abs() <= xmax && x.abs() >= xmax - BOARD }

fn xnyq2(x: f32, y: f32, xmax: f32, ymax: f32) -> bool { xq2(x, xmax) && yq2(y, ymax) }

fn pickerlogic(a: bool, b: bool, c: bool) -> bool {
        ( a &&  b &&  c)
    ||  ( a &&  b && !c)
    ||  (!a &&  b &&  c)
    ||  (!a && !b &&  c)
}

pub fn in_pos(m_x: f32, m_y: f32, plen: usize) -> bool {
    let (xmax, ymax) = get_max_without_zero(m_x > 0.0, m_y > 0.0);
    if !(
            (m_x >= XMINF - PADDING && m_x <= XMAXF - PADDING       )
        &&  (m_y >= YMINF - PADDING && m_y <= YMAXF - PADDING       )
    )   ||  (m_x.abs() <= xmax - EMPTY && m_y.abs() <= ymax - EMPTY ) { return false }
    match plen {
        2 => xq11(m_x, xmax) && yq1(m_y, ymax),
        3 => pickerlogic(xq12(m_x, xmax), xnyq2(m_x, m_y, xmax, ymax), yq1(m_y, ymax)),
        4 => pickerlogic(xqabs(m_x, xmax), yq1(m_y, ymax), yq2(m_y, ymax)),
        _ => panic!("count of players cannot be less 2 or more than 4. LEN = {plen}"),
    }
}
/*-----------------------------------------------------------------------------------------------*/

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
}
/*-----------------------------------------------------------------------------------------------*/

pub fn get_full_width() -> f32 {
    (LFT.abs() + RGT) as f32
}

pub fn get_full_height() -> f32 {
    (BTM.abs() + TOP) as f32
}

fn get_max_without_zero(xbool: bool, ybool: bool) -> (f32, f32) {
    (
        match xbool {
            true  => XMAXF - 1.0,
            false => XMAXF,
        },
        match ybool {
            true  => YMAXF - 1.0,
            false => YMAXF,
        }
    )
}
