//! # game mdule
//!
//! Game module to initialize, stop and exit the game.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::pieces::Piece;
use crate::player::{Player, PlayerAction};
use crate::Error;

/// A struct to create a game object.
#[derive(Debug)]
pub struct Game {
    /// To hold the player information.
    pub players: Vec<Player>,
    /// To hold the turn of the player.
    pub turn: usize,
    /// To hold the game update state to draw.
    pub update: bool,
    /// To track if there is a piece picked.
    pub picked: bool,
    /// To notify if the game is still being played.
    pub play: bool,
}

/// To handle operations over the Game.
pub trait GameAction {
    fn hunt(&mut self) -> Vec<Player>;
    fn next_player(&mut self) -> &mut Self;
    fn update_position(&mut self, x: i32, y: i32) -> Result<&mut Self, Error>;
    fn check_piece_in_pos(&self, x: f32, y: f32) -> bool;
    fn remove_piece_in_pos(&mut self, x: f32, y: f32) -> Result<Option<Piece>, Error>;
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Game████*/
/*-----------------------------------------------------------------------------------------------*/
impl Game {
    /// To initialize game object.
    ///
    /// Takes player1 and player2 argument to create a [`Game`] ustruct.
    ///
    /// In future, the function should handle two extra optional player argument.
    /// Omitted presently to avoid complexity.
    #[inline]
    pub fn init(players: Vec<Player>) -> Self {
        Game {
            players,
            turn: 0_usize,
            update: true,
            picked: false,
            play: true,
        }
    }
    /// To change the game state to `true`.
    ///
    /// Takes `self` reference and changes status to `true`.
    ///
    /// `Idempotent function`
    #[inline]
    pub fn set_update_true(&mut self) -> &mut Self {
        self.update = true;
        self
    }
    /// To change the game update state to `false`.
    ///
    /// Takes `self` reference and changes status to `false`.
    ///
    /// `Idempotent function`
    #[inline]
    pub fn set_update_false(&mut self) -> &mut Self {
        self.update = false;
        self
    }
    /// To change the game picked state to `true`.
    ///
    /// Takes `self` reference and changes picked to `true`.
    ///
    /// `Idempotent function`
    #[inline]
    pub fn set_picked_true(&mut self) -> &mut Self {
        self.picked = true;
        self
    }
    /// To change the game play to `true`.
    ///
    /// Takes a mutable self reference and changes play to `true`.
    ///
    /// `Idempotent function`
    pub fn set_play_false(&mut self) -> &mut Self {
        self.play = false;
        self
    }
    /// To change the game picked state to `false`.
    ///
    /// Takes `self` reference and changes picked to `false`.
    ///
    /// `Idempotent function`
    #[inline]
    pub fn set_picked_false(&mut self) -> &mut Self {
        self.picked = false;
        self
    }
    /// To return the current [`Player`] struct.
    #[inline]
    pub fn current_player(&self) -> &Player {
        &self.players[self.turn]
    }
    /// Mutable reference to the player.
    #[inline]
    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.turn]
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████GameAction for Game████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameAction for Game {
    /// Hunt for losers and kill them!
    ///
    /// function that takes self reference and searches for pieces that need to be killed at each
    /// iteration.
    #[inline]
    fn hunt(&mut self) -> Vec<Player> {
        self.players
            .drain_filter(|player|
                    player.pieces.is_empty()
                &&  !player.is_winner
            )
            .collect::<Vec<Player>>()
    }
    /// Changes the turn value to indiacate which player turn it is.
    ///
    /// Adds value to turn and changes to 0 if the value exceeds players vector len - 1.
    #[inline]
    fn next_player(&mut self) -> &mut Self {
        match self.turn < self.players.len() - 1_usize {
            true  => self.turn += 1_usize,
            false => self.turn = 0_usize,
        }
        self
    }
    /// To update player pieces position every turn.
    #[inline]
    fn update_position(&mut self, x: i32, y: i32) -> Result<&mut Self, Error> {
        // updating the piece.
        self.players[self.turn].update_piece(x, y)?; 
        // To trigger draw.
        self.set_update_true();
        Ok(self)
    }
    /// Iterates through each piece in a player and searches for a position. If that position
    /// exists then returns true else returns false.
    ///
    /// Takes `f32` x and y position values and [`binary_search`] the position in the given pieces.
    ///
    /// [`binary_search`]: slice::binary_search
    #[inline]
    fn check_piece_in_pos(&self, x: f32, y: f32) -> bool {
        !self.players
            .iter()
            .filter(|player| player.piece_index_from_xy_f32(x, y).is_ok())
            .collect::<Vec<&Player>>()
            .is_empty()
    }
    /// To know if the x and y holds a piece of another player.
    ///
    /// Takes x and y values and iterates over all the players in the games to decide which
    /// particular piece is present and removes that piece to return it.
    #[inline]
    fn remove_piece_in_pos(&mut self, x: f32, y: f32) -> Result<Option<Piece>, Error> {
        // To check if the piece is in board range.
        Piece::in_board_range(x as i32, y as i32)?;
        for player in self.players.iter_mut() {
            if let Ok(index) = player.piece_index_from_xy_f32(x, y) {
                return Ok(Some(player.kill_piece(index)?))
            }
        }
        Ok(None)
    }
}
/*-----------------------------------------------------------------------------------------------*/

impl Default for Game {
    fn default() -> Self {
        Game {
            players: Vec::new(),
            turn: usize::default(),
            update: bool::default(),
            picked: bool::default(),
            play: bool::default(),
        }
    }
}
