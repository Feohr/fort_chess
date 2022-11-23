//! highlight module.
//!
//! Handles the logic to highlight current player pieces onto the screen.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, TILESIZE, ZAxisLevel};
use bevy::prelude::{
    Entity, With, Commands, ResMut, Component, Query, SpriteBundle, Sprite, Vec3, Vec2, Transform,
    default, Color,
};
use crate::game::GameAsset;

/// Highlight color to display the current player pieces.
const HILITE_COLOR: Color = Color::rgba(0.6, 0.6, 0.6, 0.3);

/// To distinguish highlight entity.
#[derive(Component)]
pub(crate) struct Highlight;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To clear all the highlight entities from the scene.
fn clear_highlight(
    commands:   &mut Commands,
    query:      &Query<Entity, With<Highlight>>,
) {

    // Iterates over Highlight entities and despawns them.
    for blocks in query.iter() {
        commands.entity(blocks).despawn();
    }

}

/// To Draw highlight over the current player [`Piece`].
///
/// Iterating over the current active player and highlighting. The highlight size is [`TILESIZE`].
pub(crate) fn highlight_active_pieces(
    commands:   &mut Commands,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Highlight>>,
) {

    // Clean up.
    clear_highlight(commands, query);

    for piece in game.get().current_player().pieces() {

        // Spawn.
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: HILITE_COLOR,
                    custom_size: Some(Vec2::new(
                            //width.
                            TILESIZE.0          * RESOLUTION,
                            //height.
                            TILESIZE.1          * RESOLUTION,
                    )),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        //piece_pos_x.
                        piece.position.x as f32 * RESOLUTION,
                        //piece_pos_y.
                        piece.position.y as f32 * RESOLUTION,
                        // Z Level.
                        ZAxisLevel::Fifth.as_f32(),
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Highlight);

    }

}
