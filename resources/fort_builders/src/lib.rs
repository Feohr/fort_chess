//! # main fort builder module.
//!
//! Holds the entry point and the interface to interact with the below library.
//! handles initialization, run and execution of the game.
//!

pub mod game;
pub mod player;
mod pieces;

use game::Game;
use player::Team;
use thiserror::Error;
use std::time::SystemTime;

// To print red output.
const RED: &str = "\x1b[31;1m";
// To reset stdout. i.e. white.
const RST: &str = "\x1b[0m";

pub mod board {
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

    // Board tile borders.
    /// Board's right most x axis length.
    pub const X_MAX: i32 =  8_i32;

    /// Board's left most x acis length.
    pub const X_MIN: i32 = -8_i32;

    /// Board's top most y axis length.
    pub const Y_MAX: i32 =  8_i32;

    /// Board's down most y axis length.
    pub const Y_MIN: i32 = -2_i32;

    // Camera view over the board.
    /// Board's right most length in view.
    pub const RGT: i32 =  12_i32;

    /// Board's left most length in view.
    /// Extra one lower as the "Zeroeth" column is present.
    pub const LFT: i32 = -13_i32;

    /// Board's top most length in view.
    pub const TOP: i32 =  10_i32;

    /// Board's bottom most length in view.
    pub const BTM: i32 =  -4_i32;

    /// Quadrants inside the game.
    ///
    /// ## Contents:
    /// -   Block 1
    /// -   Block 2
    /// -   Block 3
    pub enum Quadrant {
        /// Block 1.
        Q1,
        /// Block 2.
        Q2,
        /// Block 3.
        Q3,
    }
    
    impl Quadrant {
        pub fn from_index(index: usize) -> Result<Self, super::Error> {
            match index {
                0   =>  Ok(Quadrant::Q1),
                1   =>  Ok(Quadrant::Q2),
                2   =>  Ok(Quadrant::Q2),
                _   =>  Err(super::Error::InvalidQuadrantIndex(index)),
            }
        }
    }
}

/// Error enum to handle errors across the lib.
///
#[derive(Error, Debug)]
pub enum Error {
    /// To handle runtime IO errors.
    #[error("{} Ran into runtime error: {0} {}", RED, RST)]
    RunTimeError(#[from] std::io::Error),

    /// Invalid position.
    #[error("{} The position ({0}, {1}) is invalid {}", RED, RST)]
    IllegalPosition(i32, i32),

    /// If the name is too long or too short.
    #[error(
        "{} The name is either too long or too short. Ideal length is (3 < name < 255). \
        Your name length: {0} {}", RED, RST
    )]
    InvalidNameLength(usize),

    /// If the position referenced is not present in the pieces vector.
    #[error("{} The given index of the piece {0} does not exist in a vec of length {1}. {}",
    RED, RST)]
    PieceVectorIndexOutOfBounds(usize, usize),

    /// If the position referenced is not present in the pieces vector.
    #[error("{} The given index of the piece {0} does not exist in a vec of length {1}. {}",
    RED, RST)]
    PlayerVectorIndexOutOfBounds(usize, usize),

    /// When an illegal position is referenced.
    #[error("{} The given index {0} cannot exist as the index for a piece vector should be \
    (0 < length < 24 | 8). {}", RED, RST)]
    IllegalPieceVectorIndex(usize),

    /// When an illegal position is referenced.
    #[error("{} The given index {0} cannot exist as the index for a player vector should be \
    (0 < length < 4). {}", RED, RST)]
    IllegalPlayerVectorIndex(usize),

    /// When more than one winner exists.
    #[error("{} There seens to be more than one winner. {}", RED, RST)]
    MoreThanOneWinner(usize),

    /// If Invalid Team index was provided.
    #[error("{} The provided index {0} does not have a team corresponding to it. {}", RED, RST)]
    InvalidTeamIndex(usize),

    /// If Invalid Piece type index was provided.
    #[error("{} The provided index {0} does not have a piece type corresponding to it. {}",
    RED, RST)]
    InvalidPieceTypeIndex(u8),

    /// If error faced during creation of __Piece__ vector.
    #[error("{} Ran into issue while creating vectors. {}",
    RED, RST)]
    EmptyPieceVectorCreated,

    /// If invalid Quadrant index was provided.
    #[error("{} The provided index {0} does not have a quadrant corresponding to it. {}",
    RED, RST)]
    InvalidQuadrantIndex(usize),
}

