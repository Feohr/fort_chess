//! player name module.
//!
//! To handle the display of player names.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, ZAxisLevel};
use bevy::{
    text::Text2dBounds,
    prelude::{
        Entity, With, Commands, Res, ResMut, Component, Query, Vec3, Vec2, Transform, default, 
        AssetServer, Text2dBundle, TextStyle, Color, Text, TextAlignment,
    }
};
use fort_builders::{
    player::Team,
    pieces::Position,
};

/// To hold the data of each player box.
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
pub(crate) struct PlayerNameTextBox {
    name:       String,
    team:       Team,
    position:   Position,
}

/// Parent object to hold the [`PlayerNameTextBox`] vec.
#[derive(Debug)]
pub(crate) struct PlayerNameBoxVec {
    boxes: Vec<PlayerNameTextBox>,
}

/// To denote a player name box entity.
#[derive(Component)]
pub(crate) struct PlayerName;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// Function to convert [`Team`] to [`Color`].
#[inline(always)]
fn color_from_team(team: Team) -> Color {

    match team {
        Team::Red       => Color::RED,
        Team::Green     => Color::GREEN,
        Team::Blue      => Color::BLUE,
        Team::Yellow    => Color::YELLOW,
    }

}

/*████PlayerNameTextBox████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerNameTextBox {

    /// To create a [`PlayerNameTextBox`].
    #[inline(always)]
    pub(crate) fn create(name: String, team: Team, x: i32, y: i32) -> Self {
        PlayerNameTextBox {
            name,
            team,
            position: Position { x, y },
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerNameBoxVec████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerNameBoxVec {

    /// To initialize the [`PlayerNameBoxVec`] object.
    #[inline(always)]
    pub(crate) fn new() -> Self {
        PlayerNameBoxVec {
            boxes: Vec::new(),
        }
    }

    /// To push a [`PlayerNameTextBox`] to the vec.
    pub(crate) fn push(&mut self, name: String, team: Team, x: i32, y: i32) {
        self.boxes.push(PlayerNameTextBox::create(name, team, x, y))
    }

    /// To find and pop the [`PlayerNameTextBox`] with the corresponding team.
    pub(crate) fn pop(&mut self, team: Team) {

        if let Ok(pos) =    self.boxes
                                .binary_search_by(|pname|
                                        pname.team.cmp(&team)
                                )
        {
            self.boxes.remove(pos);
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerName Display████*/
/*-----------------------------------------------------------------------------------------------*/
/// To despawn and clean up `PlayerName` entities.
fn clear_player_names(
    commands:   &mut Commands,
    query:      &Query<Entity, With<PlayerName>>,
) {

    for player_name in query.iter() {
        commands.entity(player_name).despawn();
    }

}

/// To display the player names onto the screen with the appropriate team color.
pub(crate) fn display_player_names(
    commands:       &mut Commands,
    player_names:   &ResMut<PlayerNameBoxVec>,
    query:          &Query<Entity, With<PlayerName>>,
    asset_server:   &Res<AssetServer>,
) {

    // Clean up.
    clear_player_names(commands, query);

    let font = asset_server.load("fonts/fira-sans.extrabold.ttf");

    for player in player_names.boxes.iter() {

        commands.spawn_bundle(Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(2_f32 * RESOLUTION),
            },
            text: Text::from_section(
                player.name.as_str(),
                TextStyle {
                    font: font.clone(),
                    font_size: 0.5_f32 * RESOLUTION,
                    color: color_from_team(player.team),
                },
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform: Transform {
                translation: Vec3::new(
                    player.position.x as f32 * RESOLUTION,
                    player.position.y as f32 * RESOLUTION,
                    ZAxisLevel::Twelfth.as_f32(),
                ),
                ..default()
            },
            ..default()
        })
        .insert(PlayerName); 

    }

}
/*-----------------------------------------------------------------------------------------------*/
