/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod expand;

use bevy::prelude::*;
use crate::{
    RESOLUTION, ZAxisLevel,
    state::FortChessState,
    despawn_entity::DespawnEntity,
    font::BoldFontHandle,
};
use expand::{ExpandTextInputButton, EXPAND_NORML, expand_btn_click};

/// Player name UI color.
const PLNAME_UI_COLOR:  Color = Color::rgba(0.2_f32, 0.3_f32, 0.1_f32, 0.25_f32);
/// Color of the text input field.
const TEXT_INPUT_COLOR: Color = Color::ANTIQUE_WHITE;
/// Main title font color.
const MAIN_TITLE_COLOR: Color = Color::BLACK;
/// Main title font size.
const MAIN_TITLE_SIZE: f32 = 96_f32;
/// Length of the main text input node.
const TEXT_INPUT_NODE: (f32, f32) = (700_f32, 300_f32);

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
struct NameEntryValue {
    player1: String,
    player2: String,
    player3: Option<String>,
    player4: Option<String>,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::StartScreen)
                .with_system(name_entry_value_res)
                .with_system(title_text)
                .with_system(name_entry_text_box_ui)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::StartScreen)
                .with_system(expand_btn_click)
            )
            .add_system_set(
                SystemSet::on_exit(FortChessState::StartScreen)
                .with_system(despawn_player_name_text_input_box)
            );
    }
}

impl NameEntryValue {

    fn new() -> Self {
        NameEntryValue {
            player1: String::new(),
            player2: String::new(),
            player3: None,
            player4: None,
        }
    }

}

fn despawn_player_name_text_input_box(
    mut commands:   Commands,
    input_box:      Query<Entity, With<PlayerNameInput>>,
    text:           Query<Entity, With<MainTitle>>
) {
     commands.despawn_entity(&input_box);
     commands.despawn_entity(&text);
}

fn name_entry_value_res(mut commands: Commands) {
    commands.insert_resource(NameEntryValue::new());
}

fn title_text(
    mut commands:   Commands,
    font:           Res<BoldFontHandle>,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Fort Chess",
            TextStyle {
                font: font.0.clone(),
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

fn expand_btn(
    commands:       &mut ChildBuilder,
    asset_server:   &Res<AssetServer>,
) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(7_f32), Val::Percent(66_f32)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceAround,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        color: UiColor::from(EXPAND_NORML),
        image: UiImage(asset_server.load("spritesheet/expand.png")),
        ..default()
    })
    .insert(ExpandTextInputButton);
}

fn text_box_sprite(
    commands:       &mut ChildBuilder,
    expandable:     bool,
    asset_server:   &Res<AssetServer>,
) {

    if expandable {
        expand_btn(commands, asset_server);
    }

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(40_f32), Val::Percent(66_f32)),
            padding: UiRect::all(Val::Percent(1_f32)),
            ..default()
        },
        visibility: Visibility { is_visible: !expandable },
        color: UiColor::from(TEXT_INPUT_COLOR),
        ..default()
    });

}

fn text_box_sprite_node(
    commands:       &mut ChildBuilder,
    expandable:     bool,
    asset_server:   &Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
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
        // Left text box.
        text_box_sprite(commands, expandable, asset_server);
        // Right text box.
        text_box_sprite(commands, expandable, asset_server);
    });
}

fn name_entry_text_box_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Input box.
    commands.spawn_bundle(NodeBundle {
        style: Style {
            // Fullscreen.
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
        // Main Node.
        commands.spawn_bundle(NodeBundle {
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
            // Top 2 names.
            text_box_sprite_node(commands, true,    &asset_server);
            // Bottom 2 names.
            text_box_sprite_node(commands, false,   &asset_server);
        });
   });
}
