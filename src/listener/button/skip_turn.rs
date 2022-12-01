//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::{
        Commands, Query, ResMut, With, Plugin, App, AssetServer, UiColor, Res, Button, Interaction,
        Changed, Component, Entity,
    },
};
use crate::{
    despawn_entity,
    game::GameAsset,
    listener::{
        possible_paths::{PossiblePaths, Paths},
        button::{style, btn_spawn, SKIP_TURN_GAME_CLOSURES},
        click::Click,
    },
};
// use fort_builders::game::{GameAction, Game};

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
        app .add_startup_system(skip_turn_btn_spawn     )
            .add_system(        skip_turn_btn_clicked   );
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

    for (&interaction, mut color) in &mut interaction_query {
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
                despawn_entity(&mut commands, &click_query);
                despawn_entity(&mut commands, &paths_query);
            },
            Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
            Interaction::None    => *color = UiColor::from(style::BTN_BKGRND_COLOR),
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Skip Turn Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/
/// To setup the `skip turn` button.
fn skip_turn_btn_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    btn_spawn(&mut commands, &asset_server, SKIP_TURN_BTN_TEXT, SkipTurnButton);

}
/*-----------------------------------------------------------------------------------------------*/
