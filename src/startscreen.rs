/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::*;
use crate::{
    state::FortChessState,
    despawn_entity::DespawnEntity,
};

pub(crate) struct MainScreenPlugin;
#[derive(Component)]
struct PlayerNameInput;
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
                .with_system(name_entry_text_box_ui_setup)
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
            player1: String::default(),
            player2: String::default(),
            player3: None,
            player4: None,
        }
    }

}

fn despawn_player_name_text_input_box(
    mut commands: Commands,
    query: Query<Entity, With<PlayerNameInput>>,
) {
     commands.despawn_entity(&query);
}

fn name_entry_text_box_ui_setup(mut commands: Commands) {

    commands.insert_resource(dbg!(NameEntryValue::new()));
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100_f32), Val::Percent(100_f32)),
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: UiColor::from(Color::NONE),
        ..default()
    })
    .insert(PlayerNameInput)
    .with_children(|commands| {
        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(500_f32), Val::Px(250_f32)),
                ..default()
            },
            color: UiColor::from(Color::rgba(0.2_f32, 0.3_f32, 0.1_f32, 0.25_f32)),
            ..default()
        });
    });

}
