/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION,
    game::GameAsset,
    font::BoldFontHandle,
    state::FortChessState,
};
use bevy::{
    prelude::{
        Commands, Res, ResMut, Vec2, App, Plugin, SystemSet, SpriteBundle, Transform, Color,
        TextAlignment, Text2dBundle, Text, TextStyle, Sprite, default, BuildChildren, Component,
        Time, Timer, Query, With, Children, KeyCode, State, Input,
    },
    text::Text2dBounds,
};
use fort_builders::{
    exit, Error,
    board::{XMINF, YMAXF},
    player::Player,
    game::Game,
};

/// Fade out speed.
const FADEOUT_SPEED: f32 = 2_f32;
/// color of the background screen.
const RES_BKGRND_COLOR: Color = Color::CYAN;

pub(crate) struct GameEndPlugin;

#[derive(Component)]
struct GameResultComponent;

#[derive(Component)]
struct GameResult {
    result: String,
    draw: bool,
    fade: Timer,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for GameEndPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for GameEndPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(FortChessState::BoardScreen)
                .with_system(jump_to_end_screen)
            )
            .add_system_set(
                SystemSet::on_exit(FortChessState::BoardScreen)
                .with_system(game_result)
            )
            .add_system_set(
                SystemSet::on_enter(FortChessState::ResultScreen)
                .with_system(display_winner)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::ResultScreen)
                .with_system(fade_in_result)
                .with_system(jump_to_end_screen)
            );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████GameResult████*/
/*-----------------------------------------------------------------------------------------------*/
impl GameResult {

    /// To get game result from player values.
    fn from(value: Result<Option<Player>, Error>) -> Self {
        match value {
            Ok(player) =>   GameResult {
                                result: format!("{} Won",
                                        player.unwrap_or(Player {
                                                name: "Nobody".to_string(),
                                                ..default()
                                        })
                                        .name
                                ),
                                draw: true,
                                fade: Timer::from_seconds(4_f32, false),
            },
            // Throw panic if there are more than one winners.
            Err(err)  => panic!("{error:?}: {error}", error = err),
        }
    }

    /// To check if the game result has been drawn already.
    fn set_draw_false(&mut self) {
        self.draw = false;
    }

}

/// To create [`GameResult`] from [`GameAsset`].
fn game_result(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
) {

    let exit_game = exit(
        std::mem::replace(game.get_mut(), Game::default())
    );

    // Dealloc.
    commands.remove_resource::<GameAsset>();
    commands.insert_resource(GameResult::from(exit_game));

}
/*-----------------------------------------------------------------------------------------------*/

fn fade_in_result(
    mut result_obj:     ResMut<GameResult>,
    time:               Res<Time>,
    mut query:          Query<(&mut Sprite, &mut Children), With<GameResultComponent>>,
    mut query_child:    Query<&mut Text>,
) {

    result_obj.fade.tick(time.delta());

    if result_obj.fade.finished() { return }

    query
        .iter_mut()
        .for_each(|(mut sprite, child)| {
            let alpha = result_obj.fade.percent() * FADEOUT_SPEED;
            sprite.color.set_a(alpha);
            child
                .iter()
                .for_each(|text| {
                    let mut text_var = query_child.get_mut(*text).unwrap();
                    text_var.sections
                        .first_mut()
                        .unwrap()
                        .style.color
                        .set_a(alpha);
                });
        });

}

// Tmp test function to get the result screen.
fn jump_to_end_screen(
    mut state:      ResMut<State<FortChessState>>,
    key:            Res<Input<KeyCode>>,
) {

    if key.just_pressed(KeyCode::R) {
        let _throw = state.set(FortChessState::ResultScreen);
    }

}

fn display_winner(
    mut commands:       Commands,
    font:               Res<BoldFontHandle>,
    mut game_result:    ResMut<GameResult>,
) {

    if !game_result.draw { return }

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: RES_BKGRND_COLOR,
            custom_size: Some(Vec2::splat(10000_f32)), // Needs to stretch the whole screen.
            ..default()
        },
        transform: Transform::from_xyz(
            0_f32,
            0_f32,
            20_f32,
        ),
        ..default()
    })
    .with_children(|commands| {
        commands.spawn_bundle(Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(20_f32 * RESOLUTION),
            },
            text: Text::from_section(
                game_result.result.clone(),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 2_f32 * RESOLUTION,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform: Transform::from_xyz(
                XMINF * RESOLUTION,
                (YMAXF / 2_f32) * RESOLUTION,
                21_f32,
            ),
            ..default()
        });
    })
    .insert(GameResultComponent);

    game_result.set_draw_false();

}
