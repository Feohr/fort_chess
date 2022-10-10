//! # pieces module
//!
//! Holds the data and functions corresponding to the chess pieces.

use crate::Error;
use crate::board::{ X_MAX, Y_MAX, X_MIN, Y_MIN };

/// To determine the piece type.
/// 
/// All the possible chess pieces to be used in game.
/// Does not contain king as this variation doesn't have it.
pub enum PieceType {
    Rook,
    Minister,
    Queen,
    Pawn,
    Knight,
}

/// To get the piece position.
///
/// The x value corresponds to the x axis.
/// Similarly, the y value corresponds to the y axis.
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// To check if a position value is inside the board.
    ///
    /// This function is used to check if a particular position is inside the board.
    /// Wherever this function returns false, the program should be stopped with an error
    /// immediately.
    /// this function takes x and y positional values and checks if the position is within the 
    /// board.
    ///
    /// ```
    /// # fn main() {
    ///     if !Position::in_range(255, 255) {
    ///         panic!("Position outside the board")
    ///     }
    /// #}
    /// ```
    /// This function should be limited to used to check wether a particular position within a
    /// context is inside a board or not.
    ///
    /// **Do not use this function to check for custom values**
    #[inline]
    pub fn in_range(x: i32, y: i32) -> bool {
        (( x <= X_MAX && x >= X_MIN ) 
        && ( y <= Y_MAX && y >= Y_MIN ))
        || x != 0 || y != 0
    }

    /// To create a position struct.
    ///
    /// Takes the x and y value, checks if it is inside the board and creates the struct.
    ///
    /// ```
    /// # fn main() {
    ///         let _pos = Position::from(12, 23).unwrap();
    /// # }
    /// ```
    #[inline]
    pub fn from(x: i32, y: i32) -> Result<Self, Error> {
        if !Self::in_range(x, y) {
            return Err(Error::IllegalPosition(x, y));
        }
        Ok(Position {
            x,
            y,
        })
    }
}

/// Piece struct that holds the type and the position of each piece.
///
/// Contents:
///     piece_type: holds the type of the piece via __PieceType__ enum.
///     position:   holds the position of the piece via __Position__ struct.
pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
}

impl Piece {
    /// To create a new piece type.
    ///
    /// Takes the positional arguments along with type argument to return a new piece type.
    /// x and y correspond to the x and y coordinates. The piece argument corresponds to the
    /// type.
    ///
    /// ```
    /// # fn main() {
    ///         let _piece = Piece::from(8, 2, PieceType::Rook)i.unwrap();
    /// # }
    /// ```
    pub fn from(x: i32, y: i32, piece: PieceType) -> Result<Piece, Error> {
        let pos = Position::from(x, y)?;
        Ok(Piece {
            piece_type: piece,
            position: pos, 
        })
    }

    /// To update the Position of the piece.
    ///
    /// Takes x and y value and changes the position to the given value.
    /// Returns error if the new position is out of range.
    ///
    /// ```
    /// # fn main() {
    /// #       let piece = Piece::from(8, 2, PieceType::Rook).unwrap();
    ///         piece.update_pos(4, 3).unwrap();
    /// #}
    /// ```
    pub fn update_pos(&mut self, x: i32, y: i32) -> Result<(), Error> {
        if !Position::in_range(x, y) {
            return Err(Error::IllegalPosition(x, y));
        } 
        Ok({
            self.position.x = x;
            self.position.y = y;
        })
    }
}
