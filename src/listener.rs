//! listener module.
//!
//! Handles the main IO interaction by the player.
//! ## Contents:
//! -   PlayerPosition.
//! -   ListenerPlugin.
//! -   Picker.
//! -   Click.

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod possible_paths;

use crate::game::GameAsset;
use crate::{RESOLUTION, TILEDRAW, ZAxisLevel};
use bevy::{
    input::Input,
    prelude::{
        default, Color, Commands, Component, CursorMoved, Entity, EventReader, MouseButton, Query,
        Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Vec3, Windows, With, Plugin, App,
    },
};
use fort_builders::{
    board::{cursor_in_window, position_in_board_bounds},
    game::GameAction,
    player::PlayerAction,
};
use possible_paths::{
    Paths, draw_possible_piece_paths,
    clear_possible_piece_paths, update_possible_piece_paths, PossiblePaths,
};

const PICKER_COLOR: Color = Color::SILVER;
const CLICKS_COLOR: Color = Color::DARK_GRAY;

#[derive(Component)]
struct Picker;

#[derive(Component)]
struct Click;

#[derive(Component)]
pub struct PlayerPosition {
    x: f32,
    y: f32,
}

pub(crate) struct ListenerPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████ListenerPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for ListenerPlugin {

    fn build(&self, app: &mut App) {
        app .add_startup_system(    initialize_listener_objects )
                    .add_system(    update_cursor_position      )
                    .add_system(    hover_listener              )
                    .add_system(    click_listener              );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████ListenerPlugin Objects Tnitializer████*/
/*-----------------------------------------------------------------------------------------------*/
fn initialize_listener_objects(mut commands: Commands) {

    commands.insert_resource(PlayerPosition {
        x: 0.0,
        y: 0.0,
    });

    commands.insert_resource(PossiblePaths {
        paths: Vec::new(),
    });

}
/*-----------------------------------------------------------------------------------------------*/

/*████Listner Logic████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_picker(
    commands:   &mut Commands,
    pickers:    &Query<Entity, With<Picker>>,
) {

    for picker in pickers.iter() {
        commands.entity(picker).despawn();
    }

}

fn hover_listener(
    mut commands:   Commands,
    pickers:        Query<Entity, With<Picker>>,
    game:           ResMut<GameAsset>,
    cursor:         Res<PlayerPosition>,
) {

    clear_picker(&mut commands, &pickers);

    let (m_x, m_y) = (cursor.x, cursor.y);

    if !hovered_position_in_player_pieces(m_x, m_y, &game) { return }

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

fn hovered_position_in_player_pieces(
    x:      f32,
    y:      f32,
    game:   &ResMut<GameAsset>,
) -> bool {

    game.get().current_player().piece_index_from_xy_f32(x, y).is_ok()

}
/*-----------------------------------------------------------------------------------------------*/

/*████ClickListener Logic████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_click(
    commands:   &mut Commands,
    clicks:     &Query<Entity, With<Click>>,
) {

    for click in clicks.iter() {
        commands.entity(click).despawn();
    }

}

fn click_listener(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
    mut paths:      ResMut<PossiblePaths>,
    click:          Res<Input<MouseButton>>,
    clicks:         Query<Entity, With<Click>>,
    cursor:         Res<PlayerPosition>,
    paths_query:    Query<Entity, With<Paths>>,
) {

    let (m_x,   m_y ) =  (cursor.x, cursor.y);

    if !position_in_board_bounds(               m_x, m_y)  { return }
    if          !click.just_pressed(MouseButton::Left   )  { return }

    clear_click(&mut commands, &clicks);

    match game.get().picked {

        true  => {

            game.get_mut().set_picked_false();
            clear_possible_piece_paths(&mut commands, &paths_query);
            paths.clear();

            if {
                game.get().current_player().piece_index_from_xy_f32(m_x, m_y).is_err()
            } {

                let pos = game.get().current_player().current_chosen_piece_index();

                game.get_mut()                                              //  Moved for clarity
                    .update_position(m_x as i32, m_y as i32, pos).unwrap();     game.get_mut()
                    .next_player();

            }
        },

        false => {

            if let Ok(index) = {
                game.get().current_player().piece_index_from_xy_f32(m_x, m_y)
            } {

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

                game.get_mut()
                    .current_player_mut()                                   //  Moved for clarity
                    .set_current_chosen_piece(index).unwrap();                  game.get_mut()
                    .set_picked_true();

                update_possible_piece_paths(game.get(), &mut paths);
                draw_possible_piece_paths(
                    &mut commands,
                    &paths,
                    &paths_query,
                    game.get()
                );

            }

        },

    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Main Listener Function████*/
/*-----------------------------------------------------------------------------------------------*/
pub fn update_cursor_position(
    mut events:     EventReader<CursorMoved>,
    mut position:   ResMut<PlayerPosition>,
    windows:        Res<Windows>,
) {

    if let Some(window) = windows.get_primary() {
        for cursor in events.iter() {
            (position.x, position.y) = cursor_in_window(
                cursor.position.x,
                cursor.position.y,
                window.height(),
                window.width(),
            );
            break;
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Spawn Sprites████*/
/*-----------------------------------------------------------------------------------------------*/
fn spawn_square_sprite(
    commands:       &mut Commands,
    color:          Color,
    translation:    Vec3,
) -> Entity {

    let width  = TILEDRAW.0 * RESOLUTION;
    let height = TILEDRAW.1 * RESOLUTION;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()

}
/*-----------------------------------------------------------------------------------------------*/
