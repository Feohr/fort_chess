//! highlight module.
//!
//! Handles the logic to highlight current player pieces onto the screen.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, TILESIZE, ZAxisLevel,
    despawn_entity::DespawnEntity,
};
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
/// To Draw highlight over the current player [`Piece`].
///
/// Iterating over the current active player and highlighting. The highlight size is [`TILESIZE`].
pub(crate) fn highlight_active_pieces(
    commands:   &mut Commands,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Highlight>>,
) {
    // Clean up.
    commands.despawn_entity(query);
    // Draw.
    game
        .get()
        .current_player()
        .pieces()
        .iter()
        .for_each(|piece| {
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
                            piece.position.x as f32 * RESOLUTION,
                            piece.position.y as f32 * RESOLUTION,
                            ZAxisLevel::Fifth.as_f32(),
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Highlight);
        })
}
