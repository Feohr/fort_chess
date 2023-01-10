//! # main fort builder module.
//!
//! Holds the entry point and the interface to interact with the below library.
//! handles initialization, run and execution of the game.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

#![feature(drain_filter)]

// Modules      //
//--------------//
pub mod board;
pub mod game;
pub mod pieces;
pub mod player;
//--------------//

use game::Game;
use player::Player;
use std::time::SystemTime;
use thiserror::Error;

/// Holds the breadth size of the board.
pub const BREADTH   : i32   = 2_i32;
// To print red output.
pub const RED       : &str  = "\x1b[31;1m";
// To reset stdout. i.e. white.
pub const RST       : &str  = "\x1b[0m";

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
        RST,
    )]
    InvalidQuadrantIndex(usize),
    /// If position does not exist inside a quadrant.
    #[error(
        "{} The provided position ({0}, {1}) does not exist inside a quadrant. {}",
        RED,
        RST,
    )]
    PositionNotInQuadrant(i32, i32),
    /// When more than one winner exists.
    #[error("{} There seems to be more than one winner. {}", RED, RST)]
    MoreThanOneWinner(usize),
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// Takes a [`player::Player`] vector and checks for the number of winners.
#[inline]
fn results(mut winners: Vec<Player>) -> Result<Option<Player>, Error> {
    match winners.len() {
        0_usize => Ok(None), // Draw
        1_usize => Ok(Some(winners.remove(0))),
        _ => Err(Error::MoreThanOneWinner(winners.len())),
    }
}

/// To check the winner of the game and close it.
///
/// Result enum with the winner player. If the return value is `None` then the game is a draw.
#[inline]
pub fn exit(game: Game) -> Result<Option<Player>, Error> {
    let winners: Vec<Player> = game
        .players
        .into_iter()
        .filter(|player| player.is_winner)
        .collect::<Vec<Player>>();
    results(winners)
}

/// To get a random value between 1 and 6.
///
/// Simulates a dice roll to get a value between 1 to 6. In exceptional cases you might be
/// constantly getting 1 as return value. This will happen if the system date is set before
/// 01/01/1970 (Unix Epoch). Please regain sanity and change it back to the current date.
#[inline]
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

/// Function to take in a number and reduce exactly 1 from it if it is greater than 0.
/// Used to fix the zero axis 'issue' in the board.
///
/// Works for `u8`, `usize`, `u16`, `u32`.
#[inline]
pub fn decrement_if_positive<T>(x: T) -> T
where
    T:      std::cmp::PartialOrd<T>
        +   std::ops::Sub<Output = T>
        +   From<u16>,
{
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
