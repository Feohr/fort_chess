//! # player module
//!
//! Contains the player objects and corresponding operations.
//! Used to create, kill, set values and to update position of the player pieces.
//! Also contains the team object to handle the player teams.

use crate::Error;
use crate::pieces::Piece;

/// Contains the types of teams.
///
/// Used to distinguish players from team to team.
/// Each player must have a unique team.
/// Maximum of only four players can play at a time.
///
/// # Contents:
/// -   Red
/// -   Blue
/// -   Green
/// -   Yellow
#[derive(Copy, Clone, Debug)]
pub enum Team {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Team {
/// To get a team corresponding to the index value.
///
/// Takes a usize and returns a TEAM struct.
    pub fn from_index(index: usize) -> Result<Self, Error> {
        match index {
            0   =>  Ok(Team::Red),
            1   =>  Ok(Team::Blue),
            2   =>  Ok(Team::Green),
            3   =>  Ok(Team::Yellow),
            _   =>  Err(Error::InvalidTeamIndex(index)),
        }
    }
}


/// __Player__ struct used to handle player specific information.
///
/// Contains data such as name, pieces held, team, etc.
///
/// # Contents:
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

/// A simple function to check if the name length is too big or too small.
///
/// returns false if the name length is invalid. The constraints are 3 > name_length < 255.
/// else returns true.
fn is_name_len_valid(len: usize) -> bool {
    // Name length of 46 seems to be an average length all over the world so I dcided to go with 50
    // as the name length limit. Also, the size of the name fits well within the game screen and
    // doesn't cause overlapping issues.
    return len > 2 && len < 50;
}

impl Player {
    /// Used to initialize a new player based on the inputs.
    ///
    /// Takes the name, team and is_defender boolean value to create a __Player__ struct.
    pub fn from(name: String, team: Team, is_defender: bool) -> Result<Self, Error> {
        // Check if len is of valid size. i.e. less than 50.
        let len = name.len();
        if !is_name_len_valid(len) {
            return Err(Error::InvalidNameLength(len));
        }
        // Finally.
        Ok(Player {
            name,
            pieces: Vec::default(),
            team,
            is_defender,
            is_winner: false,
            is_play: true,
        })
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
}

/// A public trait to handle __Player__ actions during runtime.
///
/// # Actions:
/// -   get_piece_pos:  To get piece position within the struct based on x, y coordinates.
/// -   kill_piece:     To remove a piece from the __pieces__ vector.
/// -   update_piece:   To update piece position.
pub trait PlayerAction {
    fn get_piece_pos(&self, x: i32, y: i32) -> Option<usize>;

    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error>;

    fn update_piece(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error>;
}

/// To check wether a vector index is valid.
///
/// There can be a maximum of 24 pieces inside a player pieces vec. Anything more than that is an
/// error.
pub fn is_valid_index(pos: usize, is_defender: bool) -> bool { 
    if is_defender {
        return pos < 24;
    }
    return pos < 8;
}

impl PlayerAction for Player {
    /// To get position of the piece that is clicked in relevance to the vec.
    ///
    /// Takes x and y coordinate position of the clicked piece. Then checks the piece which is
    /// being referenced and returns the position of the piece inside the __pieces__ vector.
    /// This can be used to update the piece position. Returns null if not piece exist at that
    /// location.
    fn get_piece_pos(&self, x: i32, y: i32) -> Option<usize> { 
        for (index, piece) in self.pieces.iter().enumerate() {
            if piece.position.x == x && piece.position.y == y {
                return Some(index);
            }
        }
        return None;
    }

    /// To kill a piece inside the __Player__ struct.
    ///
    /// Takes the position of the piece, finds it inside the vector and deletes that piece.
    fn kill_piece(&mut self, pos: usize) -> Result<Piece, Error> {
        if !is_valid_index(pos, self.is_defender) {
            return Err(Error::IllegalVectorIndex(pos));
        }
        // To check if the position exists inside the vector.
        let len = self.pieces.len();
        if len < pos {
            return Err(Error::PieceVectorIndexOutOfBounds(pos, len));
        }
        // Finally.
        Ok(self.pieces.remove(pos))
    }

    /// To update position of the piece inside the __Player__ struct vector.
    ///
    /// Takes x and y coordinates to update the piece at the position that is provided.
    /// returns a result type in case of errors.
    fn update_piece(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error> {
        if !is_valid_index(pos, self.is_defender) {
            return Err(Error::IllegalVectorIndex(pos));
        }
        // To check if the position exists inside the vector.
        let len = self.pieces.len();
        if len < pos {
            return Err(Error::PieceVectorIndexOutOfBounds(pos, len));
        }
        // if piece already at that position.
        if self.pieces[pos].position.x == x
        && self.pieces[pos].position.y == y {
            return Ok(false);
        }
        // Finally.
        self.pieces[pos].update_pos(x, y)?;
        return Ok(true);
    }
}
