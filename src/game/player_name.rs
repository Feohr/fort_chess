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

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
pub(crate) struct PlayerNameTextBox {
    name:       String,
    team:       usize,
    position:   Position,
}

#[derive(Debug)]
pub(crate) struct PlayerNameBoxVec {
    boxes: Vec<PlayerNameTextBox>,
}

#[derive(Component)]
pub(crate) struct PlayerName;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████PlayerNameTextBox████*/
/*-----------------------------------------------------------------------------------------------*/
impl<'a> PlayerNameTextBox {

    #[inline(always)]
    pub(crate) fn create(name: String, team: Team, x: i32, y: i32) -> Self {
        PlayerNameTextBox {
            name,
            team: team.as_usize(),
            position: Position { x, y },
        }
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerNameBoxVec████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerNameBoxVec {

    #[inline(always)]
    pub(crate) fn new() -> Self {
        PlayerNameBoxVec {
            boxes: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, name: String, team: Team, x: i32, y: i32) {

        self.boxes.push(PlayerNameTextBox::create(name, team, x, y))

    }

    pub(crate) fn pop(&mut self, team: Team) {

        if let Ok(pos) =    self.boxes
                                .binary_search_by(|pname|
                                        pname.team.cmp(&team.as_usize())
                                )
        {
            self.boxes.remove(pos);
        }

    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████AsColor For Team████*/
/*-----------------------------------------------------------------------------------------------*/
trait AsColor {

    fn as_color(&self) -> Color;

}

impl AsColor for Team {

    fn as_color(&self) -> Color {

        match self {
            Team::Red    => Color::RED,
            Team::Blue   => Color::BLUE,
            Team::Green  => Color::GREEN,
            Team::Yellow => Color::YELLOW,
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
    asset:          &Res<AssetServer>,
) {

    // Clean up.
    clear_player_names(commands, query);

    let font = asset.load("fonts/fira-sans.extrabold.ttf");

    for player in player_names.boxes.iter() {

        commands.spawn_bundle(Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(2.0 * RESOLUTION),
            },
            text: Text::from_section(
                player.name.as_str(),
                TextStyle {
                    font: font.clone(),
                    font_size: 0.5 * RESOLUTION,
                    color: Team::from_index(player.team).unwrap().as_color(),
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
