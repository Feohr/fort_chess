//! click module.
//! To handle player clicking functionality.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    despawn_entity::DespawnEntity,
    game::GameAsset,
    listener::{
        possible_paths::{
            draw_possible_piece_paths, update_possible_piece_paths, Paths, PossiblePaths,
        },
        spawn_square_sprite, CursorPosition,
    },
    ZAxisLevel, RESOLUTION,
};
use bevy::{
    input::Input,
    prelude::{Color, Commands, Component, Entity, MouseButton, Query, Res, ResMut, Vec3, With},
};
use fort_builders::{board::position_in_board_bounds, game::GameAction, player::PlayerAction};

/// Displays the clicked piece color.
const CLICKS_COLOR: Color = Color::DARK_GRAY;

/// Click component to recognise [`Click`] entities.
#[derive(Component)]
pub(crate) struct Click;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/
/// To listen for clicks and display a dark grey block where the cursor was clicked.
///
/// Capturing the cursor position and checking if the mouse is within the board bounds. Only
/// then do we start checking for the accurate position inside the player pieces. Doesn't
/// proceed if left mouse button is not not clicked.
pub(crate) fn click_listener(
    mut commands: Commands,
    mut game: ResMut<GameAsset>,
    mut paths: ResMut<PossiblePaths>,
    click: Res<Input<MouseButton>>,
    clicks: Query<Entity, With<Click>>,
    cursor: Res<CursorPosition>,
    paths_query: Query<Entity, With<Paths>>,
) {
    let (m_x, m_y) = (cursor.x, cursor.y);
    if !position_in_board_bounds(m_x, m_y) || !click.just_pressed(MouseButton::Left) {
        return;
    }
    let game = game.get_mut();
    commands.despawn_entity(&clicks);
    match game.picked {
        true => {
            if paths.contains(m_x, m_y) {
                let _killed_piece = game.remove_piece_in_pos(m_x, m_y).unwrap();
                game.update_position(m_x as i32, m_y as i32)
                    .unwrap()
                    .next_player();
            }
            game.set_picked_false();
            commands.despawn_entity(&paths_query);
            paths.clear();
        }
        false => {
            let Ok(index) = game.current_player().piece_index_from_xy_f32(m_x, m_y) else {
                return
            };
            let click = spawn_square_sprite(
                &mut commands,
                CLICKS_COLOR,
                Vec3::new(
                    m_x * RESOLUTION,
                    m_y * RESOLUTION,
                    ZAxisLevel::Seventh.as_f32(),
                ),
            );
            commands.entity(click).insert(Click);
            game.current_player_mut().set_chosen_piece_index(index);
            game.set_picked_true();
            update_possible_piece_paths(game, &mut paths);
            draw_possible_piece_paths(&mut commands, &paths, &paths_query, game);
        }
    }
}
