//! state module.
//!
//! To handle and change the game state.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum FortChessState {
    StartScreen,
    GameBuild,
    BoardScreen,
    ResultScreen,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl FortChessState {
    pub(crate) fn new() -> Self {
        FortChessState::StartScreen
    }
}
