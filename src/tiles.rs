use crate::fort_builders::board::*;
use crate::RESOLUTION;
use crate::TILESIZE;
use bevy::prelude::*;
use bevy::text::{Text, Text2dBounds, TextAlignment, TextStyle};

pub struct TilePlugin;
struct TileSheet(Handle<TextureAtlas>);

/// Holds the breadth size of the board.
// Breadth of the board.
//
// I tried playing with the value to see if the formula would scale appropriately.
// Unfortunately, it didn't. Safe to say that the formula for this board is less than perfect.
// Do not mess with the board size as it does not scale well.
const BREADTH: i32 = 2;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tile)
            .add_system(draw_board)
            .add_system(put_text)
            .add_system(draw_border)
            .add_system(draw_fort);
    }
}

/// The text on the side of the boards displaying player's names. In future this system would be
/// it's own Plugin.
// To spawn text box into the game. Holds names of the player. Can be a maximum of 4. With
// different colors.
fn put_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(3.5 * RESOLUTION),
            },
            text: Text::from_section(
                "Mohammed Rehaan",
                TextStyle {
                    font: asset_server.load("fonts/fira-sans.regular.ttf"),
                    font_size: 0.5 * RESOLUTION,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(-11.25 * RESOLUTION, -0.5 * RESOLUTION, 5.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player Name"));
}

// To decide whether a dark tile or a light tile should be spawned.
fn switch(x: i32, y: i32) -> usize {
    let flip = if x > 0 { 0_usize } else { 1_usize };
    if (x + y) % 2 == 0 {
        return 1_usize ^ flip;
    }
    return 0_usize ^ flip;
}

// To get rid of the extra "Zeroeth" column along the y-axis.
// Couldn't just add it to the loop.
// It's a long explaination... I had to use this cause, I suck at coding.
fn ret_x_minus_one(x: i32) -> i32 {
    match x > 0 {
        true => x - 1,
        false => x,
    }
}

/// Drawing the board.
// Just simple looping and condtion checking to draw the board.
// I personally am not satisfied by the logic. I feel like there should be an easier way to do
// this. Something a lot more cleaner and pretty.
fn draw_board(mut commands: Commands, tile: Res<TileSheet>) {
    (X_MIN..=X_MAX).for_each(|x| {
        (Y_MIN..=Y_MAX).for_each(|mut y| {
            match y == 0 // Removing the extra column along the x-axis.
            // Getting rid of the middle part where the fort will reside.
            ||  (
                    x.abs() <= BREADTH
                &&  y.abs() <= BREADTH
            )
            // Simple formula to build the board shape.
            // It is absolutely imperfect.
            // If you can come up with a better formula please go ahead.
            ||  (
                    x.abs() > BREADTH
                &&  y.abs() > BREADTH
            ) {
                true => (),
                false => {
                    if y > 0 {
                        y -= 1
                    }
                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        switch(x, y),
                        Vec3::new(
                            ret_x_minus_one(x) as f32 * RESOLUTION,
                            y as f32 * RESOLUTION,
                            2.0,
                        ),
                    );
                    commands.entity(tile).insert(Name::new("Tile"));
                }
            }
        })
    });
}

/// Drawing the border of the board.
// The algorithm to draw the borders.
//
// I decided to go more or less the same way as the rest of the board cause, why not?
// Just iterates through the min and max of the x and y. Adds +1 on all sides so that the border
// sticks out. Again, it's not the cleanest algorithm but it wil work. Wouldn't recommend scaling
// the sizes to see if the algorithm works.
fn draw_border(mut commands: Commands, tile: Res<TileSheet>) {
    ((X_MIN - 1)..=(X_MAX + 1)).for_each(|x| {
        ((Y_MIN - 1)..=(Y_MAX + 1)).for_each(|mut y| {
            match y == 0 // Removing the extra Zeroeth column along the Y-axis.
            ||  (
                    x.abs() > BREADTH + 1
                &&  y.abs() > BREADTH + 1
            ) {
                true => (),
                false => {
                    // To get rid of the zeroeth line.
                    if y > 0 {
                        y -= 1
                    }
                    let tile = spawn_tile(
                        &mut commands,
                        &tile,
                        2,
                        Vec3::new(
                            ret_x_minus_one(x) as f32 * RESOLUTION,
                            y as f32 * RESOLUTION,
                            1.0,
                        ),
                    );
                    commands.entity(tile).insert(Name::new("Border"));
                }
            }
        })
    });
}

/// To draw the fort.
// Simple funcion to draw the fort. Nothing special just iterating over a loop and drawing sprites.
fn draw_fort(mut commands: Commands, tile: Res<TileSheet>) {
    // To draw the outer fort tiles.
    (-BREADTH..=BREADTH).for_each(|x| {
        (-BREADTH..BREADTH).for_each(|y| {
            let tile = spawn_tile(
                &mut commands,
                &tile,
                3,
                Vec3::new(
                    ret_x_minus_one(x) as f32 * RESOLUTION,
                    y as f32 * RESOLUTION,
                    3.0,
                ),
            );
            commands.entity(tile).insert(Name::new("Fort Exterior"));
        })
    });
    // To draw the inner fort.
    ((-BREADTH + 1)..=(BREADTH - 1)).for_each(|x| {
        ((-BREADTH + 1)..(BREADTH - 1)).for_each(|y| {
            let tile = spawn_tile(
                &mut commands,
                &tile,
                4,
                Vec3::new(
                    ret_x_minus_one(x) as f32 * RESOLUTION,
                    y as f32 * RESOLUTION,
                    4.0,
                ),
            );
            commands.entity(tile).insert(Name::new("Fort Interior"));
        })
    });
}

/// To load the tile asset.
// Main functtion to load the tile asset into scope.
// Runs at startup before staging.
pub fn load_tile(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(TileSheet(texture_atlases.add(
        TextureAtlas::from_grid_with_padding(
            asset.load("spritesheet/tile_sheet.png"),
            Vec2::new(125.0, 125.0),
            5, // Rows.
            1, // Columns.
            Vec2::splat(1.0),
            Vec2::splat(0.0),
        ),
    )));
}

/// To spawn a tile.
///
/// Index:
/// 0.  Tile dark
/// 1.  Tile Light
/// 2.  Tile Border
/// 3.  Fort Exterior
/// 4.  Fort Interior
// To actually set the tile size and create a sprite sheet. Spawns specific tile that
// corresponds to the index.
fn spawn_tile(
    commands: &mut Commands,
    tile: &TileSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(TILESIZE.0 * RESOLUTION, TILESIZE.1 * RESOLUTION)),
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
