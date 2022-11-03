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
        "{} The name is either too long or too short. Ideal length is (3 < name < 255). \
        Your name length: {0} {}",
        RED,
        RST
    )]
    InvalidNameLength(usize),

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
///
/// ## Contents:
/// -   Red
/// -   Blue
/// -   Green
/// -   Yellow
#[derive(Copy, Clone, Debug)]
pub enum Team {
    Red,    // 0
    Blue,   // 1
    Green,  // 2
    Yellow, // 3
}

/// __Player__ struct used to handle player specific information.
///
/// Contains data such as name, pieces held, team, etc.
///
/// ## Contents:
/// -   name:           the name of the player.
/// -   pieces:         pieces held by the player.
/// -   team:           player team.
/// -   is_defender:    if a player is defending.
/// -   is_winner:      if the player is winner.
/// -   is_play:        is the player still playing.
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

    /// Indicates if the player is still playing.
    pub is_play: bool,
}

/// A public trait to handle __Player__ actions during runtime.
///
/// ## Actions:
/// -   get_piece_pos:  To get piece position within the struct based on x, y coordinates.
/// -   kill_piece:     To remove a piece from the __pieces__ vector.
/// -   update_piece:   To update piece position.
pub(crate) trait PlayerAction {
    fn get_piece_pos(&self, x: i32, y: i32) -> Result<Option<usize>, Error>;

    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error>;

