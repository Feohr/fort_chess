//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::{
        Commands, Query, ResMut, With, Plugin, App, UiColor, Res, Button, Interaction, Changed,
        Component, Entity, SystemSet,
    },
};
use crate::{
    despawn_entity::DespawnEntity,
    game::GameAsset,
    listener::{
        possible_paths::{PossiblePaths, Paths},
        button::{style, btn_spawn, SKIP_TURN_GAME_CLOSURES, BtnContainer},
        click::Click,
    },
    state::FortChessState,
};

/// To hold the button text.
const SKIP_TURN_BTN_TEXT: &str = "Skip Turn";
/// Plugin to handle `skip_turn` button.
pub(crate) struct SkipButtonPlugin;

#[derive(Component)]
pub struct SkipTurnButton;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for SkipButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for SkipButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::GameBuild)
                .with_system(skip_turn_btn_spawn)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::BoardScreen)
                .with_system(skip_turn_btn_clicked)
            );
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Skip Turn Button Click████*/
/*-----------------------------------------------------------------------------------------------*/
/// To handle the 'skip turn' button being clicked.
fn skip_turn_btn_clicked(
    mut commands:   Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<SkipTurnButton>),
    >,
    mut game:       ResMut<GameAsset>,
    mut paths:      ResMut<PossiblePaths>,
    paths_query:    Query<Entity, With<Paths>>,
    click_query:    Query<Entity, With<Click>>,
) {

    interaction_query
        .iter_mut()
        .for_each(|(&interaction, mut color)| {
            match interaction {
                Interaction::Clicked => {
                    // Updating color,
                    *color = UiColor::from(style::BTN_CLICKD_COLOR);
                    // Iterating over a bunch of closures to process.
                    SKIP_TURN_GAME_CLOSURES
                        .into_iter()
                        .for_each(|game_closure| game_closure(game.get_mut()));
                    // To clear off the paths and clear the screen for next player.
                    paths.clear();
                    commands.despawn_entity(&click_query);
                    commands.despawn_entity(&paths_query);
                },
                Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
                Interaction::None    => *color = UiColor::from(style::BTN_BKGRND_COLOR),
            }
        });

}
/*-----------------------------------------------------------------------------------------------*/

/*████Skip Turn Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/
/// To setup the `skip turn` button.
#[inline]
fn skip_turn_btn_spawn(
    mut commands:   Commands,
    button:         Res<BtnContainer>,
) {
    btn_spawn(&mut commands, &button, SKIP_TURN_BTN_TEXT, SkipTurnButton);
}
/*-----------------------------------------------------------------------------------------------*/
