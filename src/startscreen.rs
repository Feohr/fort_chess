//! start screen module.
//!
//! Module to handle the starting screen of the game.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

//-----------//
mod expand;
mod name_input;
mod startbtn;
//-----------//

use crate::{
    despawn_entity::DespawnEntity,
    font::{BoldFontHandle, RegFontHandle},
    state::FortChessState,
    ZAxisLevel, RESOLUTION,
};
use bevy::prelude::{
    default, AlignItems, AlignSelf, App, BuildChildren, ButtonBundle, ChildBuilder, Color,
    Commands, Component, Entity, FlexDirection, JustifyContent, NodeBundle, Plugin, Query, Res,
    Size, Style, SystemSet, Text, Text2dBundle, TextBundle, TextStyle, Transform, UiColor, UiImage,
    UiRect, Val, Visibility, With,
};
use expand::{
    style, ExpandBtnImage, ExpandTextInputButton, ExpandTextInputButtonPlugin, InputBoxNode,
    TextInputId,
};
use name_input::{NameInput, NameInputPlugin, NameInputText};
use startbtn::{spawn_start_btn, StartBtnPlugin};

/// Player name UI color.
const PLNAME_UI_COLOR: Color = Color::rgba(0.2_f32, 0.3_f32, 0.1_f32, 0.25_f32);
/// Color of the text input field.
const TEXT_INPUT_COLOR: Color = Color::ANTIQUE_WHITE;
/// Main title font color.
const MAIN_TITLE_COLOR: Color = Color::BLACK;
/// Main title font size.
const MAIN_TITLE_SIZE: f32 = 96_f32;
/// Length of the main text input node.
const TEXT_INPUT_NODE: (f32, f32) = (700_f32, 300_f32);
/// Placeholder for name.
pub(crate) const TEXT_INPUT_DEF_VAL: &str = "Enter your name";

pub(crate) trait FromBool {
    fn from_bool(value: bool) -> Self;
}
/// Main screen plugin.
pub(crate) struct MainScreenPlugin;
/// To signify player name input component.
#[derive(Component)]
struct PlayerNameInput;
/// To signify main title.
#[derive(Component)]
struct MainTitle;
/// Name entry value object.
#[derive(Debug)]
pub(crate) struct NameEntryValue {
    players: [String; 4_usize],
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for MainScreenPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(FortChessState::StartScreen)
                .with_system(name_entry_value_res)
                .with_system(title_text)
                .with_system(name_entry_text_box_ui),
        )
        .add_system_set(
            SystemSet::on_exit(FortChessState::StartScreen)
                .with_system(despawn_pname_text_input_box),
        )
        .add_plugin(ExpandTextInputButtonPlugin)
        .add_plugin(StartBtnPlugin)
        .add_plugin(NameInputPlugin);
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████NameEntryValue████*/
/*-----------------------------------------------------------------------------------------------*/
impl Default for NameEntryValue {
    /// Default implementation to create a [`NameEntryValue`].
    #[inline]
    fn default() -> Self {
        NameEntryValue {
            players: [
                String::default(),
                String::default(),
                String::default(),
                String::default(),
            ],
        }
    }
}

impl NameEntryValue {
    /// Count of player names entered.
    #[inline]
    pub(crate) fn count(&self) -> usize {
        self.players.iter().filter(|name| !name.is_empty()).count()
    }
    /// To get the string in the given position.
    #[inline]
    pub(crate) fn as_string(&self, index: usize) -> Result<String, usize> {
        if index >= self.players.len() {
            return Err(index);
        }
        Ok(self.players[index].clone())
    }
}

#[inline]
fn name_entry_value_res(mut commands: Commands) {
    commands.insert_resource(NameEntryValue::default());
}
/*-----------------------------------------------------------------------------------------------*/

/*████Main Screen UI████*/
/*-----------------------------------------------------------------------------------------------*/
/// To despawn text when the player leaves the screen.
#[inline]
fn despawn_pname_text_input_box(
    mut commands: Commands,
    input_box: Query<Entity, With<PlayerNameInput>>,
    text: Query<Entity, With<MainTitle>>,
) {
    commands.despawn_entity(&input_box);
    commands.despawn_entity(&text);
}

