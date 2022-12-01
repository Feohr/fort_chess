//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

pub(crate) mod skip_turn;
pub(crate) mod dice_roll;

mod style {

    use bevy::prelude::Color;

    /// Color of the button background.
    pub(crate) static BTN_BKGRND_COLOR: Color = Color::rgba(0.85_f32, 0.85_f32, 0.85_f32, 0.8_f32);
    /// Color of button hovered.
    pub(crate) static BTN_HOVERD_COLOR: Color = Color::rgba(0.75_f32, 0.75_f32, 0.75_f32, 0.8_f32);
    /// Color of button clicked.
    pub(crate) static BTN_CLICKD_COLOR: Color = Color::rgba(0.15_f32, 0.15_f32, 0.15_f32, 0.8_f32);
    /// Color of the button text.
    pub(crate) static BTN_FGTEXT_COLOR: Color = Color::WHITE;
    /// Color of the background node.
    pub(crate) static BTN_NODE_COLOR:   Color = Color::NONE;
    /// Button size.
    pub(crate) static BTN_SIZE      : (f32, f32) =   (153_f32, 51_f32);
    /// Button font size.
    pub(crate) static BTN_FONT_SIZE : f32        =   28_f32;

}

use bevy::{
    prelude::{
        Commands, AlignItems, Val, JustifyContent, default, ButtonBundle, Style, Size, TextStyle,
        TextBundle, AssetServer, UiColor, Res, UiRect, Component, Plugin, App, AlignSelf,
        NodeBundle,
    },
    hierarchy::{BuildChildren, ChildBuilder},
};
use skip_turn::SkipButtonPlugin;
use dice_roll::DiceRollButtonPlugin;
use fort_builders::game::{Game, GameAction};

/// Closure to hold `skip turn` button closures to run.
static SKIP_TURN_GAME_CLOSURES: [fn(&mut Game); 3_usize] = [
    GameAction::next_player, Game::set_update_true, Game::set_picked_false,
];

/// Plugin that handles the buttons.
pub(crate) struct FortButtonPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for FortButtonPlugin {

    fn build(&self, app: &mut App) {
        app .add_plugin(SkipButtonPlugin        )
            .add_plugin(DiceRollButtonPlugin    );
    }

}

/// To spawn a text component within the button.
fn btn_text_spawn<'a>(
    commands:       &mut ChildBuilder,
    asset_server:   &Res<AssetServer>,
    text:           &'a str,
) {

    commands.spawn_bundle(TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/fira-sans.regular.ttf"),
            font_size: style::BTN_FONT_SIZE,
            color: style::BTN_FGTEXT_COLOR,
        },
    ));

}

/// To spawn the button component.
fn btn_bg_spawn<'a>(
    commands:           &mut ChildBuilder,
    asset_server:       &Res<AssetServer>,
    text:               &'a str,
    button_component:   impl Component,
) {

    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(
                      Val::Px(style::BTN_SIZE.0),
                      Val::Px(style::BTN_SIZE.1),
            ),
            align_self: AlignSelf::FlexEnd,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: UiColor::from(style::BTN_BKGRND_COLOR),
        image: asset_server.load("spritesheet/button.png").into(),
        ..default()
    })
    .with_children(|parent| btn_text_spawn(parent, &asset_server, text))
    .insert(button_component);

}

/// To spawn a button node.
pub(crate) fn btn_spawn<'a>(
    commands:           &mut Commands,
    asset_server:       &Res<AssetServer>,
    text:               &'a str,
    button_component:   impl Component,
) {

    // Spawning a UI Node.
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(14_f32),
                Val::Percent(14_f32),
            ),
            align_self: AlignSelf::FlexEnd,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Percent(1_f32)),
            ..default()
        },
        color: UiColor::from(style::BTN_NODE_COLOR),
        ..default()
    })
    .with_children(|parent| btn_bg_spawn(parent, asset_server, text, button_component));

}
