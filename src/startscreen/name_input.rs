//! name input module.
//!
//! Module to handle name input.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::*;
use crate::{
    FortChessState,
    startscreen::{
        NameEntryValue, TEXT_INPUT_DEF_VAL,
        FromBool, TEXT_INPUT_COLOR,
        expand::InputBoxNode,
    },
    font::DEFAULT_FONT_CLR,
};
use std::fmt::Debug;

/// Color of the input selection.
const INPUT_SELECT_CLR: Color = Color::rgb(0.76_f32, 0.76_f32, 0.76_f32);
/// Color of the default filler text holder.
const TEXT_HOLDER_CLR:  Color = Color::GRAY;

/// Plugin to handle input of the name input.
pub(crate) struct NameInputPlugin;
/// To signify a name input button node.
#[derive(Component)]
pub(crate) struct NameInput;
/// To get the text of the input bar.
#[derive(Component)]
pub(crate) struct NameInputText;
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
                .with_system(name_input_click       )
                .with_system(name_input_color       )
                .with_system(text_typing            )
                .with_system(display_text_to_input  )
            );
    }
}

impl NameInputSelected {
    /// To create a new [`NameInputSelected`] object.
    ///
    /// Whenever called the function returns an instance of [`NameInputSelected`]. The default is
    /// all false.
    #[inline]
    fn new() -> Self {
        NameInputSelected {
            selected: [false; _],
            render: false,
        }
    }
    /// to get the `selected` vector value at the given index.
    ///
    /// The function takes an index value of `usize` and returns an `Option` value with a mutable
    /// `boolean` of the corresponding `selected` field value.
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut bool> {
        self.selected.get_mut(index)
    }
    /// To set the `render` field as true.
    ///
    /// When called, the function turns the `render` value to `true`.
    /// **Idempotent Function**
    #[inline]
    fn render(&mut self) {
        self.render = true;
    }
    /// To set the `render` field as false.
    ///
    /// When called, the function turns the `render` value to `false`.
    /// **Idempotent Function**
    #[inline]
    fn unrender(&mut self) {
        self.render = false;
    }
    /// To maintain only one `selected` value as `true`.
    ///
    /// To iterate through the `selected` value with the given index and set all the values as
    /// `false` except the value corresponding to the `index` position.
    #[inline]
    fn mutex(&mut self, index: usize) -> &mut Self {
        self.selected
            .iter_mut()
            .enumerate()
            // Filter the value whose index is same as given input index.
            .filter(|(index_f, _)| index_f.ne(&index))
            // Set all the values of the name as `false`.
            .for_each(|(_, name)| *name = false);
        self
    }
}

/// To insert the [`NameInputSelected`] resource into the `WorldQuery`.
#[inline]
fn name_input_selected_res(mut commands: Commands) {
    commands.insert_resource(NameInputSelected::new());
}

/// To make input clickable.
///
/// Takes the query with [`NameInput`] Component and processes the clicks and mouse movement. The
/// corresponding [`NameInputSelected`] values are noted down and referred when key presses are
/// listened.
fn name_input_click(
    mut name_input_query:       Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<Button>, With<NameInput>),
    >,
    parent_name_input:          Query<&InputBoxNode>,
    mut name_input_selected:    ResMut<NameInputSelected>,
) {
    // Iterate through the query and process each NameInput.
    name_input_query
        .iter_mut()
        .for_each(|(interaction, parent)| {
            match interaction {
                Interaction::Clicked    => {
                    if let Ok(parent) = parent_name_input.get(parent.get()) {
                        // To get parent and mutually exclude the selected textbox.
                        select_name_input_parent(&parent, &mut name_input_selected);
                    }
                },
                // If Hovered or None, do nothing.
                _ => {},
            }
        });
}

/// To get color from `bool`.
///
/// Takes a `bool` value and returns a [`Color`] object.
impl FromBool for Color {
    fn from_bool(value: bool) -> Self {
        match value {
            true    => INPUT_SELECT_CLR,
            false   => TEXT_INPUT_COLOR,
        }
    }
}

