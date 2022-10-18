//! # game module
//!
//! Game module to initialize, stop and exit the game.

use crate::player::{ Player, is_valid_index, PlayerAction };
use crate::Error;
use crate::pieces::{ Piece, Position };

/// __GameState__ enum to keep track of game state.
///
/// # Contents:
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
/// # Contents:
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

/// To handle operations over the Game.
///
/// # Contents:
/// -   hunt:   searches for pieces to kill and kills them.
/// -   update: updates the given player piece with x nd y value.
/// -   next:   gets the player index who's turn it is.
pub trait GameAction {
    fn hunt(&mut self) -> Result<Option<Player>, Error>;

    fn update(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error>;

    fn next(&mut self);
}

/// To check wether the position is even valid index.
fn is_valid_player_index(pos: usize) -> bool {
    // 4 is the maximum number of players that can play.
    return pos < 4;
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
        if !is_valid_player_index(pos) {
            return Err(Error::IllegalVectorIndex(pos))
        }
        let len = self.players.len();
        if pos >= len {
            eprintln!("Cannot kill the piece as already dead");
            return Err(Error::PieceVectorIndexOutOfBounds(len, pos));
        }
        // Finally
        self.players[pos].set_not_play();
        self.players[pos].set_not_winner();
        Ok(self.players.remove(pos))
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
    /// Takes __self__ reference and changes status to __Running__.
    ///
    /// **Idempotent function**
    pub fn set_state_run(&mut self) {
        self.status = GameState::Running;
    }

    /// To check if the game is still playing or if we need to stop it.
    ///
    /// Takes self argument and cross checks the players.
    /// returns true if the game is ending. else returns false.
    pub fn is_exit(&self) -> bool {
        return self.status == GameState::Exit;
    }

    /// To check if the game is interrupted.
    ///
    /// Takes a self reference argument and checks the player.
    /// If the returns true then the game should be pre-maturely stopped.
    pub fn is_interrupt(&self) -> bool {
        return self.status == GameState::Interrupt;
    }

    /// To know if the x and y holds a piece of another player.
    ///
    /// Takes x and y values and iterates over all the players in the games to decide which
    /// particular piece is present and removes that piece to return it.
    pub fn check_piece_in_pos_get(&mut self, x: i32, y: i32) -> Result<Option<Piece>, Error> {
        if !Position::in_range(x, y) {
            return Err(Error::IllegalPosition(x, y));
        }
        for player in self.players.iter_mut() {
            if let Some(index) = player.get_piece_pos(x, y) {
                let res = player.kill_piece(index)?;
                return Ok(Some(res));
            }
        }
        return Ok(None);
    }
}

impl GameAction for Game {
    /// Hunt for losers and kill them!
    ///
    /// function that takes self reference and searches for pieces that need to be killed at each
    /// iteration.
    fn hunt(&mut self) -> Result<Option<Player>, Error> {
        for (index, player) in self.players.iter().enumerate() {
            if ( player.pieces.len() == 0 && !player.is_winner ) 
                || !player.is_play {
                let kill = self.kill(index)?;
                return Ok(Some(kill));
            }
        }
        return Ok(None);
    }

    /// To update the current player piece with x and y pos.
    ///
    /// Takes the x and y value and updates it. A wrapper for update_piece essentially.
    fn update(&mut self, x: i32, y: i32, pos: usize) -> Result<bool, Error> {
        return self.players[self.turn].update_piece(x, y, pos);
    }

    /// Changes the turn value to indiacate which player turn it is.
    ///
    /// Adds value to turn and changes to 0 if the value exceeds players vector len.
    fn next(&mut self) {
        if self.turn < self.players.len() {
            self.turn += 1;
        }
        self.turn = 0;
    }
}
