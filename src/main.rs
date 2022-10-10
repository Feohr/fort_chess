/*
-* Date: 14th of September, 2022.
-* ==============================
-*
-* This was a game I came up with as a child. I was looking to create my own rust project and
-* wonderd if I could turn my game idea into reality. So here we are. I hope you enjoy it.
-*
-* Note: This game is open-source and free to use, re-engineered or re-used however you like.
-* You are also welcomed to contribute to this and rest of my repositories.
-*
-*      Github:     https://github.com/RehaanMohammed
-*      LinkedIn:   https://www.linkedin.com/in/mohammed-rehaan-193305222
-*      Blog:       https://www.myblog.io/aboutme/
-*      Articles:   https://www.letsgetrusty.io/authors/RehaanMohammed/
-* ====================================================================================
-*
-* Rules:
-*
-*      *   This is a mini-game made in rust called fort chess.
-*
-*      *   You need 2-4 players to play this game.
-*
-*      *   There are a total of 80 boxes. 25 in each quadrant (24 + 1 inside fort).
-*
-*      *   Three players (fort attackers) have 2 knights and 4 Pawns.
-*
-*      *   One player (fort defender) has 4 Knight, 2 Rook, 3 Minister, 3 Queen and 12 Pawns
-*          Evenly distributed across the boards.
-*
-*      *   There is no king. They are all sleeping peacefully while their soldiers
-*          bleed(realistic?).
-*
-*      *   The players cannot cross-over to the other player's boards.
-*          // except the fort defender (only the queens and ministers are allowed to cross).
-*
-*      *   The defending player defeats the attacking players by reaching
-*          the border of the attacking player's board.
-*
-*      *   The defending player loses if the attacking player reaches to the center of the fort.
-*
-* To Win:
-*      *   In order to win the game, the defender must either defeat all the players or
-*          the attacking player must reach the center of the fort and roll the dice to get 6.
-* ===============================================================================================
-*
-* Some ideas to re-impliment this code into your own project:
-*
-*      *   Make a full blown out 3d-version of this game.
-*
-*      *   Turn this game into online multiplayer.
-*
-*      *   Make this game adaptable with 2 compulsory opponents and rest as optional.
-*
-*      *   Add AI bots for opponents.
-*
-*      *   Make a terminal version of this game with only ASCII graphics.
-*
-*      *   Make this game cross-platform and port it to Android/IOS.
-*
-*  -- Mohammed Rehaan
-*/

// Getting the board presets from the resources library.
extern crate fort_builders;

use fort_builders::{ build_board, dice_roll, exit, PlayerLW };
use fort_builders::player::{ Player, PlayerAction, Team };
use fort_builders::game::{ Game, GameAction };
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Error while loading graphics ({0}).")]
    GraphicalError(String),
    #[error("Error withing the fort builder module ({0}).")]
    InternalErrorLibError(#[from] fort_builders::Error),
    // SoundError(sound_lib::Error),
}

fn display_hover_graphics() -> Result<(), Error> {
    todo!()
}

fn listen_mouse_press() -> Result<(i32, i32), Error> {
    todo!()
}

fn mousepressed() -> bool {
    todo!()
}

// Starting point of the game.
// Produces error to catch during game run.
fn run<'a>(
    player1: Player,
    player2: Player,
    // player3: Player,
    // player4: Player,
) -> Result<Option<PlayerLW<'a>>, Error> {
    let mut game = Game::init(player1, player2);
    build_board(&game)?;

    // Safety net cause I don't trust myself to write good code.
    let mut safety_net = 0;

    // To handle killed pieces.
    let mut killed_pieces = Vec::new();

    loop {
        // Display hover graphic
        display_hover_graphics()?;

        // Checks if the game is in exit state or not.
        if game.is_exit() || safety_net > 10_000 {
            break;
        }

        if mousepressed() {
            let (p_x, p_y) = listen_mouse_press()?;
            if let Some(pos) = game.players[game.turn].get_piece_pos(p_x, p_y) {
                let (x, y) = listen_mouse_press()?;
                // is x and y free?
                let piece_killed = game.check_piece_in_pos_get(x, y)?;
                if let Some(dead_piece) = piece_killed {
                    killed_pieces.push(dead_piece);
                }
                let updated = game.update(x, y, pos)?;
                if !updated {
                    continue;
                }
            }
            // Next turn
            game.next();
        }
        // hunt and kill non-playing players.
        game.hunt()?;
        safety_net += 1;
    }
    let res = exit(game)?;
    return Ok(res);
}

fn game<'a>() -> Result<Option<PlayerLW<'a>>, Error> {
    let bool = { dice_roll() % 2 == 1 };
    let player1 = Player::from("player1".to_string(), Team::Red, bool)?;
    let player2 = Player::from("player2".to_string(), Team::Green, !bool)?;
    let res = run(player1, player2)?;
    return Ok(res);
}
// Players are different colors based on their choice team.
// In future, make this semi-automated.
// When no option, the program will automatically assign a team.
fn main() {
    match game() {
        Ok(winner) => println!("The winner is: {:?}", winner),
        Err(err) => eprintln!("{:?}", err),
    }
}