/// A light weight representation of the __Player__ struct.
///
/// A ligth weight representation of __Player__ struct since, we do not really need to clone the
/// whole damn struct for simple info about the player. __Pieces__ and indicators aren't of use.
///
/// ## Contents:
/// -   name: name of the player.
/// -   team: team of the player.
#[derive(Debug)]
pub struct PlayerLW<'a> {
    /// To hold the light weight representation of the name of the player.
    name: &'a str,

    /// To hold the light weight representation of the team name of the player.
    team: &'a str,
}

impl<'a> PlayerLW<'a> {
    /// To create a new __PlayerLW__ struct.
    ///
    /// Takes string name input and team name to create a __PlayerLw__ struct.
    fn new(name: String, team: Team) -> PlayerLW<'a> {
        PlayerLW {
            name: str_from_string(name),
            team: teamstr_from_team(team),
        }
    }
}

/// To turn a team enum value to a String value.
///
/// Takes __Team__ enum value and converts is to __String__ value.
fn teamstr_from_team<'a>(team: Team) -> &'a str {
    match team {
        Team::Red    => "Red",
        Team::Green  => "Green",
        Team::Blue   => "Blue",
        Team::Yellow => "Yellow",
    }
}

/// Simple utility function to create a &str from a String.
///
/// Takes a String argument, clones and returns as &str using stringify macro.
fn str_from_string<'a> (_str: String) -> &'a str {
    // Simplest way I could come up with to create a string from a String without the
    // borrow-checker going crazy. Need a better way to implement this in the future.
    stringify!("{}", _str)
}

/// To check the winner of the game and close it.
///
/// Result enum with the winner player. If the return value is __None__ then the game is a draw.
pub fn exit<'a>(mut game: Game) -> Result<Option<PlayerLW<'a>>, Error> {
    // If the game is interrupted then early quit 
    if game.is_interrupt() {
        return Ok(None);
    }
    let mut winners: Vec<PlayerLW> = 
        game
        .players
        .iter()
        .filter_map(
            |player|
            match player.is_winner {
                true    =>  Some(
                                PlayerLW::new(
                                    player.name.clone(),
                                    player.team.clone(),
                                )
                            ),
                false   =>  None,
            }
        )
        .collect::<Vec<PlayerLW>>();
    // Set the game to exit after extracting the results.
    // This step is kinda unecessary but still added for my peace of mind.
    game.set_state_exit();
    // Match to declare the result.
    match winners.len() {
        // Draw.
        0   =>  Ok(None),
        // Winner.
        1   =>  Ok(Some(winners.remove(0))),
        // More than one winner.
        _   =>  Err(Error::MoreThanOneWinner(winners.len())),
    }
}

/// To get a random value between 1 and 6.
///
/// Simulates a dice roll to get a value between 1 to 6. In exceptional cases you might be
/// constantly getting 1 as return value. This will happen if the system date is set before
/// 01/01/1970 (Unix Epoch). Please regain sanity and change it back to the current date.
pub fn dice_roll() -> usize {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n)   =>  ((n.as_secs() % 6_u64) + 1_u64) as usize,
        Err(_)  =>  {
            // You should not be reaching this but if you do then your computer date is stuck
            // before the 70's.
            eprintln!("{} System date set earlier than UNIX EPOCH {}", RED, RST);
            1_usize
        },
    }
}

#[cfg(test)]
mod tests {
//    use super::*;
    #[test]
    fn test_str_from_string() {
        assert!("str1", str_from_string("str1".to_string()))
    }
}
