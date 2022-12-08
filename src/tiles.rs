//! tiles module
//!
//! To hold the logic that draws the board to the screen. Runs one time at the beginning of the
//! game.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

//  Modules //
/*----------*/
mod block;
/*----------*/

use crate::{
    RESOLUTION, SPRITESIZE, TILESIZE, ZAxisLevel,
    state::FortChessState,
};
use bevy::prelude::{
    default, App, AssetServer, Assets, Commands, Entity, Handle, Name, Plugin, Res, ResMut,
    SpriteSheetBundle, StartupStage, TextureAtlas, TextureAtlasSprite, Transform, Vec2, Vec3,
    SystemSet,
};
use fort_builders::{
    BREADTH,
    board::{X_MAX, X_MIN, Y_MAX, Y_MIN},
    decrement_if_positive,
};
use block::FortBlockPlugin;

/// To hold the row size of the tile pieces.
const TILE_TYPE_ROW : usize   = 5_usize;
/// To hold the col size of the tile pieces.
const TILE_TYPE_COL : usize   = 1_usize;

/// Struct to hold the tile texture atlas.
struct TileSheet(Handle<TextureAtlas>);

/// Plugin to handle board drawing systems.
pub(crate) struct TilePlugin;

/// To denote the type of tile.
#[derive(PartialEq, Eq)]
enum TileSpriteSheetIndex {
    /// Light color tile inside the board.
    Light,
    /// Dark color tile inside the board.
    Dark,
    /// The board border.
    Border,
    /// The outer shell of the middle of the board.
    FortOuter,
    /// The inner most part of the middle of the board.
    FortInner,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for TilePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for TilePlugin {

    /// [`Plugin`] implementation for [`TilePlugin`].
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_tile)
            .add_system_set(
                SystemSet::on_enter(FortChessState::BoardScreen)
                .with_system(draw_board )
                .with_system(draw_border)
                .with_system(draw_fort  )
            )
           .add_plugin(FortBlockPlugin);
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████TileSpriteSheetIndex████*/
/*-----------------------------------------------------------------------------------------------*/
impl TileSpriteSheetIndex {

    /// Returns the corresponding [`TileSpriteSheetIndex`] variant from usize.
    #[inline]
    fn from_usize(from: usize) -> Self {
        match from {
            0_usize => TileSpriteSheetIndex::Light,
            1_usize => TileSpriteSheetIndex::Dark,
            2_usize => TileSpriteSheetIndex::Border,
            3_usize => TileSpriteSheetIndex::FortOuter,
            4_usize => TileSpriteSheetIndex::FortInner,
            _       => panic!("TileSpriteSheetIndex cannot have index greater than 4."),
        }
    }

