//! draw_pieces module.
//!
//! Handles the logic to draw pieces onto the screen.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, TILESIZE, ZAxisLevel,
    despawn_entity::DespawnEntity,
};

use bevy::prelude::{
    Entity, With, Commands, Res, ResMut, TextureAtlasSprite, SpriteSheetBundle, Component, Query,
    Vec3, Vec2, Transform, Name, default,
};
use crate::game::GameAsset;
use crate::game::PlayerSheet;

/// The width of the pieces sprite sheet.
const PIECES_SPRITESHEET_WIDTH: usize = 5_usize;

/// To distinguish piece entity.
#[derive(Component)]
pub(crate) struct Piece;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Draw Piece████*/
/*-----------------------------------------------------------------------------------------------*/
/// call to draw the player [`Piece`]s.
///
/// Iterating over each player and drawing all the pieces once again. *row* and *col* correspond
/// to the player sheet resource. Hence each position along the columns correspond to the piece
/// type which is added to offset to it. The team corresponds to the rows and it is multiplied
/// with the spritesheet width to jump between the rows. The constant PIECE_SPRITESHEET_WIDTH is
/// nothing but the number of chess piece types i.e. 5.
pub(crate) fn draw_pieces(
    commands:   &mut Commands,
    sprite:     &Res<PlayerSheet>,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Piece>>,
) {

    // Clean up.
    commands.despawn_entity(query);

    game
        .get().players
        .iter()
        .for_each(|player| {
            player.pieces.iter().for_each(|piece| {
                let sprite = spawn_piece(
                    commands,
                    sprite,
                    (
                            // Row.
                            player.team.as_usize()  * PIECES_SPRITESHEET_WIDTH
                            // Column.
                    )   +   piece.piece_type.as_usize(),
                    Vec3::new(
                        piece.position.x as f32     * RESOLUTION,
                        piece.position.y as f32     * RESOLUTION,
                        ZAxisLevel::Eight.as_f32(),
                    ),
                );
                commands.entity(sprite).insert(Name::from("Piece")).insert(Piece);
            })
    });

}
/*-----------------------------------------------------------------------------------------------*/

/// Simple helper function to spawn [`Piece`] sprites. Sprite size is [`TILESIZE`].
fn spawn_piece(
    commands:       &mut Commands,
    tile:           &PlayerSheet,
    index:          usize,
    translation:    Vec3,
) -> Entity {

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(
                        // width.
                        TILESIZE.0 * RESOLUTION,
                        // height.
                        TILESIZE.1 * RESOLUTION,
                )),
                ..default()
            },
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()

}
