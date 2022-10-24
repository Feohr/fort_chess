//! # game module
//!
//! Game module to initialize, stop and exit the game.

use crate::pieces::{ Piece, TryIsSingular };
use crate::player::{Player, PlayerAction};
use crate::Error;

/// __GameState__ enum to keep track of game state.
///
/// ## Contents:
/// -   Start
/// -   Running
/// -   Exit
/// -   Interrupt
#[derive(PartialEq, Eq, Debug)]
enum GameState {
    Start,
    Running,
    Exit,
    // Not sure if I'll need this yet.
    Interrupt,
}

/// A struct to create a game object.
///
/// ## Contents:
/// -   players: players vector
/// -   status": state of the game
#[derive(Debug)]
pub struct Game {
    /// To hold the player information.
    pub players: Vec<Player>,

    /// To hold the turn of the player.
    pub turn: usize,

    /// To hold the game state.
    status: GameState,
}

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
            status: GameState::Start,
        }
    }

    /// To kill a player.
    ///
    /// Takes the player position in vec and removes them.
    /// Returns a Result value hence needs to be error handled.
    fn kill(&mut self, pos: usize) -> Result<Player, Error> {
        // If player position is in bounds.
        Player::is_in_bounds(pos, self.players.len())?;
        // if player can have that index at all.
        Player::is_valid_player_index(pos)?;
        // Finally.
        self.players
            .get_mut(pos)
            // Hopefully shouldn't panic as already validated before.
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
    /// Shouldn't cause unwanted issues as there can only be one piece at a position at any given
    /// time.
    pub fn get_all_pieces_in_pos(&mut self, x: i32, y: i32) -> Result<Vec<Piece>, Error> {
        Piece::in_board_range(x, y)?;
        Ok(self.players
            .iter_mut()
            .flat_map(|player| -> Result<Option<Piece>, Error> {
                if let Some(index) = player.get_piece_pos(x, y)? {
                    let res = player.kill_piece(index)?;
                    return Ok(Some(res));
                }
                Ok(None)
            })
            .filter_map(|pos| pos)
            .collect::<Vec<Piece>>()
            .try_is_singular()?)
    }

    /// To change the game state to __Interrupt__.
    ///
    /// Takes __self__ reference and changes status to __Interrupt__.
    ///
    /// **Idempotent function**
    pub fn set_state_interrupt(&mut self) {
        self.status = GameState::Interrupt;
    }

    /// To change the game state to __Exit__.
    ///
    /// Takes __self__ reference and changes status to __Exit__.
    ///
    /// **Idempotent function**
    pub fn set_state_exit(&mut self) {
        self.status = GameState::Exit;
    }

    /// To change the game state to __Running__.
    ///
    /// Takes __self__ reference and changes status to __Running__.///
    /// **Idempotent function**
    pub fn set_state_run(&mut self) {
        self.status = GameState::Running;
    }

    /// To check if the game is still playing or if we need to stop it.
    ///
    /// Takes self argument and cross checks the players.
    /// returns true if the game is ending. else returns false.
    pub fn is_exit(&self) -> bool {
        self.status == GameState::Exit
    }

    /// To check if the game is interrupted.
    ///
    /// Takes a self reference argument and checks the player.
    /// If the returns true then the game should be pre-maturely stopped.
    pub fn is_interrupt(&self) -> bool {
        self.status == GameState::Interrupt
    }
}

/// To handle operations over the Game.
///
/// ## Contents:
/// -   hunt:   searches for pieces to kill and kills them.
/// -   update: updates the given player piece with x nd y value.
/// -   next:   gets the player index who's turn it is.
pub trait GameAction {
    fn hunt(&mut self) -> Result<Vec<Player>, Error>;

    fn next(&mut self);
}

impl GameAction for Game {
    /// Hunt for losers and kill them!
    ///
    /// function that takes self reference and searches for pieces that need to be killed at each
    /// iteration.
    fn hunt(&mut self) -> Result<Vec<Player>, Error> {
        Ok(self.players
            .iter()
            .enumerate()
            .filter_map(|(index, player)| {
                match (player.pieces.is_empty() && !player.is_winner) || !player.is_play {
                    true  => Some(index),
                    false => None,
                }
            })
            .collect::<Vec<usize>>()
            .into_iter()
            .flat_map(|index| -> Result<Player, Error> {
                let kill = self.kill(index)?;
                Ok(kill)
            })
            .collect::<Vec<Player>>())
    }

    /// Changes the turn value to indiacate which player turn it is.
    ///
    /// Adds value to turn and changes to 0 if the value exceeds players vector len.
    fn next(&mut self) {
        match self.turn < self.players.len() {
            true  => self.turn += 1,
            false => self.turn = 0,
        }
    }
}
