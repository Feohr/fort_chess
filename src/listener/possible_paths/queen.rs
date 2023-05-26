//! queen module
//!
//! To handle the queen possible paths analysis.
//!
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use fort_builders::game::Game;
use crate::listener::possible_paths::{
    PositionVectorf32, analyse_rook_paths, analyse_minister_paths,
};

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// This is the easiest paths to analyse.
///
/// Queen is essentially just the combination of rook paths and minister paths. Hence we append
/// those two.
pub(crate) fn analyse_queen_paths(x: f32, y: f32, game: &Game) -> PositionVectorf32 {
    let mut _possiblepaths: PositionVectorf32 = Vec::new();
    _possiblepaths.append(&mut analyse_rook_paths(      x, y, game));
    _possiblepaths.append(&mut analyse_minister_paths(  x, y, game));
    _possiblepaths
}
