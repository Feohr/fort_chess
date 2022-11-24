//! # player module
//!
//! Contains the player objects and corresponding operations.
//! Used to create, kill, set values and to update position of the player pieces.
//! Also contains the team object to handle the player teams.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

// use crate::Error;
use crate::board::Quadrant;
use crate::pieces;
use crate::pieces::{Piece, Position};
use crate::{RED, RST};
use thiserror::Error;

/// Player Error enum.
#[derive(Error, Debug)]
pub enum Error {
    /// If the name is too long or too short.
    #[error(
        "{} The name is either too long or too short. Ideal length is (2 < name < 15). \
        Your name length: {0} {}",
        RED,
        RST
    )]
    InvalidNameLength(String, usize),

    /// If the position referenced is not present in the pieces vector.
    #[error(
        "{} The given index of the piece {0} does not exist in a vec of length {1}. {}",
        RED,
        RST
    )]
    PlayerVectorIndexOutOfBounds(usize, usize),

    /// When an illegal position is referenced.
    #[error(
        "{} The given index {0} cannot exist as the index for a player vector should be \
    (0 < length < 4). {}",
        RED,
        RST
    )]
    IllegalPlayerVectorIndex(usize),

    /// If Invalid Team index was provided.
    #[error(
        "{} The provided index {0} does not have a team corresponding to it. {}",
        RED,
        RST
    )]
    InvalidTeamIndex(usize),

    /// Error from pieces module.
    #[error("{} Error in piece module originated from player module. {}", RED, RST)]
    PlayerPieceModuleError(#[from] pieces::Error),
}

/// Contains the types of teams.
///
/// Used to distinguish players from team to team.
/// Each player must have a unique team.
/// Maximum of only four players can play at a time.
#[derive(Copy, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum Team {
    Red,    // 0
    Blue,   // 1
    Green,  // 2
    Yellow, // 3
}

/// [`Player`] struct used to handle player specific information.
///
/// Contains data such as name, pieces held, team, etc.
#[derive(Debug)]
pub struct Player {
    /// The name of the player.
    pub name: String,

    /// The pieces held by the player.
    pub pieces: Vec<Piece>,

    /// The team player belongs to.
    pub team: Team,

    /// Indicates if the player is a defender.
    pub is_defender: bool,

    /// Indicates if the player is a winner.
    pub is_winner: bool,

    /// Current position of the player piece in the vec.
    pub chosen_piece_index: usize,
}

/// A public trait to handle [`Player`] actions.
pub trait PlayerAction {

    fn piece_index_from_xy_i32(&self, x: i32, y: i32) -> Result<usize, usize>;

    fn piece_index_from_xy_f32(&self, x: f32, y: f32) -> Result<usize, usize>;

    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error>;

    fn update_piece(&mut self, x: i32, y: i32) -> Result<bool, Error>;

}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Team████*/
/*-----------------------------------------------------------------------------------------------*/
impl Team {

    /// To get a team corresponding to the index value.
    ///
    /// Takes a usize and returns a [`Team`] struct.
    #[inline(always)]
    pub fn from_index(index: usize) -> Result<Self, Error> {

        match index {
            0 => Ok(Team::Red),
            1 => Ok(Team::Blue),
            2 => Ok(Team::Green),
            3 => Ok(Team::Yellow),
            _ => Err(Error::InvalidTeamIndex(index)),
        }

    }

    /// To turn a team enum value to a String value.
    ///
    /// Takes [`Team`] enum value and converts is to [`String`] value.
    #[inline(always)]
    pub fn teamstr_from_team<'a>(team: Team) -> &'a str {

        match team {
            Team::Red    => "Red",
            Team::Green  => "Green",
            Team::Blue   => "Blue",
            Team::Yellow => "Yellow",
        }

    }

