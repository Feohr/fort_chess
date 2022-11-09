//! # game module
//!
//! Game module to initialize, stop and exit the game.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::pieces::Piece;
use crate::player::{Player, PlayerAction};
use crate::Error;

/// A struct to create a game object.
///
/// ## Contents
/// -   players: players vector
/// -   status": state of the game
#[derive(Debug)]
pub struct Game {
    /// To hold the player information.
    pub players: Vec<Player>,

    /// To hold the turn of the player.
    pub turn: usize,

    /// To hold the game update state to draw.
    pub update: bool,

    /// To track if therre is a piece picked.
    pub picked: bool,
}

/// To handle operations over the Game.
///
/// ## Contents:
/// -   hunt:   searches for pieces to kill and kills them.
/// -   update: updates the given player piece with x nd y value.
/// -   next:   gets the player index who's turn it is.
pub trait GameAction {
    fn hunt(&mut self) -> Result<Option<Player>, Error>;

    fn next_player(&mut self);

    fn update_position(&mut self, x: i32, y: i32, pos: usize) -> Result<(), Error>;
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
impl Game {
    /// To initialize game object.
    ///
    /// Takes player1 and player2 argument to create a __Game__ struct.
    ///
    /// In future, the function should handle two extra optional player argument.
    /// Omitted presently to avoid complexity.
    pub fn init(players: Vec<Player>) -> Self {
        Game {
            players,
            turn: 0_usize,
            update: true,
            picked: false,
        }
    }

    /// To kill a player.
    ///
    /// Takes the player position in vec and removes them.
    /// Returns a Result value hence needs to be error handled.
    fn kill(&mut self, pos: usize) -> Result<Player, Error> {

        Player::is_in_bounds(pos, self.players.len())?;

        Player::is_valid_player_index(pos)?;

        self.players
            .get_mut(pos)
            .expect("Invalid Player position {pos}")
            .kill_self();

        Ok(self.players.remove(pos))

    }

    /// To know if the x and y holds a piece of another player.
    ///
    /// Takes x and y values and iterates over all the players in the games to decide which
    /// particular piece is present and removes that piece to return it.
    ///
    /// This function does not check if there are more than one pieces at a position.
    /// This function will kill all the pieces at a position.
    /// Shouldn't cause unwanted issues as there can only be one piece at a position at any given time.
    pub fn piece_in_pos(&mut self, x: i32, y: i32) -> Result<Option<Piece>, Error> {

        Piece::in_board_range(x, y)?;

        for player in self.players.iter_mut() {
            if let Ok(index) = player.piece_index_from_xy_i32(x, y) {
                return Ok(Some(player.kill_piece(index)?))
            }
        }

        Ok(None)

    }

    pub fn check_piece_in_pos(&self, x: f32, y: f32) -> bool {

        for player in self.players.iter() {
            if player.piece_index_from_xy_f32(x, y).is_ok() {
                return true;
            }
        }

        return false;

    }

    /// To change the game state to __true__.
    ///
    /// Takes __self__ reference and changes status to __true__.
    ///
    /// **Idempotent function**
    pub fn set_update_true(&mut self) { self.update = true }

    /// To change the game update state to __false__.
    ///
    /// Takes __self__ reference and changes status to __false__.
    ///
    /// **Idempotent function**
    pub fn set_update_false(&mut self) { self.update = false }

    /// To change the game picked state to __true__.
    ///
    /// Takes __self__ reference and changes picked to __true__.
    ///
    /// **Idempotent function**
    pub fn set_picked_true(&mut self) { self.picked = true }

    /// To change the game picked state to __false__.
    ///
    /// Takes __self__ reference and changes picked to __false__.
    ///
    /// **Idempotent function**
    pub fn set_picked_false(&mut self) { self.picked = false }

    /// Return the players length.
    pub fn player_count(&self) -> usize { self.players.len() }

    /// To return the current __Player__ struct.
    pub fn current_player(&self) -> &Player { &self.players[self.turn] }

    /// Mutable reference to the player.
    pub fn current_player_mut(&mut self) -> &mut Player { &mut self.players[self.turn] }
}
/*-----------------------------------------------------------------------------------------------*/

/*████GameAction for Game████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAction for Game {
    /// Hunt for losers and kill them!
    ///
    /// function that takes self reference and searches for pieces that need to be killed at each
    /// iteration.
    fn hunt(&mut self) -> Result<Option<Player>, Error> {

        for (index, player) in self.players.iter().enumerate() {

            if (    player.pieces.is_empty()
                && !player.is_winner
            )   || !player.is_play {

                let kill = self.kill(index)?;

                return Ok(Some(kill));

            }

        }

        return Ok(None)

    }

    /// Changes the turn value to indiacate which player turn it is.
    ///
    /// Adds value to turn and changes to 0 if the value exceeds players vector len.
    fn next_player(&mut self) {

        match self.turn < self.players.len() - 1 {
            true => self.turn += 1,
            false => self.turn = 0,
        }

    }

    /// To update player pieces position every turn.
    fn update_position(&mut self, x: i32, y: i32, pos: usize) -> Result<(), Error> {

        Ok(match self.players[self.turn].update_piece(x, y, pos)? {
            true => self.set_update_true(),
            false => (),
        })

    }
}
/*-----------------------------------------------------------------------------------------------*/
