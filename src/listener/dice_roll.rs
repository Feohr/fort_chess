//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::{
        Commands, Component, Query, ResMut, With, Plugin, App, AlignItems, Val, JustifyContent,
        default, ButtonBundle, NodeBundle, Style, Size, TextStyle, TextBundle, AssetServer,
        UiColor, Res, UiRect, Button, Interaction, Changed,
    },
    hierarchy::{BuildChildren, ChildBuilder},
};
use crate::{
    RESOLUTION,
    game::GameAsset,
};
use fort_builders::game::GameAction;

/// Plugin to handle `skip_turn` button.
pub(crate) struct DiceRollButtonPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for DiceRollButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for SkipButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_startup_system(skip_turn_btn_node_spawn)
            .add_system(        dice_roll_btn_visibility);
    }
}
/*-----------------------------------------------------------------------------------------------*/

