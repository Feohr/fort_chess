use bevy::prelude::*;
use crate::{RESOLUTION, TILESIZE};
use fort_builders::{
    board::Quadrant,
    dice_roll,
    game::Game,
    player::{Player, Team},
};

const PLAYERS: usize = 4;

#[derive(Debug, Component)]
pub struct GameAsset(Game);

struct PlayerSheet(Handle<TextureAtlas>);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, init_game)
            .add_startup_system(load_sprite)
            .add_system(draw_pieces);
    } 
}

/// Function to map quadrants to the player correctly ignoring the defender quadrant.
///
/// The whole recurring theme seems to be that I am not satisfied by the logic/implementation of
/// the function as a whole. I really wish I could come up with a more clever way of working around
/// this problem.
fn calcq(i: usize, roll: usize) -> usize {
    match i {
        i if i == roll => roll,
        i if i < roll  => i,
        i if i > roll  => (i - 1) % 3,
        _ => panic!("Unexpected error when matching i and roll {i}, {roll}."),
    }
}

fn init_game(mut commands: Commands) {
    let roll = dbg!(dice_roll() % 3);
    let players = (0..PLAYERS)
        .into_iter()
        .map(|i| {
            Player::from(
                format!("player {}", i + 1),
                Team::from_index(i).unwrap(),
                roll == i,
                Quadrant::from_index(dbg!(calcq(i, roll))).unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<Player>>();
    commands.insert_resource(GameAsset(dbg!(Game::init(players))));
}

fn draw_pieces(
    mut commands: Commands,
    sprite: Res<PlayerSheet>,
    game: Res<GameAsset>,
) {
    game.0.players
        .iter()
        .for_each(|player| {
            player.pieces
                .iter()
                .for_each(|piece| {
                    let sprite = spawn_piece(
                        &mut commands,
                        &sprite,
                        (player.team.to_index() * 5) + piece.piece_type.to_index(),
                        Vec3::new(
                            piece.position.x as f32 * RESOLUTION,
                            piece.position.y as f32 * RESOLUTION,
                            6.0,
                        ),
                    );
                    commands.entity(sprite).insert(Name::from("Piece"));
                })
        })
}

fn load_sprite(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(PlayerSheet(texture_atlases.add(
        TextureAtlas::from_grid_with_padding(
            asset.load("spritesheet/chess_pieces_sheet.png"),
            Vec2::new(125.0, 125.0),
            5, // Rows.
            5, // Columns.
            Vec2::splat(1.0),
            Vec2::splat(0.0),
        ),
    )));
}

fn spawn_piece(
    commands: &mut Commands,
    tile: &PlayerSheet,
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
