//! player name module.
//!
//! To handle the display of player names.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, ZAxisLevel, TILESIZE,
    despawn_entity::DespawnEntity,
    game::GameAsset,
    font::BoldFontHandle,
};
use bevy::{
    text::Text2dBounds,
    prelude::{
        Entity, With, Commands, Res, ResMut, Component, Query, Vec2, Transform, default,
        Text2dBundle, TextStyle, Color, Text, TextAlignment, SpriteBundle, Vec3,
        Sprite,
    }
};
use fort_builders::{
    player::Team,
    pieces::Position,
};

#[derive(Component)]
pub(crate) struct PlayerNameOutline;
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
#[inline]
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
    #[inline]
    pub(crate) fn create(name: String, team: Team, x: i32, y: i32) -> Self {
        PlayerNameTextBox {
            name,
            team,
            position: Position {x, y},
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerNameBoxVec████*/
/*-----------------------------------------------------------------------------------------------*/
impl PlayerNameBoxVec {
    /// To initialize the [`PlayerNameBoxVec`] object.
    #[inline]
    pub(crate) fn new() -> Self {
        PlayerNameBoxVec {
            boxes: Vec::new(),
        }
    }
    /// To push a [`PlayerNameTextBox`] to the vec.
    #[inline]
    pub(crate) fn push(&mut self, name: String, team: Team, x: i32, y: i32) {
        self.boxes.push(
            PlayerNameTextBox::create(name, team, x, y),
        );
    }
    /// To find and pop the [`PlayerNameTextBox`] with the corresponding team.
    #[inline]
    pub(crate) fn pop(&mut self, team: Team) {
        if let Ok(pos) = self.search(team) {
            self.boxes.remove(pos);
        }
    }
    /// To search the player with the given team using `binary search`.
    #[inline]
    pub(crate) fn search(&self, team: Team) -> Result<usize, usize> {
        self.boxes.binary_search_by(|pname| pname.team.cmp(&team))
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████PlayerName Display████*/
/*-----------------------------------------------------------------------------------------------*/
pub(crate) fn highlight_player_name(
    commands:       &mut Commands,
    player_names:   &ResMut<PlayerNameBoxVec>,
    game:           &ResMut<GameAsset>,
    pname_query:    &Query<Entity, With<PlayerNameOutline>>,
) {
    commands.despawn_entity(pname_query); // Getting the current highlighted color.
    let highlight_color = {
        let player_team = game.get().current_player().team;
        match player_names.search(player_team) {
            Ok(_)   => *color_from_team(player_team).set_a(0.5_f32),
            Err(_)  => Color::NONE,
        }
    };
    player_names.boxes
        .iter()
        .for_each(|player| {
            if game.get().current_player().team != player.team { return }
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: highlight_color,
                        custom_size: Some(Vec2::new(
                                //width.
                                TILESIZE.0 * RESOLUTION * 2_f32,
                                //height.
                                TILESIZE.1 * RESOLUTION,
                        )),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (player.position.x as f32 + 0.5_f32) * RESOLUTION,
                            player.position.y as f32 * RESOLUTION,
                            ZAxisLevel::Eleventh.as_f32(),
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(PlayerNameOutline);
        })
}

/// To display the player names onto the screen with the appropriate team color.
pub(crate) fn display_player_names(
    commands:       &mut Commands,
    player_names:   &ResMut<PlayerNameBoxVec>,
    query:          &Query<Entity, With<PlayerName>>,
    font:           &Res<BoldFontHandle>,
) {
    commands.despawn_entity(query);
    player_names.boxes
        .iter()
        .for_each(|player| {
            commands.spawn_bundle(Text2dBundle {
                text_2d_bounds: Text2dBounds {
                    size: Vec2::splat(2_f32 * RESOLUTION),
                },
                text: Text::from_section(
                    player.name.as_str(),
                    TextStyle {
                        font: font.get().clone(),
                        font_size: 0.5_f32 * RESOLUTION,
                        color: color_from_team(player.team),
                    },
                )
                .with_alignment(TextAlignment::CENTER_LEFT),
                transform: Transform::from_xyz(
                        (player.position.x as f32 - 0.3_f32) * RESOLUTION,
                        player.position.y as f32 * RESOLUTION,
                        ZAxisLevel::Twelfth.as_f32(),
                ),
                ..default()
            })
            .insert(PlayerName);
        })
}
/*-----------------------------------------------------------------------------------------------*/
