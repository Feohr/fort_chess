//! name input module.
//!
//! Module to handle name input.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::*,
//    input::keyboard::KeyboardInput,
};
use crate::{
    FortChessState,
    startscreen::{
//        NameEntryValue,
        TextInputId,
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
/// To get the text of the input bar.
#[derive(Component)]
pub(crate) struct NameInputText {
    pub(crate) id: TextInputId,
}
/// To check if the name input is selected.
#[derive(Debug)]
pub(crate) struct NameInputSelected {
    selected: [bool; 4_usize],
    render: bool,
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
                .with_system(name_input_color)
            );
    }
}

impl NameInputSelected {
    fn new() -> Self {
        NameInputSelected {
            selected: [false; _],
            render: false,
        }
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut bool> {
        self.selected.get_mut(index)
    }
    fn render(&mut self) -> &mut Self {
        self.render = true;
        self
    }
    fn unrender(&mut self) -> &mut Self {
        self.render = false;
        self
    }
    fn mutex(&mut self, index: usize) -> &mut Self {
        self.selected
            .iter_mut()
            .enumerate()
            .filter(|(index_f, _)| index_f.ne(&index))
            .for_each(|(_, name)| *name = false);
        self
    }
}

fn name_input_selected_res(mut commands: Commands) {
    commands.insert_resource(NameInputSelected::new());
}

fn name_input_click(
    mut name_input_query:       Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<Button>, With<NameInput>),
    >,
    parent_name_input:          Query<&InputBoxNode>,
    mut name_input_selected:    ResMut<NameInputSelected>,
) {
    name_input_query
        .iter_mut()
        .for_each(|(interaction, parent)| {
            match interaction {
                Interaction::Clicked    => {
                    if let Ok(parent) = parent_name_input.get(parent.get()) {
                        select_name_input_parent(
                            &parent,
                            &mut name_input_selected,
                        );
                    }
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

fn name_input_color(
    input_node_query:           Query<&InputBoxNode>,
    mut text_input_query:       Query<(&mut UiColor, &Parent), (With<Button>, With<NameInput>)>,
    mut name_input_selected:    ResMut<NameInputSelected>,
) {
    if !name_input_selected.render { return }
    text_input_query
        .iter_mut()
        .for_each(|(mut color, parent)| {
            if let Ok(parent) = input_node_query.get(parent.get()) {
                if let Some(name) = name_input_selected.get_mut(parent.as_usize()) {
                    *color = UiColor::from(Color::from_bool(*name));
                }
            }
        });
    name_input_selected.unrender();
}

fn select_name_input_parent(
    parent:                 &InputBoxNode,
    name_input_selected:    &mut ResMut<NameInputSelected>,
) {
    if let Some(bool_val) = name_input_selected.get_mut(parent.as_usize()) {
        *bool_val = !(*bool_val);
    } else { return }
    name_input_selected.mutex(parent.as_usize()).render();
}