    /// Converts a given [`TileSpriteSheetIndex`] variant to corresponding usize value.
    #[inline]
    fn as_usize(&self) -> usize {
        match self {
            TileSpriteSheetIndex::Light     => 0_usize,
            TileSpriteSheetIndex::Dark      => 1_usize,
            TileSpriteSheetIndex::Border    => 2_usize,
            TileSpriteSheetIndex::FortOuter => 3_usize,
            TileSpriteSheetIndex::FortInner => 4_usize,
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/// To decide if a position should have a dark or a light tile. Used to alternate tiles for a
/// chess pattern.
///
/// Returns [`TileSpriteSheetIndex::Dark`] or [`TileSpriteSheetIndex::Light`] tile depending on the
/// tiles. Returns Dark for even tiles and Light for odd tiles until the x value is less than zero.
/// After `x > 0`, the tiles are then switched with Light for even and Dark for odd tiles.
fn dark_or_light_tile_index(x: i32, y: i32) -> TileSpriteSheetIndex {
   TileSpriteSheetIndex::from_usize({
        (
            match x > 0_i32 {
                true  => TileSpriteSheetIndex::Light,
                false => TileSpriteSheetIndex::Dark,
            }
            .as_usize()
        ) ^ (
            match (x + y) % 2_i32 == 0_i32 {
                true  => TileSpriteSheetIndex::Dark,
                false => TileSpriteSheetIndex::Light,
            }
            .as_usize()
        )
    })
}

/*████Drawing the Board████*/
/*-----------------------------------------------------------------------------------------------*/

/// To Draw the board.
///
/// Iterates from [`X_MIN`] to [`X_MAX`] and from [`Y_MIN`] to [`Y_MAX`] as it generates the board
///
/// with the power of mathematics. The one issue I do have is with the *Zeroeth* axis as that
/// produced odd number of tiles at each axis.
// I chose to decrement x and y values if they are greater than zero. Not the most elegant solution
// and far from being the best one as apparent from the trouble I go through everytime I refactor.
fn draw_board(
    mut commands:   Commands,
    tile:           Res<TileSheet>,
) {

    (X_MIN..=X_MAX).for_each(|x| {
        (Y_MIN..=Y_MAX).for_each(|mut y| {
            // If x value as well as y value are less than BREADTH value then it won't be drawn
            // as that is the center of the board where the fort will reside.
            // if x value as well as y value is greater than BREADTH, then a tile won't be
            // drawn as that will come outside the board bounds.
            // Y == 0 column is ignored to remove the extra zeroeth column.
            if !{   y == 0_i32
            ||  (
                    x.abs() <=  BREADTH
                &&  y.abs() <=  BREADTH
                )
            ||  (
                    x.abs() >   BREADTH
                &&  y.abs() >   BREADTH
            )}  {
                    // To get rid of the zeroeth line.
                    if y > 0_i32 { y -= 1_i32 }
                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        dark_or_light_tile_index(x, y),
                        Vec3::new(
                            decrement_if_positive(x)    as f32 * RESOLUTION,
                            y                           as f32 * RESOLUTION,
                            ZAxisLevel::Second.as_f32(),
                        ),
                    );
                    commands.entity(tile).insert(Name::new("Tile"));
            }
        })
    });

}

/// To Draw the border of the board.
///
/// This is essentially just the same as *draw_board* but with bounds incremented to form a border.
fn draw_border(
    mut commands:   Commands,
    tile:           Res<TileSheet>,
) {

    ((X_MIN - 1)..=(X_MAX + 1)).for_each(|x| {
        ((Y_MIN - 1)..=(Y_MAX + 1)).for_each(|mut y| {
            // Exactly the same as draw_board fucntion but with one column and row extra padding
            // for the border.
            if !{   y == 0_i32
            ||  (
                    x.abs() <=  BREADTH
                &&  y.abs() <=  BREADTH
                )
            ||  (
                    x.abs() >   BREADTH + 1_i32
                &&  y.abs() >   BREADTH + 1_i32
            )}  {
                    // To get rid of the zeroeth line.
                    if y > 0_i32 { y -= 1_i32 }
                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        TileSpriteSheetIndex::Border,
                        Vec3::new(
                            decrement_if_positive(x)    as f32 * RESOLUTION,
                            y                           as f32 * RESOLUTION,
                            ZAxisLevel::First.as_f32(),
                        ),
                    );
                    commands.entity(tile).insert(Name::new("Border"));
            }
        })
    });

}

/// To draw the fort.
///
/// Follows the similar logic as other "drawing" functions.
fn draw_fort(
    mut commands:   Commands,
    tile_sheet:     Res<TileSheet>,
) {

    // Draws the fort in the BREADTH side square.
    (-BREADTH..=BREADTH).for_each(|x| {
        (-BREADTH..BREADTH).for_each(|y| {
            let tile = spawn_tile(
                &mut commands,
                &tile_sheet,
                TileSpriteSheetIndex::FortOuter,
                Vec3::new(
                    decrement_if_positive(x)    as f32 * RESOLUTION,
                    y                           as f32 * RESOLUTION,
                    ZAxisLevel::Third.as_f32(),
                ),
            );
            commands.entity(tile).insert(Name::new("Fort Exterior"));
        })
    });

    // Draws the middle most part which is BREADTH - 1 size square.
    ((-BREADTH + 1_i32)..=(BREADTH - 1_i32)).for_each(|x| {
        ((-BREADTH + 1_i32)..(BREADTH - 1_i32)).for_each(|y| {
            let tile = spawn_tile(
                &mut commands,
                &tile_sheet,
                TileSpriteSheetIndex::FortInner,
                Vec3::new(
                    decrement_if_positive(x)    as f32 * RESOLUTION,
                    y                           as f32 * RESOLUTION,
                    ZAxisLevel::Fourth.as_f32(),
                ),
            );
            commands.entity(tile).insert(Name::new("Fort Interior"));
        })
    });

}
/*-----------------------------------------------------------------------------------------------*/

/*████Drawing the Board████*/
/*-----------------------------------------------------------------------------------------------*/
/// To load the tile asset.
///
/// To load the tile resource `.png` from assets folder. This folder needs to exist with the
/// executable binary otherwise the game won't have the asset.
fn load_tile(
    mut commands:           Commands,
    asset:                  Res<AssetServer>,
    mut texture_atlases:    ResMut<Assets<TextureAtlas>>,
) {

   commands.insert_resource(TileSheet(texture_atlases.add(
        TextureAtlas::from_grid_with_padding(
            asset.load("spritesheet/tile_sheet.png"),
            Vec2::splat(SPRITESIZE),
            TILE_TYPE_ROW, // Rows.
            TILE_TYPE_COL, // Columns.
            Vec2::splat(0_f32),
            Vec2::splat(0_f32),
        ),
    )));

}

/// To spawn a tile.
///
/// ### Index:
/// 0.  Tile dark.
/// 1.  Tile Light.
/// 2.  Tile Border.
/// 3.  Fort Exterior.
/// 4.  Fort Interior.
///
/// This is a helper function to spawn a tile from the given input.
fn spawn_tile(
    commands:       &mut Commands,
    tile:           &TileSheet,
    index:          TileSpriteSheetIndex,
    translation:    Vec3,
) -> Entity {

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: index.as_usize(),
                custom_size: Some(Vec2::new(
                        // width.
                        TILESIZE.0 * RESOLUTION,
                        // height.
                        TILESIZE.1 * RESOLUTION,
                )),
                ..default()
            },
            // Creates a copy of the texture everytime a tile is created.
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()

}
/*-----------------------------------------------------------------------------------------------*/
