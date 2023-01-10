//! name input module.
//!
//! Module to handle name input.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::*;
use crate::{
    FortChessState,
    startscreen::{
        FromBool, TEXT_INPUT_COLOR,
        expand::InputBoxNode,
    },
};
use std::fmt::Debug;

/// Plugin to handle input of the name input.
pub(crate) struct NameInputPlugin;
/// To signify a name input button node.
#[derive(Component)]
pub(crate) struct NameInput;
/// To check if the name input is selected.
#[derive(Debug)]
pub(crate) struct NameInputSelected {
    selected: [bool; 4_usize],
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for NameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::StartScreen)
                .with_system(name_input_selected_res)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::StartScreen)
                .with_system(name_input_click)
            );
    }
}

impl NameInputSelected {
    fn new() -> Self {
        NameInputSelected {
            selected: [false; _],
        }
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut bool> {
        self.selected.get_mut(index)
    }
}

fn name_input_selected_res(mut commands: Commands) {
    commands.insert_resource(NameInputSelected::new());
}

fn name_input_click(
    mut name_input_query:       Query<
        (&Interaction, &mut UiColor, &Parent),
        (Changed<Interaction>, With<Button>, With<NameInput>),
    >,
    parent_name_input:          Query<&InputBoxNode>,
    mut name_input_selected:    ResMut<NameInputSelected>,
) {
    name_input_query
        .iter_mut()
        .for_each(|(interaction, mut color, parent)| {
            match interaction {
                Interaction::Clicked    => {
                    if let Ok(parent) = parent_name_input.get(parent.get()) {
                        select_name_input_parent(
                            &parent,
                            &mut name_input_selected,
                        );
                        *color = UiColor::from(Color::from_bool(
                            *name_input_selected.get_mut(parent.as_usize()).unwrap()
                        )) ;
                    }
                    dbg!(&name_input_selected);
                },
                _ => {},
            }
        });
}

impl FromBool for Color {
    fn from_bool(value: bool) -> Self {
        match value {
            true    => Color::GRAY,
            false   => TEXT_INPUT_COLOR,
        }
    }
}

fn select_name_input_parent(
    parent:                 &InputBoxNode,
    name_input_selected:    &mut ResMut<NameInputSelected>,
) {
    if let Some(bool_val) = name_input_selected.get_mut(parent.as_usize()) {
        *bool_val = !(*bool_val);
    }
}
