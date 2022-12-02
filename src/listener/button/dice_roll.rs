//! dice_roll module.
//!
//! To handle the `dice roll` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::{
        Commands, Component, Query, ResMut, With, Plugin, App, AssetServer, UiColor, Res, Button,
        Interaction, Changed, StartupStage, Visibility, Entity,
    },
};
use crate::{
    despawn_entity::DespawnEntity,
    game::GameAsset,
    listener::{
        possible_paths::{PossiblePaths, Paths},
        click::Click,
        button::{btn_spawn, style, SKIP_TURN_GAME_CLOSURES},
    },
};
use fort_builders::{
    dice_roll,
//    game::GameAction,
    player::PlayerAction,
};

/// To hold the button text.
const DICE_ROLL_BTN_TEXT: &str = "Dice Roll";

/// Plugin to handle `skip_turn` button.
pub(crate) struct DiceRollButtonPlugin;

/// To signify a DiceRoll Button.
#[derive(Component)]
pub(crate) struct DiceRollButton;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for DiceRollButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for DiceRollButtonPlugin {

    // The `dice roll` button was spawning before the `skip turn` button unpredictably. Hence,
    // the button spawning is pushed to `PostStartup`. Not the most elegant solution.
    fn build(&self, app: &mut App) {
        app .add_startup_system_to_stage(StartupStage::PostStartup, dice_roll_btn_spawn     )
            .add_system(                                            dice_roll_btn_clicked   )
            .add_system(                                            dice_roll_btn_visibility);
    }

}
/*-----------------------------------------------------------------------------------------------*/
/*████Dice Roll Button Visibility████*/
/*-----------------------------------------------------------------------------------------------*/
/// Fucntion to check if the piece is at the other side of the board in enemy territory so that we
/// can make the `roll_dice` button visible.
fn dice_roll_btn_visibility(
    mut dice_roll_query:    Query<&mut Visibility, With<DiceRollButton>>,
    game:                   Res<GameAsset>,
) {

    // Matching to see if the current player piece is in opposite side.
    match game.get().current_player().in_opposite_side() && game.get().picked {
        true => {
            dice_roll_query
                .iter_mut()
                .for_each(|mut visibility| visibility.is_visible = true)
        },
        false => {
            dice_roll_query
                .iter_mut()
                .for_each(|mut visibility| visibility.is_visible = false)
        },
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Button Clicked████*/
/*-----------------------------------------------------------------------------------------------*/
fn dice_roll_btn_clicked(
    mut commands:   Commands,
    mut dice_roll_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<DiceRollButton>),
    >,
    mut game:       ResMut<GameAsset>,
    mut paths:      ResMut<PossiblePaths>,
    paths_query:    Query<Entity, With<Paths>>,
    click_query:    Query<Entity, With<Click>>,
) {

    // Matching with the interaction to display the respective animations.
    for (&interaction, mut color) in &mut dice_roll_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                // Updating color,
                *color = UiColor::from(style::BTN_CLICKD_COLOR);
                // Rolling the dice.
                if dice_roll() == 5_usize {
                    // Set the player as winner if it is six.
                    game.get_mut().current_player_mut().set_winner();
                    dbg!(&game);
                }
                // To chenge the display and move the player along.
                SKIP_TURN_GAME_CLOSURES
                    .into_iter()
                    .for_each(|game_closure| game_closure(game.get_mut()));
                paths.clear();
                commands.despawn_entity(&click_query);
                commands.despawn_entity(&paths_query);
            },
            Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
            Interaction::None    => *color = UiColor::from(style::BTN_BKGRND_COLOR),
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/
/// To spawn a button.
fn dice_roll_btn_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    btn_spawn(&mut commands, &asset_server, DICE_ROLL_BTN_TEXT, DiceRollButton);

}
/*-----------------------------------------------------------------------------------------------*/