    fn update_piece(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error>;

    fn kill_self(&mut self);
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Team████*/
/*-----------------------------------------------------------------------------------------------*/
impl Team {
    /// To get a team corresponding to the index value.
    ///
    /// Takes a usize and returns a TEAM struct.
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
    /// Takes __Team__ enum value and converts is to __String__ value.
    pub fn teamstr_from_team<'a>(team: Team) -> &'a str {
        match team {
            Team::Red    => "Red",
            Team::Green  => "Green",
            Team::Blue   => "Blue",
            Team::Yellow => "Yellow",
        }
    }
    
    /// Takes the __Team__ value and returns the respective index.
    pub fn to_index(&self) -> usize {
        match self {
            Team::Red    => 0,
            Team::Blue   => 1,
            Team::Green  => 2,
            Team::Yellow => 3,
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Player████*/
/*-----------------------------------------------------------------------------------------------*/
impl Player {
    /// Used to initialize a new player based on the inputs.
    ///
    /// Takes the name, team and is_defender boolean value to create a __Player__ struct.
    pub fn from(
        name: String,
        team: Team,
        is_defender: bool,
        quadrant_active: usize,
        quadrant: Quadrant,
    ) -> Result<Self, Error> {
        // Check if len is of valid size. i.e. less than 50.
        Self::is_name_len_valid(name.len())?;
        // Finally.
        Ok(Player {
            name,
            pieces: Piece::init_pieces(is_defender, quadrant, quadrant_active)?,
            team,
            is_defender,
            is_winner: false,
            is_play: true,
        }
        .sort_pieces_get())
    }

    /// To set the player as a winner.
    ///
    /// Changes the __is_winner__ value to __true__.
    /// Conversely, __set_not_winner__ will set the value to true.
    pub fn set_winner(&mut self) {
        self.is_winner = true;
    }

    /// Set not winner function.
    pub fn set_not_winner(&mut self) {
        self.is_winner = false;
    }

    /// To set player __is_play__ value to true.
    ///
    /// Conversely, __set_not_play__ funtion will convert to false.
    pub fn set_play(&mut self) {
        self.is_play = true;
    }

    /// Player not play function.
    pub fn set_not_play(&mut self) {
        self.is_play = false;
    }

    /// To check wether a index is even possible.
    pub(crate) fn is_valid_player_index(pos: usize) -> Result<(), Error> {
        // 4 is the maximum number of players that can play.
        match pos < 4_usize {
            true  => Ok(()),
            false => Err(Error::IllegalPlayerVectorIndex(pos)),
        }
    }

    /// To check if the position is less than the vector length.
    pub(crate) fn is_in_bounds(pos: usize, len: usize) -> Result<(), Error> {
        match pos < len {
            true  => Ok(()),
            false => Err(Error::PlayerVectorIndexOutOfBounds(pos, len)),
        }
    }

    /// For sorting the pieces.
    ///
    /// TimeSorts the pieces based on their position value.
    fn sort_pieces(&mut self) { self.pieces.sort_by(|a, b| a.position.cmp(&b.position)) }

    /// To sort and return the object.
    ///
    /// Used when intializing the player struct.
    fn sort_pieces_get(mut self) -> Self {
        self.sort_pieces();
        self
    }

    /// To get the players in the present turn.
    pub fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    /// To get the players in the present turn.
    pub fn pieces_mut(&mut self) -> &mut Vec<Piece> {
        &mut self.pieces
    }

    /// A simple function to check if the name length is too big or too small.
    ///
    /// returns false if the name length is invalid. The constraints are 3 > name_length < 255.
    /// else returns true.
    fn is_name_len_valid(len: usize) -> Result<(), Error> {
        // Name length of 46 seems to be an average length all over the world so I dcided to go
        // with 50 as the name length limit. Also, the size of the name fits well within the game
        // screen and doesn't cause overlapping issues.
        match len > 2 && len < 50 {
            true  => Ok(()),
            false => Err(Error::InvalidNameLength(len)),
        }
    }

    /// Binary search to get the piece.
    pub fn search(&self, x: f32, y: f32) -> Result<usize, usize> {
        self.pieces
            .binary_search_by(|piece|
                piece
                .position
                .cmp(
                    &Position {
                        x: x as i32,
                        y: y as i32,
                    }
                )
            )
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerAction for Player████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerAction for Player {
    /// To get position of the piece that is clicked in relevance to the vec.
    ///
    /// Takes x and y coordinate position of the clicked piece. Then checks the piece which is
    /// being referenced and returns the position of the piece inside the __pieces__ vector.
    /// This can be used to update the piece position. Returns null if not piece exist at that
    /// location.
    fn get_piece_pos(&self, x: i32, y: i32) -> Result<Option<usize>, Error> {
        let position_holder = Position::from(x, y)?;
        Ok(self
            .pieces
            .iter()
            .position(|&pos| pos.position == position_holder)
        )
    }

   /// To kill a piece inside the __Player__ struct.
    ///
    /// Takes the position of the piece, finds it inside the vector and deletes that piece.
    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error> {
        Piece::is_valid_index(pos, self.is_defender)?;
        // To check if the position exists inside the vector.
        Piece::is_in_bounds(pos, self.pieces.len())?;
        // Finally.
        let piece = self.pieces.remove(pos);
        self.sort_pieces();
        Ok(piece)
    }

    /// To update position of the piece inside the __Player__ struct vector.
    ///
    /// Takes x and y coordinates to update the piece at the position that is provided.
    /// returns a result type in case of errors.
    fn update_piece(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error> {
        // Is the index valid.
        Piece::is_valid_index(pos, self.is_defender)?;
        // To check if the position exists inside the vector.
        Piece::is_in_bounds(pos, self.pieces.len())?;
        // if piece already at that position.
        match self.pieces[pos].position.x == x && self.pieces[pos].position.y == y {
            true  => Ok(false),
            false => {
                // Finally.
                self.pieces
                    .get_mut(pos)
                    // Hopefully won't panic because previously checked.
                    .expect("Invalid piece position {pos}")
                    .update_pos(x, y)?;
                // sort the pieces.
                self.sort_pieces();
                Ok(true)
            },
        }
    }

    /// To kill itself.
    ///
    /// Suicide.
    fn kill_self(&mut self) {
        self.set_not_play();
        self.set_not_winner();
    }
}
/*-----------------------------------------------------------------------------------------------*/
