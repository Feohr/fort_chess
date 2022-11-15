//! listener module.
//!
//! Handles the main IO interaction by the player.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

/// Module to handle possible piece paths logic.
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

/// Displays the hover color.
const PICKER_COLOR: Color = Color::SILVER;
/// Displays the clicked piece color.
const CLICKS_COLOR: Color = Color::DARK_GRAY;

/// Picker component to recognise [`Picker`] enities.
#[derive(Component)]
struct Picker;

/// Click component to recognise [`Click`] entities.
#[derive(Component)]
struct Click;

/// To hold the current cursor position.
#[derive(Component)]
pub(crate) struct CursorPosition {
    x: f32,
    y: f32,
}

/// Plugin to handle the hover and click listener systems.
pub(crate) struct ListenerPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████ListenerPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for ListenerPlugin {

    /// [`Plugin`] implementation for [`ListenerPlugin`].
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
/// To initialize [`CursorPosition`] and [`PossiblePaths`] structs.
fn initialize_listener_objects(mut commands: Commands) {

    // To create and insert the CursorPosition resource.
    commands.insert_resource(CursorPosition {
        x: default(),
        y: default(),
    });

    // To create an insert the PossiblePaths resource.
    commands.insert_resource(PossiblePaths {
        paths: Vec::default(),
    });

}
/*-----------------------------------------------------------------------------------------------*/

/*████Listner Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// Clear picker function to cleanup [`Picker`] entities.
fn clear_picker(
    commands:   &mut Commands,
    pickers:    &Query<Entity, With<Picker>>,
) {

    // Iterate over all the entities that have the Picker component and despawn them.
    for picker in pickers.iter() {
        commands.entity(picker).despawn();
    }

}

/// To check wether a given hovered position is inside player pieces.
///
/// Return a bool value that is checked at each [`CursorMoved`] event. If the cursor position is
/// not a piece position then there won't be a light grey block displayed.
fn hovered_position_in_player_pieces(
    x:      f32,
    y:      f32,
    game:   &ResMut<GameAsset>,
) -> bool {

    // Does a binary search to find the piece.
    game.get().current_player().piece_index_from_xy_f32(x, y).is_ok()

}

/// To display a light gray block over the piece where the mouse is hovering.
///
/// Early return if not in player pieces or inside board bounds.
fn hover_listener(
    mut commands:   Commands,
    pickers:        Query<Entity, With<Picker>>,
    game:           ResMut<GameAsset>,
    cursor:         Res<CursorPosition>,
) {

    // Clean up.
    clear_picker(&mut commands, &pickers);

    let (m_x, m_y) = (cursor.x, cursor.y);
    if !position_in_board_bounds(m_x, m_y)                  { return }
    if !hovered_position_in_player_pieces(m_x, m_y, &game)  { return }

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

    // Spawn.
    commands.entity(hover).insert(Picker);

}
/*-----------------------------------------------------------------------------------------------*/

/*████ClickListener Logic████*/
/*-----------------------------------------------------------------------------------------------*/
/// To clear [`Click`] enitities.
fn clear_click(
    commands:   &mut Commands,
    clicks:     &Query<Entity, With<Click>>,
) {

    // Iterate over all the entities that have the Click component and despawn them.
    for click in clicks.iter() {
        commands.entity(click).despawn();
    }

}

/// To listen for clicks and display a dark grey block where the cursor was clicked.
///
/// Capturing the cursor position and checking if the mouse is within the board bounds. Only
/// then do we start checking for the accurate position inside the player pieces. Doesn't
/// proceed if left mouse button is not not clicked.
fn click_listener(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
    mut paths:      ResMut<PossiblePaths>,
    click:          Res<Input<MouseButton>>,
    clicks:         Query<Entity, With<Click>>,
    cursor:         Res<CursorPosition>,
    paths_query:    Query<Entity, With<Paths>>,
) {

    let (m_x, m_y) = (cursor.x, cursor.y);
    if !position_in_board_bounds(m_x, m_y)
    || !click.just_pressed(MouseButton::Left) { return }

    // Clean up.
    clear_click(&mut commands, &clicks);

    match game.get().picked {

        // If a piece is already picked.
        true  => {

            game.get_mut().set_picked_false();

            if paths.contains(m_x, m_y) {

                let pos = game.get().current_player().current_chosen_piece_index();

                game.get_mut()                                              //  Moved for clarity
                    .update_position(m_x as i32, m_y as i32, pos).unwrap();     game.get_mut()
                    .next_player();

            }

            // Clean up.
            clear_possible_piece_paths(&mut commands, &paths_query);
            paths.clear();

        },

        // If piece not picked.
        false => {

            // if a piece is inside the player pieces then process else do nothing 
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

                // Spawn.
                commands.entity(click).insert(Click);

                // Setting game current chosen piece for reference as well as setting picked as
                // true.
                game.get_mut()
                    .current_player_mut()                                   //  Moved for clarity
                    .set_current_chosen_piece(index).unwrap();                  game.get_mut()
                    .set_picked_true();

                // Update the possible paths.
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
/// To update cursor position at each frame.
///
/// Mainly used to solve the "clicking" issue that was faced in the beginning of the project.
/// Without this, I had to "move" the cursor in order for the `click_listener` to activate and read
/// the cursor position.
pub(crate) fn update_cursor_position(
    mut events:     EventReader<CursorMoved>,
    mut position:   ResMut<CursorPosition>,
    windows:        Res<Windows>,
) {

    // Does not read when the window is not active.
    let Some(window) = windows.get_primary()    else { return };
    // Updating cursor position at each frame.
    let Some(cursor) = events.iter().next()     else { return };

    // Updating the position struct.
    (position.x, position.y) = cursor_in_window(
        cursor.position.x,
        cursor.position.y,
        window.height(),
        window.width(),
    );

}
/*-----------------------------------------------------------------------------------------------*/

/*████Spawn Sprites████*/
/*-----------------------------------------------------------------------------------------------*/
/// To spawn a square [`TILEDRAW`] size block.
fn spawn_square_sprite(
    commands:       &mut Commands,
    color:          Color,
    translation:    Vec3,
) -> Entity {

    // Spawning a TILEDRAW size sprite when called.
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
