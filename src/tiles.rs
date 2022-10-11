use bevy::prelude::*;
use crate::TILESIZE;
use crate::fort_builders::board::BOARD;
use bevy::text::{ Text, TextStyle, TextAlignment };
use crate::RESOLUTION;

pub struct TilePlugin;
struct TileSheet(Handle<TextureAtlas>);

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tile)
            .add_startup_system(draw_board)
            .add_startup_system(put_text);
    }
}

fn put_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/fira-sans.extrabold.ttf");
    let text_box = Text2dBundle {
        text: Text::from_section(
                        "Player 1",
                        TextStyle {
                            font,
                            font_size: 0.5 * RESOLUTION,
                            color: Color::GOLD,
                        },
                    ).with_alignment(TextAlignment::BOTTOM_LEFT),
        transform: Transform {
            translation: Vec3::new(-8.5 * RESOLUTION, 1.5 * RESOLUTION, 1.0),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(text_box).insert(Name::new("Player Name"));
}

fn switch(x: i32, y: i32) -> usize {
    if  (
                x % 2 == 0
            &&  y % 2 == 0
        ) || (
                x % 2 != 0
            &&  y % 2 != 0
        ) {
            // Dark tiles
            // Can be 0, 2, 4
            let val = x.abs() - y.abs();
            return ((val.abs() % 6) % 4) as usize;
    }
    // Light Tile
    // Can be 1, 3, 5
    return ((x.abs() - y.abs()).abs() % 6)  as usize;
}

fn draw_board(mut commands: Commands, tile: Res<TileSheet> ) {
    for (mut x, mut y) in BOARD {
        // To fill the Zero position gap in the board.
        if x > 0 { x -= 1 }
        if y > 0 { y -= 1 }
        // Spawining the tile.
        let tile    =   spawn_tile(
                            &mut commands,
                            &tile,
                            switch(x, y),
                            Vec3::new(x as f32 * RESOLUTION, y as f32 * RESOLUTION, 1.0),
                        );
        commands.entity(tile).insert(Name::new("Tile")).id();
    }
}

pub fn load_tile(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile: Handle<Image> = asset.load("spritesheet/tile_sheet.png");
    let atlas = TextureAtlas::from_grid_with_padding(
                    tile,
                    Vec2::new(125.0, 125.0),
                    6,
                    1,
                    Vec2::splat(1.0),
                    Vec2::splat(0.0),
                );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(TileSheet(atlas_handle));
}


fn spawn_tile(
    commands: &mut Commands,
    tile: &TileSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut tile_sprite = TextureAtlasSprite::new(index);
    tile_sprite.custom_size = Some(Vec2::new(TILESIZE.0 * RESOLUTION, TILESIZE.1 * RESOLUTION));

    commands.spawn_bundle(SpriteSheetBundle{
        sprite: tile_sprite,
        texture_atlas: tile.0.clone(),
        transform: Transform {
           translation: translation,
           ..Default::default()
        },
        ..Default::default()
    }).id()
}