/// To choose the color of the input box node.
///
/// To set the color of the selected textbox so as to identify the current selected text box. The
/// color of the selected text box is calculated by it's value.
fn name_input_color(
    input_node_query:           Query<&InputBoxNode>,
    mut text_input_query:       Query<(&mut UiColor, &Parent), (With<Button>, With<NameInput>)>,
    mut name_input_selected:    ResMut<NameInputSelected>,
) {
    // Early return if render is not `true`.
    if !name_input_selected.render { return }
    // Iterate throw the query and set the color of the textbox.
    text_input_query
        .iter_mut()
        .for_each(|(mut color, parent)| {
            if let Ok(parent) = input_node_query.get(parent.get()) {
                if let Some(name) = name_input_selected.get_mut(parent.as_usize()) {
                    *color = UiColor::from(Color::from_bool(*name));
                }
            }
        });
    // Set render as `false` to avoid processing once again.
    name_input_selected.unrender();
}

/// For selecting the input box node.
///
/// Takes the parent Node and corresponds that to the [`NameInputSelected`] value and sets the
/// selected textbox value.
fn select_name_input_parent(
    parent:                 &InputBoxNode,
    name_input_selected:    &mut ResMut<NameInputSelected>,
) {
    // To get the parent selected value and inversing it's boolean value.
    if let Some(bool_val) = name_input_selected.get_mut(parent.as_usize()) {
        *bool_val = !(*bool_val);
    } else { return }
    // Make the selected value mutually exclusive.
    name_input_selected.mutex(parent.as_usize()).render();
}

/// For typing text to the input.
///
/// Iterates through the [`RecievedCharacter`] [`EventReader`] and pushes the character values to
/// the corresponding [`NameEntryValue`]. Pops a character if __backspace__ is pressed.
fn text_typing(
    mut input:                  EventReader<ReceivedCharacter>,
    mut name_entry_value_res:   ResMut<NameEntryValue>,
    name_input_selected:        Res<NameInputSelected>,
    key_press:                  Res<Input<KeyCode>>,
) {
    // Fetching the index of the text to get NameEntryValue string.
    let Some(&(index, _)) = name_input_selected.selected
        .iter()
        .enumerate()
        .filter(|(_, name)| **name)
        .collect::<Vec<(usize, &bool)>>()
        .get(0_usize) else { return };
    let Some(name) = name_entry_value_res.players.get_mut(index) else { return };
    // To backspace if pressed.
    if key_press.just_pressed(KeyCode::Back) { name.pop(); }
    // Iter over inputting character and push to NameEntryValue.
    input
        .iter()
        .for_each(|ch| {
            if ch.char.ne(&'\u{08}') {
                name.push(ch.char);
            }
        });
}

/// To handle the display of the input text.
///
/// Gets the text from [`NameEntryValue`] and displays it to the textbox node.
fn display_text_to_input(
    name_entry_value:   Res<NameEntryValue>,
    mut text_boxes:     Query<(&mut Text, &Parent), With<NameInputText>>,
    name_input:         Query<&Parent, With<NameInput>>,
    parent_text:        Query<&InputBoxNode>,
) {
    text_boxes
        .iter_mut()
        .for_each(|(mut text_box, parent)| {
            // Get InputBoxNode.
            if let Some(text_node) = get_text_node_parent(parent, &name_input, &parent_text) {
                // Get text value from the text section.
                if let Some(text) = text_box.sections.first_mut() {
                    // Setting the text value and the bg color.
                    let name = name_entry_value.as_string(text_node.as_usize()).unwrap();
                    (text.value, text.style.color) = if name.is_empty() {
                        (String::from(TEXT_INPUT_DEF_VAL), TEXT_HOLDER_CLR)
                    } else { (name, DEFAULT_FONT_CLR) };
                }
           }
      });
}

/// To get the parent of the text node.
///
/// To get the [`NameInput`] query value corresponding to the parent and return the
/// [`InputBoxNode`] `Option` value.
fn get_text_node_parent<'a>(
    parent:         &Parent,
    name_input:     &Query<&Parent, With<NameInput>>,
    parent_text:    &'a Query<&InputBoxNode>,
) -> Option<&'a InputBoxNode> {
    // Fetching the value corresponding to the parent from the query.
    let Ok(name_input)  = name_input.get(parent.get())      else { return None };
    let Ok(text_node)   = parent_text.get(name_input.get()) else { return None };
    // Return InputBoxNode.
    Some(text_node)
}
