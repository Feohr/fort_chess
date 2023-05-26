//! # pieces module
//!
//! Holds the data and functions corresponding to the chess pieces.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod piece_alignment;

use crate::{
    board::{Quadrant, X_MAX, X_MIN, Y_MAX, Y_MIN},
    RED, RST,
};
use piece_alignment::{piece_type, position_from_quadrant};
use thiserror::Error;
use std::fmt;

/// To hold the number of maximum enemies at a given point for `defender`.
const DEFND_COUNT   : usize = 24_usize;
/// To hold the number of maximum enemies at a given point for `non-defender`.
const ENEMY_COUNT   : usize =  8_usize;

/// Piece error enum.
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid position.
    #[error("{} The position ({0}, {1}) is invalid {}", RED, RST)]
    IllegalPosition(i32, i32),
    /// If the position referenced is not present in the pieces vector.
    #[error(
        "{} The given index of the piece {0} does not exist in a vec of length {1}. {}",
        RED,
        RST
    )]
    PieceVectorIndexOutOfBounds(usize, usize),
    /// When an illegal position is referenced.
    #[error(
        "{} The given index {0} cannot exist as the index for a piece vector should be \
    (0 < length < 24 | 8). {}",
        RED,
        RST
    )]
    IllegalPieceVectorIndex(usize),
    /// If Invalid Piece type index was provided.
    #[error(
        "{} The provided index {0} does not have a piece type corresponding to it. {}",
        RED,
        RST
    )]
    InvalidPieceTypeIndex(u8),
    /// If Invalid Piece type index was provided.
    #[error("{} The vector id non-singular with length {0}. {}", RED, RST)]
    VectorNonSingular(usize),
}

/// To determine the [`Piece`] type.
///
/// All the possible chess pieces to be used in game.
/// Does not contain king as this variation doesn't have it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PieceType {
    Rook,     // 0
    Minister, // 1
    Queen,    // 2
    Pawn,     // 3
    Knight,   // 4
}

/// To get the piece position.
///
/// The x value corresponds to the x axis.
/// Similarly, the y value corresponds to the y axis.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    /// The x-axis value.
    pub x: i32,
    /// The y-axis value.
    pub y: i32,
}

/// Piece struct that holds the type and the position of each piece.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Piece {
    /// To hold the type information for the piece.
    pub piece_type: PieceType,
    /// To hold the position information.
    pub position: Position,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████PieceType████*/
/*-----------------------------------------------------------------------------------------------*/
#[doc(hidden)]
impl fmt::Debug for PieceType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<8}", self.as_str())
    }
}

impl<'a> PieceType {
    /// Takes self reference and returns the corresponding [`PieceType`] value as a `&str`.
    #[inline]
    fn as_str(&self) -> &'a str {
        match self {
            PieceType::Rook     => "Rook",
            PieceType::Minister => "Minister",
            PieceType::Queen    => "Queen",
            PieceType::Pawn     => "Pawn",
            PieceType::Knight   => "Knight",
        }
    }
}

impl PieceType {
    /// Returns a [`Result`] type with [`PieceType`] by taking in a `u8` value to correspond with
    /// it.
    ///
    /// [`Result`]: std::result::Result
    #[inline]
    fn from_index(index: u8) -> Result<PieceType, Error> {
        match index {
            0_u8 => Ok(PieceType::Rook),
            1_u8 => Ok(PieceType::Minister),
            2_u8 => Ok(PieceType::Queen),
            3_u8 => Ok(PieceType::Pawn),
            4_u8 => Ok(PieceType::Knight),
            _ => Err(Error::InvalidPieceTypeIndex(index)),
        }
    }
    /// Takes a self reference and returns a `usize` value that corresponds to the type.
    #[inline]
    pub fn as_usize(&self) -> usize {
        match self {
            PieceType::Rook     => 0_usize,
            PieceType::Minister => 1_usize,
            PieceType::Queen    => 2_usize,
            PieceType::Pawn     => 3_usize,
            PieceType::Knight   => 4_usize,
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Position████*/
/*-----------------------------------------------------------------------------------------------*/
#[doc(hidden)]
impl fmt::Debug for Position {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:2}, {:2})", self.x, self.y)
    }
}

impl Position {
    /// To create a [`Position`] struct.
    ///
    /// Takes the x and y value, checks if it is inside the board and creates the struct.
    #[inline]
    pub(crate) fn from(x: i32, y: i32) -> Result<Self, Error> {
        Piece::in_board_range(x, y)?;
        Ok(Position { x, y })
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Piece████*/
/*-----------------------------------------------------------------------------------------------*/
#[doc(hidden)]
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} {:?}]", self.piece_type, self.position)
    }
}

impl Piece {
    /// To create a new [`Piece`] type.
    ///
    /// Takes the positional arguments along with type argument to return a new piece type.
    /// x and y correspond to the x and y coordinates. The piece argument corresponds to the type.
    #[inline]
    pub fn from(x: i32, y: i32, piece: PieceType) -> Result<Piece, Error> {
        Ok(Piece {
            piece_type: piece,
            position: Position::from(x, y)?,
        })
    }
    /// Function used to initialize the [`Piece`] vector.
    pub(crate) fn init_pieces(
        is_defender:        bool,
        quadrant:           Quadrant,
        quadrant_active:    usize,
    ) -> Result<Vec<Piece>, Error> {
        let mut pieces: Vec<Piece> = Vec::new();
        for ((pos1, pos2), piece_type) in   position_from_quadrant(
                                                &quadrant,
                                                quadrant_active,
                                            )
                                            .into_iter()
                                            .zip(piece_type(is_defender, quadrant_active))
        {
            pieces.push(
                Piece::from(
                    pos1, pos2, PieceType::from_index(piece_type)?,
                )?
            ); 
        }
        Ok(pieces)
    }
    /// To update the [`Position`] of the [`Piece`].
    ///
    /// Takes x and y value and changes the position to the given value.
    /// Returns error if the new position is out of range.
    #[inline]
    pub(crate) fn update_pos(&mut self, x: i32, y: i32) -> Result<(), Error> {
        Self::in_board_range(x, y)?;
        Ok({
            self.position.x = x;
            self.position.y = y;
        })
    }
    /// To check wether a vector index is valid.
    ///
    /// There can be a maximum of 24 [`Piece`] inside a player pieces vec. Anything more than that
    /// is an error.
    #[inline]
    pub(crate) fn is_valid_index(pos: usize, is_defender: bool) -> Result<(), Error> {
        match match is_defender {
            true  => pos < DEFND_COUNT,
            false => pos < ENEMY_COUNT,
        } {
            true  => Ok(()),
            false => Err(Error::IllegalPieceVectorIndex(pos)),
        }
    }
    /// To check if the [`Position`] is inside the [`Piece`] vector bounds.
    ///
    /// takes a `usize` value and checks the vector size with the length of the pieces `vec`.
    #[inline]
    pub(crate) fn is_in_bounds(pos: usize, len: usize) -> Result<(), Error> {
        match pos < len {
            true => Ok(()),
            false => Err(Error::PieceVectorIndexOutOfBounds(pos, len)),
        }
    }
    /// To check if a [`Position`] value is inside the board.
    ///
    /// This function is used to check if a particular [`Position`] is inside the board.
    #[inline]
    pub(crate) fn in_board_range(x: i32, y: i32) -> Result<(), Error> {
        match (x <= X_MAX && x >= X_MIN) && (y <= Y_MAX && y >= Y_MIN) {
            true => Ok(()),
            false => return Err(Error::IllegalPosition(x, y)),
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/
