//! tiles module
//!
//! To hold the logic that draws the board to the screen. Runs one time at the beginning of the
//! game.
//!
//! ##Contents:
//! -   TilePlugin.
//! -   TileSheet.
//! -   BREADTH.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, SPRITESIZE, TILESIZE, ZAxisLevel};
use bevy::prelude::{
    default, App, AssetServer, Assets, Commands, Entity, Handle, Name, Plugin, Res, ResMut,
    SpriteSheetBundle, StartupStage, TextureAtlas, TextureAtlasSprite, Transform, Vec2, Vec3,
};
use fort_builders::{
    board::{X_MAX, X_MIN, Y_MAX, Y_MIN},
    decrement_if_positive,
};

/// Holds the breadth size of the board.
const BREADTH: i32 = 2;

struct TileSheet(Handle<TextureAtlas>);

#[derive(PartialEq, Eq)]
enum TileSpriteSheetIndex {
    Light,
    Dark,
    Border,
    FortOuter,
    FortInner,
}

pub(crate) struct TilePlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for TilePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for TilePlugin {

    fn build(&self, app: &mut App) {
        app .add_startup_system_to_stage(StartupStage::PreStartup,  load_tile   )
            .add_startup_system_to_stage(StartupStage::Startup,     draw_board  )
            .add_startup_system_to_stage(StartupStage::Startup,     draw_border )
            .add_startup_system_to_stage(StartupStage::Startup,     draw_fort   );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████TileSpriteSheetIndex████*/
/*-----------------------------------------------------------------------------------------------*/
impl TileSpriteSheetIndex {

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

fn dark_or_light_tile_index(x: i32, y: i32) -> TileSpriteSheetIndex {

    TileSpriteSheetIndex::from_usize({
        (

            match x > 0 {
                true  => TileSpriteSheetIndex::Light,
                false => TileSpriteSheetIndex::Dark,
            }
            .as_usize()

        ) ^ (

            match (x + y) % 2 == 0 {
                true  => TileSpriteSheetIndex::Dark,
                false => TileSpriteSheetIndex::Light,
            }
            .as_usize()

        )
    })

}

/*████Drawing the Board████*/
/*-----------------------------------------------------------------------------------------------*/
/// Drawing the board.
fn draw_board(
    mut commands:   Commands,
    tile:           Res<TileSheet>,
) {

    (X_MIN..=X_MAX).for_each(|x| {

        (Y_MIN..=Y_MAX).for_each(|mut y| {

            if !{ y == 0
            ||  (
                    x.abs() <= BREADTH
                &&  y.abs() <= BREADTH
                )
            ||  (
                    x.abs() > BREADTH
                &&  y.abs() > BREADTH
            )}  {

                    // To get rid of the zeroeth line.
                    if y > 0 { y -= 1 }

                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        dark_or_light_tile_index(x, y).as_usize(),
                        Vec3::new(
                            decrement_if_positive(x) as f32 * RESOLUTION,
                            y as f32 * RESOLUTION,
                            ZAxisLevel::Second.as_f32(),
                        ),
                    );

                    commands.entity(tile).insert(Name::new("Tile"));

            }

        })

    });

}

/// Drawing the border of the board.
fn draw_border(
    mut commands:   Commands,
    tile:           Res<TileSheet>,
) {

    ((X_MIN - 1)..=(X_MAX + 1)).for_each(|x| {

        ((Y_MIN - 1)..=(Y_MAX + 1)).for_each(|mut y| {

            if !{   y == 0
            ||  (   x.abs() > BREADTH + 1
                &&  y.abs() > BREADTH + 1
            )}  {

                    // To get rid of the zeroeth line.
                    if y > 0 { y -= 1 }

                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        TileSpriteSheetIndex::Border.as_usize(),
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
fn draw_fort(
    mut commands:   Commands,
    tile:           Res<TileSheet>,
) {

    (-BREADTH..=BREADTH).for_each(|x| {

        (-BREADTH..BREADTH).for_each(|y| {

            let tile = spawn_tile(
                &mut commands,
                &tile,
                TileSpriteSheetIndex::FortOuter.as_usize(),
                Vec3::new(
                    decrement_if_positive(x)    as f32 * RESOLUTION,
                    y                           as f32 * RESOLUTION,
                    ZAxisLevel::Third.as_f32(),
                ),
            );

            commands.entity(tile).insert(Name::new("Fort Exterior"));

        })

    });

    ((-BREADTH + 1)..=(BREADTH - 1)).for_each(|x| {

        ((-BREADTH + 1)..(BREADTH - 1)).for_each(|y| {

            let tile = spawn_tile(
                &mut commands,
                &tile,
                TileSpriteSheetIndex::FortInner.as_usize(),
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
fn load_tile(
    mut commands:           Commands,
    asset:                  Res<AssetServer>,
    mut texture_atlases:    ResMut<Assets<TextureAtlas>>,
) {

    commands.insert_resource(TileSheet(texture_atlases.add(
        TextureAtlas::from_grid_with_padding(
            asset.load("spritesheet/tile_sheet.png"),
            Vec2::splat(SPRITESIZE),
            5, // Rows.
            1, // Columns.
            Vec2::splat(0.0),
            Vec2::splat(0.0),
        ),
    )));

}

/// To spawn a tile.
///
/// Index:
/// 0.  Tile dark.
/// 1.  Tile Light.
/// 2.  Tile Border.
/// 3.  Fort Exterior.
/// 4.  Fort Interior.
fn spawn_tile(
    commands:       &mut Commands,
    tile:           &TileSheet,
    index:          usize,
    translation:    Vec3,
) -> Entity {

    let width  = TILESIZE.0 * RESOLUTION;
    let height = TILESIZE.1 * RESOLUTION;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation: translation,
                ..default()
            },
            ..default()
        })
        .id()

}
/*-----------------------------------------------------------------------------------------------*/
