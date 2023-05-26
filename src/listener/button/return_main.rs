//! return main module
//!
//! Handles the return to the main button.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    despawn_entity::DespawnEntity,
    game::{
        draw_piece::Piece,
        game_end::{GameResult, GameResultComponent},
        highlight::Highlight,
        player_name::{PlayerName, PlayerNameOutline},
    },
    listener::{
        button::{btn_spawn, style, BtnColorQuery, BtnContainer},
        click::Click,
        possible_paths::Paths,
    },
    startscreen::NameEntryValue,
    state::FortChessState,
    tiles::{block::Blocker, TileComponent},
};
use bevy::prelude::{
    App, Button, Changed, Commands, Component, Entity, Interaction, Plugin, Query, Res, ResMut,
    State, SystemSet, UiColor, With,
};

/// Text of the return button.
const RET_BTN_TEXT: &str = "Return";

/// Component to query return button.
#[derive(Component)]
struct ReturnButtonComponent;
/// Plugin to handle the return button.
pub(crate) struct ReturnButtonPlugin;

/// Type alias for return button
type ReturnBtnQuery = (
    Changed<Interaction>,
    With<Button>,
    With<ReturnButtonComponent>,
);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for ReturnButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for ReturnButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(FortChessState::ResultScreen).with_system(return_button_spawn),
        )
        .add_system_set(
            SystemSet::on_update(FortChessState::ResultScreen).with_system(return_btn_clicked),
        )
        .add_system_set(
            SystemSet::on_exit(FortChessState::ResultScreen)
                .with_system(return_main_res_clear)
                .with_system(return_main_desapawn_entity),
        );
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Return Main Button Clicked████*/
/*-----------------------------------------------------------------------------------------------*/
/// To animate the button clicked animations.
fn return_btn_clicked(
    mut interaction_query: Query<BtnColorQuery, ReturnBtnQuery>,
    mut state: ResMut<State<FortChessState>>,
) {
    interaction_query
        .iter_mut()
        .for_each(|(&interaction, mut color)| match interaction {
            Interaction::Clicked => {
                *color = UiColor::from(style::BTN_CLICKD_COLOR);
                state.set(FortChessState::StartScreen).unwrap();
            }
            Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
            Interaction::None => *color = UiColor::from(style::BTN_BKGRND_COLOR),
        });
}

/// To clean out the unnecessary resources.
fn return_main_res_clear(mut commands: Commands) {
    commands.remove_resource::<GameResult>();
    commands.remove_resource::<BtnContainer>();
    commands.remove_resource::<NameEntryValue>();
}

/// To clean the entities for a fresh game start.
fn return_main_desapawn_entity(
    mut commands: Commands,
    result_screen: Query<Entity, With<GameResultComponent>>,
    return_btn: Query<Entity, With<ReturnButtonComponent>>,
    board_tiles: Query<Entity, With<TileComponent>>,
    board_block: Query<Entity, With<Blocker>>,
    player_pieces: Query<Entity, With<Piece>>,
    player_hilite: Query<Entity, With<Highlight>>,
    player_names: Query<Entity, With<PlayerName>>,
    player_outline: Query<Entity, With<PlayerNameOutline>>,
    player_paths: Query<Entity, With<Paths>>,
    tile_clicked: Query<Entity, With<Click>>,
) {
    commands.despawn_entity(&result_screen);
    commands.despawn_entity(&return_btn);
    commands.despawn_entity(&board_tiles);
    commands.despawn_entity(&board_block);
    commands.despawn_entity(&player_pieces);
    commands.despawn_entity(&player_hilite);
    commands.despawn_entity(&player_names);
    commands.despawn_entity(&player_outline);
    commands.despawn_entity(&player_paths);
    commands.despawn_entity(&tile_clicked);
}

/*-----------------------------------------------------------------------------------------------*/

/// Spawns the return to main menu button.
fn return_button_spawn(mut commands: Commands, button: Res<BtnContainer>) {
    btn_spawn(&mut commands, &button, RET_BTN_TEXT, ReturnButtonComponent);
}
