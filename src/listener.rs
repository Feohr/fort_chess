//! listener module.
//!
//! Handles the main IO interaction by the player.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

//------------------//
pub(crate) mod button;
mod click;
mod hover;
mod possible_paths;
//------------------//

use crate::{state::FortChessState, RESOLUTION, TILEDRAW};
use bevy::prelude::{
    default, App, Color, Commands, Component, CursorMoved, Entity, EventReader, Plugin, Res,
    ResMut, Sprite, SpriteBundle, SystemSet, Transform, Vec2, Vec3, Windows,
};
use button::FortButtonPlugin;
use click::click_listener;
use fort_builders::board::cursor_in_window;
use hover::{clear_picker, hover_listener};
use possible_paths::PossiblePaths;

/// To hold the current cursor position.
#[derive(Component)]
pub(crate) struct CursorPosition {
    x: f32,
    y: f32,
}
/// Plugin to handle the hover and click listener systems.
pub(crate) struct ListenerPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for ListenerPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for ListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(FortChessState::BoardScreen)
                .with_system(initialize_listener_objects),
        )
        .add_system_set(
            SystemSet::on_update(FortChessState::BoardScreen)
                .with_system(update_cursor_position)
                .with_system(clear_picker)
                .with_system(hover_listener)
                .with_system(click_listener),
        )
        .add_system_set(
            SystemSet::on_exit(FortChessState::BoardScreen)
                .with_system(deallocate_listener_objects),
        )
        .add_plugin(FortButtonPlugin);
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████ListenerPlugin Objects████*/
/*-----------------------------------------------------------------------------------------------*/
/// To initialize [`CursorPosition`] and [`PossiblePaths`] structs.
fn initialize_listener_objects(mut commands: Commands) {
    commands.insert_resource(CursorPosition {
        x: default(),
        y: default(),
    });
    commands.insert_resource(PossiblePaths {
        paths: Vec::default(),
    });
}

/// To remove the board resources.
fn deallocate_listener_objects(mut commands: Commands) {
    commands.remove_resource::<CursorPosition>();
    commands.remove_resource::<PossiblePaths>();
}
/*-----------------------------------------------------------------------------------------------*/

/*████Listener Function████*/
/*-----------------------------------------------------------------------------------------------*/
/// To update cursor position at each frame.
///
/// Mainly used to solve the "clicking" issue that was faced in the beginning of the project.
/// Without this, I had to "move" the cursor in order for the `click_listener` to activate and read
/// the cursor position.
pub(crate) fn update_cursor_position(
    mut events: EventReader<CursorMoved>,
    mut position: ResMut<CursorPosition>,
    windows: Res<Windows>,
) {
    let Some(window) = windows.get_primary()    else { return };
    let Some(cursor) = events.iter().next()     else { return };
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
fn spawn_square_sprite(commands: &mut Commands, color: Color, translation: Vec3) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(
                    //width.
                    TILEDRAW.0 * RESOLUTION,
                    //height.
                    TILEDRAW.1 * RESOLUTION,
                )),
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
