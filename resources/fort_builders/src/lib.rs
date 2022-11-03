//! # main fort builder module.
//!
//! Holds the entry point and the interface to interact with the below library.
//! handles initialization, run and execution of the game.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

pub mod game;
pub mod player;
pub mod board;
pub mod pieces;

use game::Game;
use player::Team;
use std::time::SystemTime;
use thiserror::Error;

// To print red output.
pub const RED: &str = "\x1b[31;1m";
// To reset stdout. i.e. white.
pub const RST: &str = "\x1b[0m";

/// Error enum to handle errors across the lib.
#[derive(Error, Debug)]
pub enum Error {
    /// To handle runtime IO errors.
    #[error("{} Ran into runtime error: {0} {}", RED, RST)]
    RunTimeError(#[from] std::io::Error),

    // Piece.
    /// Piece module errors,
    #[error("{} Error in the piece module: {0} {}", RED, RST)]
    PieceModuleError(#[from] pieces::Error),

    // Player.
    /// Player modile error.
    #[error("{} Error in the player module: {0} {}", RED, RST)]
    PlayerModuleError(#[from] player::Error),

    // Misc.
    /// If invalid Quadrant index was provided.
    #[error(
        "{} The provided index {0} does not have a quadrant corresponding to it. {}",
        RED,
        RST
    )]
    InvalidQuadrantIndex(usize),

    /// When more than one winner exists.
    #[error("{} There seens to be more than one winner. {}", RED, RST)]
    MoreThanOneWinner(usize),
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

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████PlayerLW████*/
/*-----------------------------------------------------------------------------------------------*/
impl<'a> PlayerLW<'a> {
    /// To create a new __PlayerLW__ struct.
    ///
    /// Takes string name input and team name to create a __PlayerLw__ struct.
    fn new(name: String, team: Team) -> PlayerLW<'a> {
        PlayerLW {
            name: str_from_string(name),
            team: Team::teamstr_from_team(team),
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/// Simple utility function to create a &str from a String.
///
/// Takes a String argument, clones and returns as &str using stringify macro.
fn str_from_string<'a>(_str: String) -> &'a str {
    // Simplest way I could come up with to create a string from a String without the
    // borrow-checker going crazy. Need a better way to implement this in the future.
    stringify!("{}", _str)
}

/// Takes a PlayerLW  vector and checks for the number of winners.
fn results<'a>(mut winners: Vec<PlayerLW<'a>>) -> Result<Option<PlayerLW<'a>>, Error> {
    match winners.len() {
        // Draw.
        0 => Ok(None),
        // Winner.
        1 => Ok(Some(winners.remove(0))),
        // More than one winner.
        _ => Err(Error::MoreThanOneWinner(winners.len())),
    }
}

/// To check the winner of the game and close it.
///
/// Result enum with the winner player. If the return value is __None__ then the game is a draw.
pub fn exit<'a>(game: Game) -> Result<Option<PlayerLW<'a>>, Error> {
    // Gathering the winners.
    let winners: Vec<PlayerLW> = game
        .players
        .iter()
        .filter_map(|player|
            match player.is_winner {
                true  => Some(PlayerLW::new(player.name.clone(), player.team.clone())),
                false => None,
            }
        )
        .collect::<Vec<PlayerLW>>();
    // Finally.
    // Match to declare the result.
    results(winners)
}

/// To get a random value between 1 and 6.
///
/// Simulates a dice roll to get a value between 1 to 6. In exceptional cases you might be
/// constantly getting 1 as return value. This will happen if the system date is set before
/// 01/01/1970 (Unix Epoch). Please regain sanity and change it back to the current date.
pub fn dice_roll() -> usize {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n)  => (n.as_secs() % 6_u64) as usize,
        Err(_) => {
            // You should not be reaching this but if you do then your computer date is stuck
            // before the 70's.
            eprintln!("{} System date set earlier than UNIX EPOCH {}", RED, RST);
            1_usize
        }
    }
}

// To get rid of the extra "Zeroeth" column along the y-axis.
// Couldn't just add it to the loop.
// It's a long explaination... I had to use this cause, I suck at coding.
// Can use with i32, i64, u16, u32, u64, isize and usize.
pub fn ret_minus_one<T>(x: T) -> T
where
    T:      std::cmp::PartialOrd<T>
        +   std::ops::Sub<Output=T>
        +   From<u16> {
    match x > From::from(0) {
        true  => x - From::from(1),
        false => x,
    }
}

/*████Tests██████████████████████████████████████████████████████████████████████████████████████*/

#[cfg(test)]
mod tests {
    //    use super::*;
    #[test]
    fn test_str_from_string() {
        assert!("str1", str_from_string("str1".to_string()))
    }
}