    /// Takes the [`Team`] value and returns the respective index.
    pub fn as_usize(&self) -> usize {

        match self {
            Team::Red    => 0_usize,
            Team::Blue   => 1_usize,
            Team::Green  => 2_usize,
            Team::Yellow => 3_usize,
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Player████*/
/*-----------------------------------------------------------------------------------------------*/
impl Player {

    /// Used to initialize a new player based on the inputs.
    ///
    /// Takes the name, team and is_defender boolean value to create a [`Player`] struct.
    #[inline(always)]
    pub fn from(
        name:               String,
        team:               Team,
        is_defender:        bool,
        quadrant_active:    usize,
        quadrant:           Quadrant,
    ) -> Result<Self, Error> {

        Ok(Player {
            name:   Player::validate_name(name)?,
            pieces: Piece::init_pieces(is_defender, quadrant, quadrant_active)?,
            team,
            is_defender,
            is_winner: false,
            chosen_piece_index: 0_usize,
        }
        .to_sorted())

    }

    /// To set the player as a winner.
    ///
    /// Changes the `is_winner` value to `true`.
    ///
    /// `Idempotent function`
    pub fn set_winner(&mut self) { self.is_winner = true }

    /// For sorting the pieces.
    ///
    /// TimeSorts the pieces based on their position value.
    fn sort_pieces(&mut self) {
        self.pieces.sort_by(|a, b| a.position.cmp(&b.position))
    }

    /// To sort and return the object.
    ///
    /// Used when intializing the player struct.
    fn to_sorted(mut self) -> Self {
        self.sort_pieces();
        self
    }

    /// To get the piece reference in the current player.
    #[inline]
    pub fn pieces(&self) -> &Vec<Piece> { &self.pieces }

    /// To get the pieces mutable reference in the current player.
    #[inline]
    pub fn pieces_mut(&mut self) -> &mut Vec<Piece> { &mut self.pieces }

    /// A simple function to check if the name length is too big or too small.
    ///
    /// returns false if the name length is invalid. The constraints are 2 < name_length < 15.
    #[inline(always)]
    fn validate_name(name: String) -> Result<String, Error> {

        match (3_usize..15_usize).contains(&name.len()) {
            true  => Ok(name),
            false => Err(Error::InvalidNameLength(name.clone(), name.len())),
        }

    }

    /// To set the chosen piece position in the vec.
    #[inline]
    pub fn set_chosen_piece_index(&mut self, chosen_piece_index: usize) {
        self.chosen_piece_index = chosen_piece_index
    }

    /// To get the chosen piece reference.
    #[inline]
    pub fn current_chosen_piece(&self) -> &Piece {
        &self.pieces[self.chosen_piece_index]
    }

    /// To get the chosen piece mutable reference.
    #[inline]
    pub fn current_chosen_piece_mut(&mut self) -> &mut Piece {
        &mut self.pieces[self.chosen_piece_index]
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerAction for Player████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerAction for Player {

    /// To get position of the piece that is clicked in relevance to the vec.
    ///
    /// Takes x and y coordinate position, as `f32`, of the clicked piece. Then checks the piece
    /// which is being referenced and returns the position of the piece inside the [`pieces`]
    /// vector. This can be used to update the piece position. Returns null if not piece exist at
    /// that location.
    ///
    /// [`pieces`]: crate::pieces
    #[inline]
    fn piece_index_from_xy_f32(&self, x: f32, y: f32) -> Result<usize, usize> {
        self.piece_index_from_xy_i32(x as i32, y as i32)
    }

    /// To get position of the piece that is clicked in relevance to the vec.
    ///
    /// Takes x and y coordinate position, as `i32`, of the clicked piece. Then checks the piece
    /// which is being referenced and returns the position of the piece inside the [`pieces`]
    /// vector. This can be used to update the piece position. Returns null if not piece exist at
    /// that location.
    ///
    /// [`pieces`]: crate::pieces
    #[inline]
    fn piece_index_from_xy_i32(&self, x: i32, y: i32) -> Result<usize, usize> {

        self.pieces.binary_search_by(|piece| {
            piece.position.cmp(&Position {
                x,
                y,
            })
        })

    }

    /// To kill a piece inside the [`Player`] struct.
    ///
    /// Takes the position of the piece, finds it inside the vector and deletes that piece.
    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error> {

        Piece::is_valid_index(pos, self.is_defender)?;
        Piece::is_in_bounds(pos, self.pieces.len())?;

        let piece = self.pieces.remove(pos);

        self.sort_pieces();

        Ok(piece)

    }

    /// To update position of the piece inside the [`Player`] struct vector.
    ///
    /// Takes x and y coordinates to update the piece at the position that is provided.
    /// returns a result type in case of errors.
    fn update_piece(&mut self, x: i32, y: i32) -> Result<bool, Error> {

        let pos = self.chosen_piece_index;

        // Is the index a valid index?
        Piece::is_valid_index(pos, self.is_defender)?;

        match self.pieces[pos].position.x == x && self.pieces[pos].position.y == y {
            true => Ok(false),
            false => {
                self.pieces.get_mut(pos)
                           .expect("cannot update piece as the position is not valid: {pos}.")
                           .update_pos(x, y)?;
                self.sort_pieces();
                Ok(true)
            },
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/
