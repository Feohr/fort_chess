//! hover module.
//!
//! Handles the player mouse hovering functionality.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, ZAxisLevel,
    listener::{CursorPosition,spawn_square_sprite},
    game::GameAsset,
    despawn_entity::DespawnEntity,
};
use bevy::prelude::{Color, Commands, Component, Entity, Query, Res, ResMut, Vec3, With};
use fort_builders::{
    board::position_in_board_bounds,
    player::PlayerAction,
};

/// Displays the hover color.
const PICKER_COLOR: Color = Color::SILVER;

/// Picker component to recognise [`Picker`] enities.
#[derive(Component)]
pub(crate) struct Picker;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Picker Despawn████*/
/*-----------------------------------------------------------------------------------------------*/
/// Clear picker function to cleanup [`Picker`] entities.
pub(crate) fn clear_picker(
    mut commands:   Commands,
    pickers:        Query<Entity, With<Picker>>,
) {
    // Iterate over all the entities that have the Picker component and despawn them.
    commands.despawn_entity(&pickers);
}
/*-----------------------------------------------------------------------------------------------*/

/*████Hover Listener████*/
/*-----------------------------------------------------------------------------------------------*/
/// To check wether a given hovered position is inside player pieces.
///
/// Return a bool value that is checked at each [`CursorMoved`] event. If the cursor position is
/// not a piece position then there won't be a light grey block displayed.
#[inline]
fn hovered_position_in_player_pieces(
    x:      f32,
    y:      f32,
    game:   &ResMut<GameAsset>,
) -> bool {
    game.get().current_player().piece_index_from_xy_f32(x, y).is_ok()
}

/// To display a light gray block over the piece where the mouse is hovering.
///
/// Early return if not in player pieces or inside board bounds.
pub(crate) fn hover_listener(
    mut commands:   Commands,
    game:           ResMut<GameAsset>,
    cursor:         Res<CursorPosition>,
) {
    // Checking for early return.
    let (m_x, m_y) = (cursor.x, cursor.y);
    if  !position_in_board_bounds(m_x, m_y)
    ||  !hovered_position_in_player_pieces(m_x, m_y, &game)  { return }
    // Creating a hover tile.
    let hover = spawn_square_sprite(
        &mut commands,
        PICKER_COLOR,
        Vec3::new(
            m_x * RESOLUTION,
            m_y * RESOLUTION,
            ZAxisLevel::Sixth.as_f32(),
        ),
    );
    commands.entity(hover).insert(Picker);
}
/*-----------------------------------------------------------------------------------------------*/
