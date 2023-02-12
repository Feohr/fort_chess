//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

pub(crate) mod skip_turn;
pub(crate) mod dice_roll;
pub(crate) mod return_main;

mod style {
    use bevy::prelude::Color;
    /// Color of the button background.
    pub(crate) static BTN_BKGRND_COLOR: Color = Color::rgba(0.85_f32, 0.85_f32, 0.85_f32, 0.8_f32);
    /// Color of button hovered.
    pub(crate) static BTN_HOVERD_COLOR: Color = Color::rgba(0.75_f32, 0.75_f32, 0.75_f32, 0.8_f32);
    /// Color of button clicked.
    pub(crate) static BTN_CLICKD_COLOR: Color = Color::rgba(0.15_f32, 0.15_f32, 0.15_f32, 0.8_f32);
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
        TextBundle, UiColor, Res, UiRect, Component, Plugin, App, AlignSelf, NodeBundle, With,
        AssetServer, Handle, Font, Image, Color, SystemSet, Query, Entity,
    },
    hierarchy::{BuildChildren, ChildBuilder},
};
use crate::{
    font::{RegFontHandle, DEFAULT_FONT_CLR},
    state::FortChessState,
    despawn_entity::DespawnEntity,
};
use skip_turn::SkipButtonPlugin;
use dice_roll::DiceRollButtonPlugin;
use return_main::ReturnButtonPlugin;

/// Object to create instances of button.
pub(crate) struct BtnContainer {
    /// Font type for the button.
    font: Handle<Font>,
    /// Font size for the button.
    font_size: f32,
    /// Image for the button.
    image: Handle<Image>,
    /// Size of the UI node.
    size: Size<Val>,
    /// Size of the button.
    btn_size: Size<Val>,
    /// Resting color of the button.
    bg_color: Color,
    /// Color of the text.
    fg_text_color: Color,
}
/// Plugin that handles the buttons.
pub(crate) struct FortButtonPlugin;
#[derive(Component)]
struct BoardButton;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for FortButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::StartScreen)
                .with_system(init_btn_obj)
            )
            .add_system_set(
                SystemSet::on_exit(FortChessState::BoardScreen)
                .with_system(despawn_buttons)
            )
            .add_plugin(SkipButtonPlugin    )
            .add_plugin(DiceRollButtonPlugin)
            .add_plugin(ReturnButtonPlugin);
    }
}

/// Despawns button when leaving the board screen.
fn despawn_buttons(
    mut commands:   Commands,
    buttons:        Query<Entity, With<BoardButton>>,
) {
    commands.despawn_entity(&buttons);
}

/// Creating button object and adding it to resources to be able to call later.
fn init_btn_obj(
    mut commands:   Commands,
    font:           Res<RegFontHandle>,
    asset_server:   Res<AssetServer>,
) {
    commands.insert_resource(BtnContainer{
        font: font.get().clone(),
        font_size: style::BTN_FONT_SIZE,
        image: asset_server.load("spritesheet/button.png").into(),
        size: Size::new(Val::Percent(14_f32), Val::Percent(14_f32)),
        btn_size: Size::new(Val::Px(style::BTN_SIZE.0), Val::Px(style::BTN_SIZE.1)),
        bg_color: style::BTN_BKGRND_COLOR,
        fg_text_color: DEFAULT_FONT_CLR,
    });
}

/// To spawn a text component within the button.
fn btn_text_spawn<'a>(
    commands:   &mut ChildBuilder,
    button:     &Res<BtnContainer>,
    text:       &'a str,
) {
    commands.spawn_bundle(TextBundle::from_section(
        text,
        TextStyle {
            font: button.font.clone(),
            font_size: button.font_size,
            color: button.fg_text_color,
        },
    ));
}

/// To spawn the button component.
fn btn_bg_spawn<'a>(
    commands:           &mut ChildBuilder,
    button:             &Res<BtnContainer>,
    text:               &'a str,
    button_component:   impl Component,
) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: button.btn_size,
            align_self: AlignSelf::FlexEnd,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: UiColor::from(button.bg_color),
        image: button.image.clone().into(),
        ..default()
    })
    .with_children(|parent| btn_text_spawn(parent, button, text))
    .insert(button_component);
}

/// To spawn a button node.
pub(crate) fn btn_spawn<'a>(
    commands:           &mut Commands,
    button:             &Res<BtnContainer>,
    text:               &'a str,
    button_component:   impl Component,
) {
    // Spawning a UI Node.
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: button.size,
            align_self: AlignSelf::FlexEnd,
            align_items: AlignItems::Stretch,
            padding: UiRect::all(Val::Percent(1_f32)),
            ..default()
        },
        color: UiColor::from(style::BTN_NODE_COLOR),
        ..default()
    })
    .with_children(|parent| btn_bg_spawn(parent, button, text, button_component))
    .insert(BoardButton);
}