/// Main title text.
fn title_text(mut commands: Commands, font: Res<BoldFontHandle>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Fort Chess",
                TextStyle {
                    font: font.get().clone(),
                    font_size: MAIN_TITLE_SIZE,
                    color: MAIN_TITLE_COLOR,
                },
            ),
            transform: Transform::from_xyz(
                -5_f32 * RESOLUTION,
                7_f32 * RESOLUTION,
                ZAxisLevel::First.as_f32(),
            ),
            ..default()
        })
        .insert(MainTitle);
}

/// To insert the expand button to open and close the name input three and four.
fn expand_btn(
    commands: &mut ChildBuilder,
    asset_server: &Res<ExpandBtnImage>,
    textinputid: TextInputId,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(7_f32), Val::Percent(66_f32)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            color: UiColor::from(style::EXPAND_NORML),
            image: UiImage(asset_server.open.clone()),
            ..default()
        })
        .insert(ExpandTextInputButton {
            id: textinputid,
            expanded: false, // default value
        });
}

/// To insert the text box UI node.
fn text_box_sprite(
    commands: &mut ChildBuilder,
    expandable: bool,
    font: &Res<RegFontHandle>,
    asset_server: &Res<ExpandBtnImage>,
    textinputid: TextInputId,
) {
    if expandable {
        expand_btn(commands, asset_server, textinputid);
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(40_f32), Val::Percent(66_f32)),
                padding: UiRect::all(Val::Percent(1_f32)),
                ..default()
            },
            visibility: Visibility {
                is_visible: !expandable,
            },
            color: UiColor::from(Color::NONE),
            ..default()
        })
        .insert(InputBoxNode {
            expandable,
            id: textinputid,
        })
        .with_children(|commands| {
            commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100_f32), Val::Percent(100_f32)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: UiColor::from(TEXT_INPUT_COLOR),
                    ..default()
                })
                .insert(NameInput)
                .with_children(|commands| {
                    commands
                        .spawn_bundle(TextBundle::from_section(
                            TEXT_INPUT_DEF_VAL,
                            TextStyle {
                                font: font.get().clone(),
                                color: MAIN_TITLE_COLOR,
                                ..default()
                            },
                        ))
                        .insert(NameInputText);
                });
        });
}

/// To insert a row of text boxes.
fn text_box_sprite_node(
    commands: &mut ChildBuilder,
    expandable: bool,
    asset_server: &Res<ExpandBtnImage>,
    font: &Res<RegFontHandle>,
) {
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
            text_box_sprite(commands, expandable, font, asset_server, TextInputId::One);
            text_box_sprite(commands, expandable, font, asset_server, TextInputId::Two);
        });
}

/// To add the main entry box node.
///
/// Contains the insert boxes and buttons to start game.
fn name_entry_text_box_ui(
    mut commands: Commands,
    asset_server: Res<ExpandBtnImage>,
    start_btn_font: Res<RegFontHandle>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100_f32), Val::Percent(100_f32)),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Percent(6_f32)),
                ..default()
            },
            color: UiColor::from(Color::NONE),
            ..default()
        })
        .insert(PlayerNameInput)
        .with_children(|commands| {
            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(TEXT_INPUT_NODE.0), Val::Px(TEXT_INPUT_NODE.1)),
                        align_self: AlignSelf::FlexStart,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Percent(3_f32)),
                        ..default()
                    },
                    color: UiColor::from(PLNAME_UI_COLOR),
                    ..default()
                })
                .with_children(|commands| {
                    spawn_start_btn(commands, &start_btn_font);
                    text_box_sprite_node(commands, true, &asset_server, &start_btn_font);
                    text_box_sprite_node(commands, false, &asset_server, &start_btn_font);
                });
        });
}
/*-----------------------------------------------------------------------------------------------*/
