//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    despawn_entity::DespawnEntity,
    game::GameAsset,
    listener::{
        button::{btn_spawn, style, BtnColorQuery, BtnContainer},
        click::Click,
        possible_paths::{Paths, PossiblePaths},
    },
    state::FortChessState,
};
use bevy::prelude::{
    App, Button, Changed, Commands, Component, Entity, Interaction, Plugin, Query, Res, ResMut,
    SystemSet, UiColor, With,
};
use fort_builders::game::GameAction;

/// To hold the button text.
const SKIP_TURN_BTN_TEXT: &str = "Skip Turn";

/// Plugin to handle `skip_turn` button.
pub(crate) struct SkipButtonPlugin;
#[derive(Component)]
pub struct SkipTurnButton;

/// Type alias for skip turn button query.
type SkipTurnBtnQuery = (Changed<Interaction>, With<Button>, With<SkipTurnButton>);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for SkipButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for SkipButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(FortChessState::GameBuild).with_system(skip_turn_btn_spawn),
        )
        .add_system_set(
            SystemSet::on_update(FortChessState::BoardScreen).with_system(skip_turn_btn_clicked),
        );
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Skip Turn Button Click████*/
/*-----------------------------------------------------------------------------------------------*/
/// To handle the 'skip turn' button being clicked.
fn skip_turn_btn_clicked(
    mut commands: Commands,
    mut interaction_query: Query<BtnColorQuery, SkipTurnBtnQuery>,
    mut game: ResMut<GameAsset>,
    mut paths: ResMut<PossiblePaths>,
    paths_query: Query<Entity, With<Paths>>,
    click_query: Query<Entity, With<Click>>,
) {
    interaction_query
        .iter_mut()
        .for_each(|(&interaction, mut color)| match interaction {
            Interaction::Clicked => {
                *color = UiColor::from(style::BTN_CLICKD_COLOR);
                game.get_mut()
                    .next_player()
                    .set_update_true()
                    .set_picked_false();
                paths.clear();
                commands.despawn_entity(&click_query);
                commands.despawn_entity(&paths_query);
            }
            Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
            Interaction::None => *color = UiColor::from(style::BTN_BKGRND_COLOR),
        });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Skip Turn Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/
/// To setup the `skip turn` button.
#[inline]
fn skip_turn_btn_spawn(mut commands: Commands, button: Res<BtnContainer>) {
    btn_spawn(&mut commands, &button, SKIP_TURN_BTN_TEXT, SkipTurnButton);
}
/*-----------------------------------------------------------------------------------------------*/
