//! # pieces module
//!
//! Holds the data and functions corresponding to the chess pieces.

use crate::Error;
use crate::board::{ X_MAX, Y_MAX, X_MIN, Y_MIN, Quadrant };
use std::fmt;
use piece_alignment::{
    DEF_Q1, DEF_Q2, DEF_Q3, DEF_P_Q1, DEF_P_Q2, DEF_P_Q3,
    ENM_Q1, ENM_Q2, ENM_Q3, ENM_P,
};

type PosInfo    = [(i32, i32); 8];
type TypeInfo   = [u8; 8];
type PieceInfo  = (PosInfo, TypeInfo);

mod piece_alignment {
    // The defender.
    pub const DEF_Q1: [(i32, i32); 8] = [
        (-3,    -2), (-3,   -1), (-3,    0), (-3,     1),
        (-4,    -2), (-4,   -1), (-4,    0), (-4,     1),
    ];
    pub const DEF_Q2: [(i32, i32); 8] = [
        (-2,     2), (-1,    2), ( 0,    2), ( 1,     2),
        (-2,     3), (-1,    3), ( 0,    3), ( 1,     3),
    ];
    pub const DEF_Q3: [(i32, i32); 8] = [
        ( 2,     1), ( 2,    0), ( 2,   -1), ( 2,    -2),
        ( 3,     1), ( 3,    0), ( 3,   -1), ( 3,    -2),
    ];

    // The enemies.
    pub const ENM_Q1: [(i32, i32); 8] = [
        (-7,    -2), (-7,   -1), (-7,    0), (-7,     1),
        (-8,    -2), (-8,   -1), (-8,    0), (-8,     1),
    ];
    pub const ENM_Q2: [(i32, i32); 8] = [
        (-2,     6), (-1,    6), ( 0,    6), ( 1,     6),
        (-2,     7), (-1,    7), ( 0,    7), ( 1,     7),
    ];
    pub const ENM_Q3: [(i32, i32); 8] = [
        ( 6,     1), ( 6,    0), ( 6,   -1), ( 6,    -2),
        ( 7,     1), ( 7,    0), ( 7,   -1), ( 7,    -2),
    ];

    // piece type index
    pub const ENM_P     : [u8; 8] = [ 3, 3, 3, 3, 4, 3, 3, 4 ];
    pub const DEF_P_Q1  : [u8; 8] = [ 4, 1, 2, 0, 3, 3, 3, 3 ];
    pub const DEF_P_Q2  : [u8; 8] = [ 4, 2, 1, 4, 3, 3, 3, 3 ];
    pub const DEF_P_Q3  : [u8; 8] = [ 0, 1, 2, 4, 3, 3, 3, 3 ];
}

/// To determine the piece type.
///
/// All the possible chess pieces to be used in game.
/// Does not contain king as this variation doesn't have it.
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
    fn type_from_index(index: u8) -> Result<PieceType, Error> {
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
/// # Contents:
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

impl Position {
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
    #[inline]
    pub fn in_range(x: i32, y: i32) -> bool {
            ( x < X_MAX && x >= X_MIN )
        &&  ( y < Y_MAX && y >= Y_MIN )
    }

    /// To create a position struct.
    ///
    /// Takes the x and y value, checks if it is inside the board and creates the struct.
    #[inline]
    pub fn from(x: i32, y: i32) -> Result<Self, Error> {
        if !Self::in_range(x, y) {
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
/// # Contents:
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
        write!(f, "<{:?}, {:?}>", self.piece_type, self.position)
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

    fn get_pos_from_quadrant(is_defender: bool, quadrant: &Quadrant) -> PosInfo {
        if is_defender {
            match quadrant {
                Quadrant::Q1 => return DEF_Q1,
                Quadrant::Q2 => return DEF_Q2,
                Quadrant::Q3 => return DEF_Q3,
            }
        }
            match quadrant {
                Quadrant::Q1 => return ENM_Q1,
                Quadrant::Q2 => return ENM_Q2,
                Quadrant::Q3 => return ENM_Q3,
            }
    }

    fn get_type_from_quadrant(is_defender: bool, quadrant: &Quadrant) -> TypeInfo {
        if is_defender {
            match quadrant {
                Quadrant::Q1 => return DEF_P_Q1,
                Quadrant::Q2 => return DEF_P_Q2,
                Quadrant::Q3 => return DEF_P_Q3,
            }
        }
            return ENM_P;
    }

    fn get_resp_info(is_defender: bool, quadrant: Quadrant) -> PieceInfo {
        let pos     = Piece::get_pos_from_quadrant(is_defender, &quadrant);
        let tp      = Piece::get_type_from_quadrant(is_defender, &quadrant);
        return (pos, tp);
    }

    pub fn init_pieces(is_defender: bool, quadrant: Quadrant) -> Vec<Piece> {
        let (pos, tp) = Piece::get_resp_info(is_defender, quadrant);
        let mut piece_vec = Vec::new();
        for (position, piece_type) in pos.zip(tp).iter() {
            piece_vec.push(
                Piece {
                    piece_type: PieceType::type_from_index(*piece_type).unwrap(),
                    position: Position {
                        x: position.0,
                        y: position.1,
                    }
                }
            );
        }
            return piece_vec;
    }

    /// To update the Position of the piece.
    ///
    /// Takes x and y value and changes the position to the given value.
    /// Returns error if the new position is out of range.
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
