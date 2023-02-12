//! expand module.
//!
//! Module to handle the expanding input fields.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

pub(crate) mod style {
    use bevy::prelude::Color;
    /// When expand button is clicked.
    pub(crate) const EXPAND_CLICK:          Color = Color::DARK_GRAY;
    /// When expand button is hovered.
    pub(crate) const EXPAND_HOVER:          Color = Color::GRAY;
    /// When expand is idle.
    pub(crate) const EXPAND_NORML:          Color = Color::SILVER;
}

use bevy::prelude::{
    Handle, Image, Component, Interaction, Query, With, Changed, Plugin, App, SystemSet,
    Visibility, Button, Res, StartupStage, AssetServer, UiImage, Commands, ResMut, UiColor,
};
use crate::{
    FortChessState,
    startscreen::{
        NameEntryValue,
        expand::style::{EXPAND_CLICK, EXPAND_HOVER, EXPAND_NORML},
    },
};

/// Resource to access the expand button image.
pub(crate) struct ExpandBtnImage {
    /// Stores the opened button image.
    pub(crate) open: Handle<Image>,
    /// Stores the closed button image.
    pub(crate) close: Handle<Image>,
}
/// To recognise the respective text input.
#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub(crate) enum TextInputId {
    One,
    Two,
}
/// To signify expand input button component.
#[derive(Component)]
pub(crate) struct ExpandTextInputButton {
    pub(crate) id: TextInputId,
    pub(crate) expanded: bool,
}
/// To recogrise what [`TextInputId`] of the input box is.
#[derive(Component)]
pub(crate) struct InputBoxNode {
    pub(crate) expandable: bool,
    pub(crate) id: TextInputId,
}
/// Plugin to handle [`ExpandTextInputButton`].
pub(crate) struct ExpandTextInputButtonPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for ExpandTextInputButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for ExpandTextInputButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_expand_input_btn_res)
            .add_system_set(
                SystemSet::on_update(FortChessState::StartScreen)
                .with_system(expand_btn_click   )
                .with_system(input_toggle       )
            )
            .add_system_set(
                SystemSet::on_exit(FortChessState::StartScreen)
                .with_system(deallocate_expand_input_btn_res)
            );
    }
}
/*-----------------------------------------------------------------------------------------------*/

impl TextInputId {
    /// To get [`TextInputId`] as `usize` value.
    #[inline]
    pub(crate) fn as_usize(&self) -> usize {
        match self {
            TextInputId::One => 0_usize,
            TextInputId::Two => 1_usize,
        }
    }
}

impl InputBoxNode {
    /// To get the [`InputBoxNode`] value as `usize` value in order to identify the text box.
    #[inline]
    pub(crate) fn as_usize(&self) -> usize {
        self.id.as_usize()
        +   (if self.expandable {
            2_usize
        } else {
            0_usize
        })
    }
}

/*████ExpandBtnImage████*/
/*-----------------------------------------------------------------------------------------------*/
/// To store the button image resources when we enter the screen.
#[inline]
fn insert_expand_input_btn_res(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ExpandBtnImage {
        open:   asset_server.load("spritesheet/expand.png"  ),
        close:  asset_server.load("spritesheet/unexpand.png"),
    });
}

/// To deallocate button image resource when we leave the screen.
#[inline]
fn deallocate_expand_input_btn_res(mut commands: Commands) {
    commands.remove_resource::<ExpandBtnImage>();
}
/*-----------------------------------------------------------------------------------------------*/

/*████Expand Button Click████*/
/*-----------------------------------------------------------------------------------------------*/
/// To process the expand button click.
///
/// When the mouse is hovered, or does not hover, the only difference is change in color of the
/// button being clicked.
pub(crate) fn expand_btn_click(
    icons:                  Res<ExpandBtnImage>,
    mut names:              ResMut<NameEntryValue>,
//    mut input_box:          ResMut<InputBoxTimers>,
    mut expand_text_query:  Query<
        (&Interaction, &mut UiColor, &mut UiImage, &mut ExpandTextInputButton),
        (Changed<Interaction>, With<Button>, With<ExpandTextInputButton>),
    >,
) {
    expand_text_query
        .iter_mut()
        .for_each(|(&interaction, mut color, mut image, mut expand)| {
            match interaction {
                Interaction::Clicked    => {
                    // Expand and unexpand button switch.
                    expand.expanded = !expand.expanded;
                    reset_input_str(&expand.id, &mut names);
                    *color = UiColor::from(EXPAND_CLICK);
                    *image = expand_btn_icon_select(expand.expanded, &icons);
                },
                Interaction::Hovered    => *color = UiColor::from(EXPAND_HOVER),
                Interaction::None       => *color = UiColor::from(EXPAND_NORML),
            }
        });
}

/// To reset the input string value.
///
/// Takes [`TextInputId`] value and maps to the respective string buffer which is to be cleared.
#[inline]
fn reset_input_str(
    id:     &TextInputId,
    names:  &mut ResMut<NameEntryValue>,
) {
    (
        match id {
            TextInputId::One    => names.players.get_mut(2_usize),
            TextInputId::Two    => names.players.get_mut(3_usize),
        }
    )
    .unwrap_or(&mut String::new())
    .clear()
}

/// Changes the expand button icon.
///
/// Takes a boolean value and returns the appropriate [`UiImage`] to show if the button is expanded
/// or not.
#[inline]
fn expand_btn_icon_select(
    expanded:   bool,
    icon:       &Res<ExpandBtnImage>,
) -> UiImage {
    match expanded {
        true    => UiImage(icon.close.clone()),
        false   => UiImage(icon.open.clone()),
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Input toggle████*/
/*-----------------------------------------------------------------------------------------------*/
/// To make the Input UI visibile or invisible.
///
/// To toggle the input when the box is pressed. Iterates through the box query and makes the
/// corresponding [`InputBoxNode`] `Visible`.
pub(crate) fn input_toggle(
    mut box_query:  Query<(&mut Visibility, &InputBoxNode), With<InputBoxNode>>,
    btn_query:      Query<&ExpandTextInputButton, (With<Button>, With<ExpandTextInputButton>)>,
) {
    // Iterating over input_boxes.
    box_query
        .iter_mut()
        .for_each(|(mut visibility, input_box)| {
            // Iterating over the button queries.
            btn_query
                .iter()
                .for_each(|btn| {
                    // If the input box is expandable and button id correspond then make the input
                    // box visible/invisible.
                    if input_box.expandable && btn.id.eq(&input_box.id) {
                        visibility.is_visible = btn.expanded;
                    }
                });
        });
}
/*-----------------------------------------------------------------------------------------------*/
