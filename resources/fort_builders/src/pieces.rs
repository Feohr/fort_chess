//! # pieces module
//!
//! Holds the data and functions corresponding to the chess pieces.

mod piece_alignment;

// use crate::Error;
use crate::{
    RED, RST,
    board::{
        Quadrant, X_MAX, X_MIN, Y_MAX, Y_MIN
    },
};
use piece_alignment::{get_pos_from_quadrant, get_piece_type};
use thiserror::Error;

use std::fmt;

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
    #[error(
        "{} The vector id non-singular with length {0}. {}",
        RED,
        RST
    )]
    VectorNonSingular(usize),

}

/// To determine the piece type.
///
/// All the possible chess pieces to be used in game.
/// Does not contain king as this variation doesn't have it.
///
/// ## Contents:
/// -   Rook
/// -   Minister
/// -   Queen
/// -   Pawn
/// -   Knight
#[derive(Copy, Clone)]
pub enum PieceType {
    Rook,     // 0
    Minister, // 1
    Queen,    // 2
    Pawn,     // 3
    Knight,   // 4
}

impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<8}", self.as_str())
    }
}

impl<'a> PieceType {
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
    fn from_index(index: u8) -> Result<PieceType, Error> {
        match index {
            0 => Ok(PieceType::Rook),
            1 => Ok(PieceType::Minister),
            2 => Ok(PieceType::Queen),
            3 => Ok(PieceType::Pawn),
            4 => Ok(PieceType::Knight),
            _ => Err(Error::InvalidPieceTypeIndex(index)),
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            PieceType::Rook     => 0,
            PieceType::Minister => 1,
            PieceType::Queen    => 2,
            PieceType::Pawn     => 3,
            PieceType::Knight   => 4,
        }
    }
}

/// To get the piece position.
///
/// The x value corresponds to the x axis.
/// Similarly, the y value corresponds to the y axis.
///
/// ## Contents:
/// -   x:  the x-axis value.
/// -   y:  the y-axis value.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Position {
    /// The x-axis value.
    pub x: i32,

    /// The y-axis value.
    pub y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:2}, {:2})", self.x, self.y)
    }
}

impl Position {
    /// To create a position struct.
    ///
    /// Takes the x and y value, checks if it is inside the board and creates the struct.
    #[inline]
    pub fn from(x: i32, y: i32) -> Result<Self, Error> {
        Piece::in_board_range(x, y)?;
        // Finally.
        Ok(Position {
            x,
            y,
        })
    }
}

/// Piece struct that holds the type and the position of each piece.
///
/// ## Contents:
/// -   piece_type: holds the type of the piece via __PieceType__ enum.
/// -   position:   holds the position of the piece via __Position__ struct.
#[derive(Copy, Clone)]
pub struct Piece {
    /// To hold the type information for the piece.
    pub piece_type: PieceType,

    /// To hold the position information.
    pub position: Position,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} {:?}]", self.piece_type, self.position)
    }
}

impl Piece {
    /// To create a new piece type.
    ///
    /// Takes the positional arguments along with type argument to return a new piece type.
    /// x and y correspond to the x and y coordinates. The piece argument corresponds to the
    /// type.
    fn from(x: i32, y: i32, piece: PieceType) -> Result<Piece, Error> {
        Ok(Piece {
            piece_type: piece,
            position: Position::from(x, y)?,
        })
    }

    /// Function used to initialize the pieces vector.
    pub(crate) fn init_pieces(is_defender: bool, quadrant: Quadrant) -> Result<Vec<Piece>, Error> {
        Ok(get_pos_from_quadrant(is_defender, &quadrant)
            .into_iter()
            .zip(get_piece_type(is_defender))
            .flat_map(|(position, piece_type)| -> Result<Piece, Error> {
                let piece = Piece::from(
                                position.0,
                                position.1,
                                PieceType::from_index(piece_type)?,
                            )?;
                Ok(piece)
            })
            .collect::<Vec<Piece>>())
    }

    /// To update the Position of the piece.
    ///
    /// Takes x and y value and chan    ges the position to the given value.
    /// Returns error if the new position is out of range.
    pub(crate) fn update_pos(&mut self, x: i32, y: i32) -> Result<(), Error> {
        Self::in_board_range(x, y)?;
        Ok({
            self.position.x = x;
            self.position.y = y;
        })
    }

    /// To check wether a vector index is valid.
    ///
    /// There can be a maximum of 24 pieces inside a player pieces vec. Anything more than that is an
    /// error.
    pub(crate) fn is_valid_index(pos: usize, is_defender: bool) -> Result<(), Error> {
        match match is_defender {
            true  => pos < 24,
            false => pos < 8,
        } {
            true  => Ok(()),
            false => Err(Error::IllegalPieceVectorIndex(pos)),
        }
    }

    /// To check if the position is inside the piece vector bounds.
    ///
    /// takes a usize value and checks the vector size with the length of the pieces vec.
    pub(crate) fn is_in_bounds(pos: usize, len: usize) -> Result<(), Error> {
        match pos < len {
            true  => Ok(()),
            false => Err(Error::PieceVectorIndexOutOfBounds(pos, len)),
        }
    }

    /// To check if a position value is inside the board.
    ///
    /// This function is used to check if a particular position is inside the board.
    pub(crate) fn in_board_range(x: i32, y: i32) -> Result<(), Error> {
        match (x < X_MAX && x >= X_MIN) && (y < Y_MAX && y >= Y_MIN) {
            true  => Ok(()),
            false => return Err(Error::IllegalPosition(x, y)),
        }
    }
}

pub(crate) trait TryIsSingular {
    type Item;
    type Error;

    fn try_is_singular(self) -> Result<Vec<Self::Item>, Self::Error>;
}

impl TryIsSingular for std::vec::Vec<Piece> {
    type Item = Piece;
    type Error = Error;

    fn try_is_singular(self) -> Result<Vec<Self::Item>, Self::Error> {
        match self.len() < 2 {
            true  => Ok(self),
            false => Err(Error::VectorNonSingular(self.len())),
        }
    }
}
