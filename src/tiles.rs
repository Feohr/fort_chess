use bevy::prelude::*;
use crate::TILESIZE;
use crate::fort_builders::board::*;
use bevy::text::{ Text, TextStyle, TextAlignment, Text2dBounds };
use crate::RESOLUTION;

pub struct TilePlugin;
struct TileSheet(Handle<TextureAtlas>);

// Breadth of the board.
//
// I tried playing with the value to see if the formula would scale appropriately.
// Unfortunately, it didn't. Safe to say that the formula for this board is less than perfect.
// Do not mess with the board size unless you want the game to mess up and then spend hours fixing
// it. Or maybe you do. In which case, type away.
const BREADTH: i32 = 2;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tile)
            .add_startup_system(draw_board)
            .add_startup_system(put_text);
    }
}

fn put_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(4.2 * RESOLUTION),
            },
            text:   Text::from_section(
                        "Mohammed Rehaan",
                        TextStyle {
                            font: asset_server.load("fonts/fira-sans.extrabold.ttf"),
                            font_size: 0.4 * RESOLUTION,
                            color: Color::RED,
                        },
                    )
                    .with_alignment(TextAlignment::CENTER),
            transform:  Transform {
                            translation: Vec3::new(-10.8 * RESOLUTION, -1.8 * RESOLUTION, 2.0),
                            ..Default::default()
                        },
            ..Default::default()
        }
    )
    .insert(Name::new("Player Name"));
}

// To decide whether a dark tile or a light tile should be spawned.
// The tiles on the "top" board seem to mirror. 
// Just added a stupid if-else check to flip the tiles.
// Wish I had a better way of handling this.
fn switch(x: i32, y: i32) -> usize {
    if  (
                x % 2 == 0
            &&  y % 2 == 0
        ) || (
                x % 2 != 0
            &&  y % 2 != 0
        ) {
            // HACK
            if x > 0 && y > 1 { return 0_usize }
            return 1_usize;
    }
    // HACK
    if x > 0 && y > 1 { return 1_usize }
    return 0_usize;
}

// To get rid of the extra "Zeroeth" column along the y-axis.
// Couldn't just add it to the loop.
// It's a long explaination... I had to use this cause, I suck at coding.
fn ret_x_minus_one(x: i32) -> i32 {
    if x > 0 {
        return x - 1;
    }
    return x;
}

// Using a mathematical equation to draw the board.
//
// I personally don't think it's practical. It would be too much calculation at runtime for just
// some board graphics which could've just been a drawn from a const array with a set of points for
// the tiles. Would be cleaner and easier to maintain. But, wouldn't that make my life too easy?
// It would be better practice as it reduces CPU load at runtime. But, I had to mess it up somehow.
// So I went through the painstaking process of calculating/coming-up with an equation to draw the
// board which IMO is one dumpster fire, kanye west going batshit crazy at the grammy awards shitty
// algorithm that will give you a migraine so bad that you will drill your hands into your skull
// like kisshot in Kizumonogatari II. Apologies for ranting on. It's just that I started coding at 
// 7 PM and it's 5 AM now. This god-awful idea has sucked my life force out and redered me unable 
// to think or move. Please don't be a prick and just use a pre-defined data to build stuff in your
// game rather than calculating at runtime.
fn draw_board(mut commands: Commands, tile: Res<TileSheet> ) {
    for mut x in X_MIN..=X_MAX {
        for mut y in Y_MIN..=Y_MAX {
            let (x_a, y_a) = (x.abs(), y.abs());
            if      y == 0 // Removing the extra column along the x-axis.
                ||  (
                    // Getting rid of the middle part where the fort will reside.
                        x_a <= BREADTH
                    &&  y_a <= BREADTH
                )
                    // Simple formula to build the board shape.
                    // Absolutely imperfect. If you can come up with a better formula please go
                    // ahead.
                ||  (
                        ( x_a * y_a ) 
                    -   (BREADTH * ( x_a + y_a ))
                    +   (BREADTH * BREADTH)
                ) > 0 {
                        continue;
            }
            if  y > 0 { y -= 1 }
            let tile =  spawn_tile(
                            &mut commands,
                            &tile,
                            switch(x, y),
                            Vec3::new(
                                ret_x_minus_one(x) as f32 * RESOLUTION,
                                y as f32 * RESOLUTION,
                                1.0,
                            ),
                        );
            commands.entity(tile).insert(Name::new("Tile")).id();
        }
    }
}

// Main functtion to load the tile asset into scope.
// Runs at startup before staging.
pub fn load_tile(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(
        TileSheet(  texture_atlases.add(
                        TextureAtlas::from_grid_with_padding(
                            asset.load("spritesheet/tilesheet.png"),
                            Vec2::new(125.0, 125.0),
                            2, // Rows.
                            2, // Columns.
                            Vec2::splat(1.0),
                            Vec2::splat(0.0),
                        )
                    )
        )
    );
}

// To actually set the tile size and create a sprite sheet.
fn spawn_tile(
    commands: &mut Commands,
    tile: &TileSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    commands.spawn_bundle(SpriteSheetBundle{
        sprite: TextureAtlasSprite{
                    index,
                    custom_size: Some(
                                    Vec2::new(
                                        TILESIZE.0 * RESOLUTION,
                                        TILESIZE.1 * RESOLUTION,
                                    )
                                ),
                    ..Default::default()
                },
        texture_atlas: tile.0.clone(),
        transform: Transform {
           translation: translation,
           ..Default::default()
        },
        ..Default::default()
    }).id()
}
