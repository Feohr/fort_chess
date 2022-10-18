//! # pieces module
//!
//! Holds the data and functions corresponding to the chess pieces.

mod piece_alignment;

use crate::Error;
use crate::board::{ X_MAX, Y_MAX, X_MIN, Y_MIN, Quadrant };
use std::fmt;
use piece_alignment::get_resp_info;

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
pub enum PieceType {
    Rook,       // 0
    Minister,   // 1
    Queen,      // 2
    Pawn,       // 3
    Knight,     // 4
}

impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> PieceType {
    fn as_str(&self) -> &'a str {
        match self {
            PieceType::Rook     =>  "Rook",
            PieceType::Minister =>  "Minister",
            PieceType::Queen    =>  "Queen",
            PieceType::Pawn     =>  "Pawn",
            PieceType::Knight   =>  "Knight",
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
            _ => Err(Error::InvalidPieceTypeIndex(index))
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
pub struct Position {
    /// The x-axis value.
    pub x: i32,

    /// The y-axis value.
    pub y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// To check if a position value is inside the board.
    ///
    /// This function is used to check if a particular position is inside the board.
    /// Wherever this function returns false, the program should be stopped with an error
    /// immediately.
    /// this function takes x and y positional values and checks if the position is within the 
    /// board.
    ///
    /// This function should be limited to used to check wether a particular position within a
    /// context is inside a board or not.
    ///
    /// **Do not use this function to check for custom values**
#[inline(always)]
pub fn in_board_range(x: i32, y: i32) -> bool {
        ( x < X_MAX && x >= X_MIN )
    &&  ( y < Y_MAX && y >= Y_MIN )
}

impl Position {
    /// To create a position struct.
    ///
    /// Takes the x and y value, checks if it is inside the board and creates the struct.
    #[inline]
    pub fn from(x: i32, y: i32) -> Result<Self, Error> {
        if !in_board_range(x, y) {
            return Err(Error::IllegalPosition(x, y));
        }
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
pub struct Piece {
    /// To hold the type information for the piece.
    pub piece_type: PieceType,

    /// To hold the position information.
    pub position: Position,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.piece_type, self.position)
    }
}

impl Piece {
    /// To create a new piece type.
    ///
    /// Takes the positional arguments along with type argument to return a new piece type.
    /// x and y correspond to the x and y coordinates. The piece argument corresponds to the
    /// type.
    fn from(x: i32, y: i32, piece: PieceType) -> Result<Piece, Error> {
        let pos = Position::from(x, y)?;
        Ok(Piece {
            piece_type: piece,
            position: pos, 
        })
    }

    pub fn init_pieces(is_defender: bool, quadrant: Quadrant) -> Result<Vec<Piece>, Error> {
        let (pos, tp) = get_resp_info(is_defender, quadrant);
        let piece_vec = pos
                        .into_iter()
                        .zip(tp)
                        .flat_map(
                            |(position, piece_type)| -> Result<Piece, Error>
                            {
                                let piece = Piece::from(
                                                position.0,
                                                position.1,
                                                PieceType::from_index(piece_type)?,
                                            )?;
                                Ok(piece)
                            }
                        )
                        .collect::<Vec<Piece>>();
        match piece_vec.is_empty() {
            true    =>  Err(Error::EmptyPieceVectorCreated),
            false   =>  Ok(piece_vec),
        }
    }

    /// To update the Position of the piece.
    ///
    /// Takes x and y value and chan    ges the position to the given value.
    /// Returns error if the new position is out of range.
    pub fn update_pos(&mut self, x: i32, y: i32) -> Result<(), Error> {
        if !in_board_range(x, y) {
            return Err(Error::IllegalPosition(x, y));
        } 
        Ok({
            self.position.x = x;
            self.position.y = y;
        })
    }
    /// To check wether a vector index is valid.
    ///
    /// There can be a maximum of 24 pieces inside a player pieces vec. Anything more than that is an
    /// error.
    pub fn is_valid_index(pos: usize, is_defender: bool) -> Result<(), Error> { 
        let check = match is_defender {
                        true    =>  pos < 24,
                        false   =>  pos < 8,
                    };
        match check {
            true    =>  Ok(()),
            false   =>  Err(Error::IllegalPieceVectorIndex(pos)),
        }
    }

    /// To check if the position is inside the piece vector bounds.
    ///
    /// takes a usize value and checks the vector size with the length of the pieces vec.
    pub fn is_in_bounds(pos: usize, len: usize) -> Result<(), Error> {
        match len < pos {
            true    =>  Err(Error::PieceVectorIndexOutOfBounds(pos, len)),
            false   =>  Ok(()),
        }
    }
}
