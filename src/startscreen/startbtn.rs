//! start button module.
//!
//! Module to handle the start button.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod style {
    use bevy::prelude::Color;
    /// Color when the button is clicked.
    pub(crate) const START_BTN_CLICK: Color = Color::DARK_GRAY;
    /// Color when the button is hovered.
    pub(crate) const START_BTN_HOVER: Color = Color::GRAY;
    /// Color when the button is idle.
    pub(crate) const START_BTN_NORML: Color = Color::SILVER;
    /// Color for the error message text.
    pub(crate) const ERRMSG_TEXT_CLR: Color = Color::RED;
    /// Size of the error message text.
    pub(crate) const ERRMSG_TEXT_SIZE: f32 = 40_f32;
    /// Size of the font of buttons.
    pub(crate) const START_BTN_TEXT_SIZE: f32 = 36_f32;
}

use crate::{
    close_window,
    despawn_entity::DespawnEntity,
    font::{BoldFontHandle, RegFontHandle, DEFAULT_FONT_CLR},
    startscreen::NameEntryValue,
    FortChessState, ZAxisLevel, RESOLUTION,
};
use bevy::prelude::{
    default, AlignItems, App, BuildChildren, Button, ButtonBundle, Changed, ChildBuilder, Children,
    Color, Commands, Component, Entity, Interaction, JustifyContent, NodeBundle, Plugin, Query,
    Res, ResMut, Size, State, Style, SystemSet, Text, Text2dBundle, TextBundle, TextStyle,
    Transform, UiColor, UiRect, Val, Windows, With,
};
use fort_builders::player::{NAME_MAX_LEN, NAME_MIN_LEN};

/// [`Component`] to identify start button.
#[derive(Component, Debug)]
struct ButtonComp {
    btn_type: ButtonType,
}
/// To determine the button type and appropriate action.
#[derive(Debug)]
enum ButtonType {
    Start,
    Exit,
}
/// [`Component`] to identify start button text.
#[derive(Component)]
struct ButtonText;
/// [`Plugin`] to handle start button function.
pub(crate) struct StartBtnPlugin;
/// [`Component`] to identify error message.
#[derive(Component)]
pub struct ErrDropDown;

/// Type alias for start button query.
type StartBtnQuery = (Changed<Interaction>, With<Button>, With<ButtonComp>);
/// Type alias for start button color query.
type StartBtnColorQuery<'a> = (
    &'a Interaction,
    &'a mut UiColor,
    &'a Children,
    &'a ButtonComp,
);

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for StartBtnPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for StartBtnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(FortChessState::StartScreen).with_system(start_btn_click),
        );
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Spawn Start Button████*/
/*-----------------------------------------------------------------------------------------------*/
/// To spwan the start button sprite.
///
/// Takes font handle and spawns a UI node with insert and exit button as children.
pub(crate) fn spawn_start_btn(commands: &mut ChildBuilder, font: &Res<RegFontHandle>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100_f32), Val::Percent(33.33_f32)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::SpaceAround,
                padding: UiRect::all(Val::Percent(1_f32)),
                ..default()
            },
            color: UiColor::from(Color::NONE),
            ..default()
        })
        .with_children(|commands| {
            start_btn(commands, font);
            exit_btn(commands, font);
        });
}

/// To spawn a start button UI node.
fn start_btn(commands: &mut ChildBuilder, font: &Res<RegFontHandle>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(20_f32), Val::Percent(66_f32)),
                padding: UiRect::all(Val::Percent(1_f32)),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor::from(style::START_BTN_NORML),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn_bundle(TextBundle::from_section(
                    "Start",
                    TextStyle {
                        font: font.get().clone(),
                        font_size: style::START_BTN_TEXT_SIZE,
                        color: DEFAULT_FONT_CLR,
                    },
                ))
                .insert(ButtonText);
        })
        .insert(ButtonComp {
            btn_type: ButtonType::Start,
        });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Spawn Exit Button████*/
/*-----------------------------------------------------------------------------------------------*/
/// To spawn an exit button UI node.
fn exit_btn(commands: &mut ChildBuilder, font: &Res<RegFontHandle>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(20_f32), Val::Percent(66_f32)),
                padding: UiRect::all(Val::Percent(1_f32)),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor::from(style::START_BTN_NORML),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn_bundle(TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font: font.get().clone(),
                        font_size: style::START_BTN_TEXT_SIZE,
                        color: DEFAULT_FONT_CLR,
                    },
                ))
                .insert(ButtonText);
        })
        .insert(ButtonComp {
            btn_type: ButtonType::Exit,
        });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Button Click████*/
