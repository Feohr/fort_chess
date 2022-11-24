//! skip_turn module.
//!
//! To handle the `skip_turn` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

mod button_style {

    use bevy::prelude::Color;

    /// Color of the button background.
    pub(crate) const BTN_BKGRND_COLOR  : Color      =   Color::rgba(
                                                            0.85_f32, 0.85_f32, 0.85_f32, 0.8_f32,
                                                        );
    /// Color of button hovered.
    pub(crate) const BTN_HOVERD_COLOR  : Color      =   Color::rgba(
                                                            0.75_f32, 0.75_f32, 0.75_f32, 0.8_f32,
                                                        );
    /// Color of button clicked.
    pub(crate) const BTN_CLICKD_COLOR  : Color      =   Color::rgba(
                                                            0.15_f32, 0.15_f32, 0.15_f32, 0.8_f32,
                                                        );
    /// Color of the button text.
    pub(crate) const BTN_FGTEXT_COLOR  : Color      =   Color::WHITE;
    /// Color of the background node.
    pub(crate) const BTN_NODE_COLOR    : Color      =   Color::NONE;
    /// Button size.
    pub(crate) const BTN_SIZE          : (f32, f32) =   (153_f32, 51_f32);
    /// Button font size.
    pub(crate) const BTN_FONT_SIZE     : f32        =   28_f32;
    /// To hold the button text.
    pub(crate) const BTN_TEXT          : &str       =   "Skip Turn";

}

use bevy::{
    prelude::{
        Commands, Query, ResMut, With, Plugin, App, AlignItems, Val, JustifyContent,
        default, ButtonBundle, NodeBundle, Style, Size, TextStyle, TextBundle, AssetServer,
        UiColor, Res, UiRect, Button, Interaction, Changed,
    },
    hierarchy::{BuildChildren, ChildBuilder},
};
use crate::{
    RESOLUTION,
    game::GameAsset,
};
use fort_builders::game::GameAction;

/// Plugin to handle `skip_turn` button.
pub(crate) struct SkipButtonPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for SkipButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for SkipButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_startup_system(skip_turn_btn_node_spawn)
            .add_system(        skip_turn_btn_clicked   );
    }
}
/*-----------------------------------------------------------------------------------------------*/

fn skip_turn_btn_clicked(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game: ResMut<GameAsset>,
) {

    for (&interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor::from(button_style::BTN_CLICKD_COLOR);
                game.get_mut().next_player();
                game.get_mut().set_update_true();
            },
            Interaction::Hovered => *color = UiColor::from(button_style::BTN_HOVERD_COLOR),
            Interaction::None    => *color = UiColor::from(button_style::BTN_BKGRND_COLOR),
        }
    }

}

/*████Skip Turn Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/

fn skip_turn_btn_text_spawn(
    commands:       &mut ChildBuilder,
    asset_server:   &Res<AssetServer>,
) {

    commands.spawn_bundle(TextBundle::from_section(
        button_style::BTN_TEXT,
        TextStyle {
            font:       asset_server.load("fonts/fira-sans.extrabold.ttf"),
            font_size:  button_style::BTN_FONT_SIZE,
            color:      button_style::BTN_FGTEXT_COLOR,
        },
    ));

}

fn skip_turn_btn_bg_spawn(
    commands:       &mut ChildBuilder,
    asset_server:   &Res<AssetServer>,
) {

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(
                          Val::Px(button_style::BTN_SIZE.0),
                          Val::Px(button_style::BTN_SIZE.1),
                ),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor::from(button_style::BTN_BKGRND_COLOR),
            image: asset_server.load("spritesheet/skip_turn_button.png").into(),
            ..default()
        })
        .with_children(|parent| skip_turn_btn_text_spawn(parent, &asset_server));

}

fn skip_turn_btn_node_spawn(
    mut commands:   Commands,
    asset_server:   Res<AssetServer>,
) {

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(
                        Val::Px(button_style::BTN_SIZE.0),
                        Val::Px(button_style::BTN_SIZE.1),
                ),
                position: UiRect::new(
                    Val::Px(4.2_f32 * RESOLUTION.round()),
                    Val::Auto,
                    Val::Auto,
                    Val::Px(8_f32 * RESOLUTION.round()),
                ),
                ..default()
            },
            color: UiColor::from(button_style::BTN_NODE_COLOR),
            ..default()
        })
        .with_children(|parent| skip_turn_btn_bg_spawn(parent, &asset_server));

}
/*-----------------------------------------------------------------------------------------------*/
