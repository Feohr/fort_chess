//! return main module
//!
//! Handles the return to the main button.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    state::FortChessState,
    listener::button::{BtnContainer, btn_spawn, style},
    game::game_end::{GameResult, GameResultComponent},
    startscreen::NameEntryValue,
    despawn_entity::DespawnEntity,
};
use bevy::prelude::{
    Commands, Res, App, Plugin, SystemSet, Component, UiColor, Query, Interaction, Button, With,
    Changed, ResMut, State, Entity,
};

/// Text of the return button.
const RET_BTN_TEXT: &str = "Return";

#[derive(Component)]
struct ReturnButtonComponent;
pub(crate) struct ReturnButtonPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for ReturnButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::ResultScreen)
                .with_system(return_button_spawn)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::ResultScreen)
                .with_system(return_btn_clicked)
            )
            .add_system_set(
                SystemSet::on_exit(FortChessState::ResultScreen)
                .with_system(return_to_main_process)
            );
    }
}
/*████Return Main Button Clicked████*/
/*-----------------------------------------------------------------------------------------------*/
/// To animate the button clicked animations.
fn return_btn_clicked(
    mut interaction_query:  Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ReturnButtonComponent>),
    >,
    mut state:              ResMut<State<FortChessState>>,
) {
    interaction_query
        .iter_mut()
        .for_each(|(&interaction, mut color)| {
            match interaction {
                Interaction::Clicked => {
                    *color = UiColor::from(style::BTN_CLICKD_COLOR);
                    state.set(FortChessState::StartScreen).unwrap();
                },
                Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
                Interaction::None    => *color = UiColor::from(style::BTN_BKGRND_COLOR),
            }
        });
}

fn return_to_main_process(
    mut commands: Commands,
    game_screen: Query<Entity, With<GameResultComponent>>,
) {
    commands.remove_resource::<GameResult>();
    commands.remove_resource::<BtnContainer>();
    commands.remove_resource::<NameEntryValue>();
    commands.despawn_entity(&game_screen);
}


/*-----------------------------------------------------------------------------------------------*/

/// Spawns the return to main menu button.
fn return_button_spawn(
    mut commands:   Commands,
    button:         Res<BtnContainer>,
) {
    btn_spawn( &mut commands, &button, RET_BTN_TEXT, ReturnButtonComponent);
}