/*-----------------------------------------------------------------------------------------------*/
/// To listen when the buttons on the start screen are clicked and handle them appropriately.
fn start_btn_click(
    mut commands: Commands,
    mut start_btn_query: Query<StartBtnColorQuery, StartBtnQuery>,
    mut text_query: Query<&mut Text, With<ButtonText>>,
    mut windows: ResMut<Windows>,
    name_entry_value: Res<NameEntryValue>,
    mut state: ResMut<State<FortChessState>>,
    font: Res<BoldFontHandle>,
    err_msg_query: Query<Entity, With<ErrDropDown>>,
) {
    start_btn_query
        .iter_mut()
        .for_each(|(&interaction, mut color, children, _btn_comp)| {
            let text_color = &mut text_query
                .get_mut(children[0])
                .expect("button does not have text inside")
                .sections[0]
                .style
                .color;
            match interaction {
                Interaction::Clicked => {
                    *color = UiColor::from(style::START_BTN_CLICK);
                    *text_color = Color::WHITE;
                    click(
                        &mut commands,
                        &_btn_comp.btn_type,
                        &mut windows,
                        &name_entry_value,
                        &mut state,
                        &font,
                        &err_msg_query,
                    );
                }
                Interaction::Hovered => {
                    *color = UiColor::from(style::START_BTN_HOVER);
                    *text_color = DEFAULT_FONT_CLR;
                }
                Interaction::None => {
                    *color = UiColor::from(style::START_BTN_NORML);
                    *text_color = DEFAULT_FONT_CLR;
                }
            }
        });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Start Button Click████*/
/*-----------------------------------------------------------------------------------------------*/
/// To process when a button is clicked in the start screen.
fn click(
    commands: &mut Commands,
    button_type: &ButtonType,
    windows: &mut ResMut<Windows>,
    name_entry_value: &Res<NameEntryValue>,
    state: &mut ResMut<State<FortChessState>>,
    font: &Res<BoldFontHandle>,
    err_msg_query: &Query<Entity, With<ErrDropDown>>,
) {
    match button_type {
        ButtonType::Start => {
            commands.despawn_entity(err_msg_query);
            validate_and_start_game(commands, name_entry_value, state, font);
        }
        ButtonType::Exit => close_window(windows),
    }
}

/// To check the name validations and start if all are clear else display an error message.
fn validate_and_start_game(
    commands: &mut Commands,
    name_entry_value: &Res<NameEntryValue>,
    state: &mut ResMut<State<FortChessState>>,
    font: &Res<BoldFontHandle>,
) {
    if name_entry_value
        .players
        .get(0_usize)
        .unwrap_or(&String::new())
        .is_empty()
        && name_entry_value
            .players
            .get(1_usize)
            .unwrap_or(&String::new())
            .is_empty()
    {
        err_msg(commands, font, "There needs to be a minimum of 2 players");
        return;
    }
    for name in name_entry_value.players.iter() {
        {
            let len = name.len();
            if (len <= NAME_MIN_LEN || len >= NAME_MAX_LEN) && len != 0_usize {
                err_msg(
                    commands,
                    font,
                    "Name length should be greater than 3 and less than 15",
                );
                return;
            }
        }
        if name.contains('\u{0020}') || name.contains('\u{00a0}') {
            err_msg(commands, font, "Name cannot have whitespace");
            return;
        }
        if let Some(ch) = name.chars().next() {
            if !ch.is_alphabetic() {
                err_msg(commands, font, "Name should start with an alphabet");
                return;
            }
            if !ch.is_ascii() {
                err_msg(commands, font, "Name can only contain ascii values");
                return;
            }
            if ch.is_ascii_punctuation() {
                err_msg(commands, font, "Name cannot have punctuations");
                return;
            }
        }
    }
    state.set(FortChessState::GameBuild).unwrap_or_default();
}

/// Display an error message in the given spot when called.
fn err_msg(commands: &mut Commands, font: &Res<BoldFontHandle>, err_msg: &str) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                err_msg,
                TextStyle {
                    font: font.get().clone(),
                    font_size: style::ERRMSG_TEXT_SIZE,
                    color: style::ERRMSG_TEXT_CLR,
                },
            ),
            transform: Transform::from_xyz(
                -8_f32 * RESOLUTION,
                8_f32 * RESOLUTION,
                ZAxisLevel::First.as_f32(),
            ),
            ..default()
        })
        .insert(ErrDropDown);
}
/*-----------------------------------------------------------------------------------------------*/
