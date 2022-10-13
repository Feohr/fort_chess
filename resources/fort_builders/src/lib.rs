//! # main fort builder module.
//!
//! Holds the entry point and the interface to interact with the below library.
//! handles initialization, run and execution of the game.
//!
//! ```
//! # use fort_builder::*;
//!
//! # fn main() {
//!     let bool = {
//!         dice_roll() % 2 == 1
//!     };
//!
//!     let player1 = Player::init("player1".to_string(), team::Red, bool);
//!     let player2 = Player::init("player2".to_string(), team::Green, !bool);
//!
//!     let game = Game::init(player1, player2);
//!     launch(&game)?;
//!
//!     // Safety net cause I don't trust myself to write good code.
//!     let mut safety_net = 0;
//!
//!     // To handle killed pieces.
//!     let killed_pieces = Vec::new();
//!
//!     loop {
//!         // Display hover graphic
//!         display_hover_graphics()?;
//!
//!         // Checks if the game is in exit state or not.
//!         if game.is_exit() || safety_net > 10_000 {
//!             break;
//!         }
//!
//!         if mousepressed() {
//!             let (p_x, p_y) = listen_mouse_press()?;
//!             if let Some(pos) = self.players[self.turn].get_piece_pos(p_x, p_y) {
//!                 let (x, y) = listen_mouse_press()?;
//!                 // is x and y free?
//!                 let piece_killed = check_piece_in_pos_get(x, y)?;
//!                 if let Some(dead_piece) = piece_killed {
//!                     killed_pieces.push(dead_piece);
//!                 }
//!                 let updated = game.update(x, y, pos)?;
//!                 if !updated {
//!                     continue;
//!                 }
//!             }
//!             // Next turn
//!             game.next();
//!         }
//!         // hunt and kill non-playing players.
//!         game.hunt()?;
//!         safety_net += 1;
//!     }
//!     exit(&game).unwrap();
//! #}
//! ```

pub mod game;
pub mod player;

mod pieces;

use game::Game;
use player::{ Player, Team };
use thiserror::Error;
use std::time::SystemTime;

pub mod board {
    //! # board module
    //!
    //! module to hold board specific values like dimensions etc.
    //! 
    //! Quadrant 1 = left block
    //! Quadrant 2 = top block
    //! Quadrant 3 = right block
    //! 
    //! Contents:
    //!     X_MAX   (const)
    //!     X_MIN   (const)
    //!     Y_MAX   (const)
    //!     Y_MIN   (const)
    //!     RGT     (const)
    //!     LFT     (const)
    //!     TOP     (const)
    //!     BTM     (const)

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
    pub const LFT: i32 = -13_i32;

    /// Board's top most length in view.
    pub const TOP: i32 =  10_i32;

    /// Board's bottom most length in view.
    pub const BTM: i32 =  -3_i32;
}

/// Error enum to handle errors across the lib.
///
#[derive(Error, Debug)]
pub enum Error {
    /// To handle runtime IO errors.
    #[error("Ran into runtime error: {0}")]
    RunTimeError(#[from] std::io::Error),

    /// Invalid position.
    #[error("The position ({0}, {1}) is invalid")]
    IllegalPosition(i32, i32),

    /// If the name is too long or too short.
    #[error(
        "The name is either too long or too short. Ideal length is (3 < name < 255). \
        Your name length: {0}"
    )]
    InvalidNameLength(usize),

    /// If the position referenced is not present in the pieces vector.
    #[error("The given index of the piece {0} does not exist in a vec of length {1}.")]
    PieceVectorIndexOutOfBounds(usize, usize),

    /// When an illegal position is referenced.
    #[error("The given index cannot exist as the vector index can only be (0 < length < 24)")]
    IllegalVectorIndex(usize),

    /// When more than one winner exists.
    #[error("There seens to be more than one winner")]
    MoreThanOneWinner(usize),
}

/// A light weight representation of the __Player__ struct.
///
/// A ligth weight representation of __Player__ struct since, we do not really need to clone the
/// whole damn struct for simple info about the player. __Pieces__ and indicators aren't of use.
///
/// Contents:
///     name: name of the player.
///     team: team of the player.
#[derive(Debug)]
pub struct PlayerLW<'a> {
    name: &'a str,
    team: &'a str,
}

impl<'a> PlayerLW<'a> {
    /// To create a new __PlayerLW__ struct.
    ///
    /// Takes string name input and team name to create a __PlayerLw__ struct.
    /// ```
    /// # fn main() {
    ///     let _player_lw = PlayerLW::new("player".to_string(), Team::Red);
    /// #}
    /// ```
    fn new(name: String, team: Team) -> PlayerLW<'a> {
        PlayerLW {
            name: str_from_string(name.clone()),
            team: PlayerLW::teamstr_from_team(team),
        }
    }

    /// To turn a team enum value to a String value.
    ///
    /// Takes __Team__ enum value and converts is to __String__ value.
    /// ```
    /// # fn main() {
    ///     let _team = teamstr_from_team(Team::Red);
    /// #}
    /// ```
    fn teamstr_from_team(team: Team) -> &'a str {
        match team {
            Team::Red    => "Red",
            Team::Green  => "Green",
            Team::Blue   => "Blue",
            Team::Yellow => "Yellow",
        }
    }
}

/// Simple utility function to create a &str from a String.
///
/// Takes a String argument, clones and returns as &str using stringify macro.
fn str_from_string<'a> (str: String) -> &'a str { stringify!("{}", str) }

/// To check the winner of the game and close it.
///
/// Result enum with the winner player. If the return value is __None__ then the game is a draw.
pub fn exit<'a>(game: Game) -> Result<Option<PlayerLW<'a>>, Error> {
    if game.is_exit() {
        return Ok(None);
    }
    // To save the number of winners.
    let mut winners: Vec<PlayerLW> = Vec::new();
    for player in &game.players {
        if player.is_winner {
            winners.push(PlayerLW::new(
                                    player.name.clone(),
                                    player.team.clone(),
                                )
                    );
        }
    }
    // Draw
    if winners.len() == 0 { return Ok(None) }
    // More than one woman
    if winners.len() > 1 { return Err(Error::MoreThanOneWinner(winners.len())) }
    // Finally
    Ok(Some(winners.remove(0)))
}

/// To get a random value between 1 and 6.
///
/// Simulates a dice roll to get a value between 1 to 6. In exceptional cases you might be
/// constantly getting 1 as return value. This will happen if the system date is set before 
/// 01/01/1970 (Unix Epoch). Please regain sanity and change it back to the current the date.
pub fn dice_roll() -> u8 {
    if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        // The values after mod 6 will be in-between 0 and 5.
        // Adding 1 to make the range as 1 to 6.
        return ((n.as_secs() % 6_u64) + 1_u64) as u8;
    }
    // You should not be reaching this but if you do then your computer date is stuck before the
    // 70's.
    eprintln!("System date value earlier than EPOCH");
    1_u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_from_string() {
        assert!("str1", str_from_string("str1".to_string()))
    }
}
