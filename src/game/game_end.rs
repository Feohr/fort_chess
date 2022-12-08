/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, ZAxisLevel,
    game::GameAsset,
    font::BoldFontHandle,
    state::FortChessState,
};
use bevy::{
    prelude::{
        Commands, Res, ResMut, Vec2, App, Plugin, SystemSet, SpriteBundle, Transform, Color,
        TextAlignment, Text2dBundle, Text, TextStyle, Sprite, default, BuildChildren, Component,
    },
    text::Text2dBounds,
};
use fort_builders::{exit, Error, player::Player, game::Game};

pub(crate) struct GameEndPlugin;

#[derive(Component)]
struct GameResult {
    result: String,
    draw: bool,
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl Plugin for GameEndPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_exit(FortChessState::BoardScreen)
                .with_system(game_result)
            )
            .add_system_set(
                SystemSet::on_enter(FortChessState::ResultScreen)
               .with_system(display_winner)
            );
    }

}

impl GameResult {

    fn from(value: Result<Option<Player>, Error>) -> Self {
        if let Ok(player) = value {
            let player_name = match player.is_none() {
                true  => "Draw".to_owned(),
                false => format!("Winner: {:?}", player.unwrap().name),
            };
            return  GameResult {
                        result: player_name,
                        draw: true,
                    };
        }
        panic!("{:?}", value);
    }

    fn set_draw_false(&mut self) {
        self.draw = false;
    }

}

fn game_result(
    mut commands:   Commands,
    mut game:       ResMut<GameAsset>,
) {

    let exit = exit(
        std::mem::replace(game.get_mut(), Game::default())
    );
    println!("{:#?}", exit);

    commands.remove_resource::<GameAsset>();
    commands.insert_resource(GameResult::from(exit));

}

fn display_winner(
    mut commands:       Commands,
    font:               Res<BoldFontHandle>,
    mut game_result:    ResMut<GameResult>,
) {

    if !game_result.draw { return }

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(Vec2::new(20_f32, 30_f32)),
            ..default()
        },
        transform: Transform::from_xyz(
            0_f32,
            0_f32,
            ZAxisLevel::Fifteenth.as_f32(),
        ),
        ..default()
    })
    .with_children(|commands| {
        commands.spawn_bundle(Text2dBundle {
            text_2d_bounds: Text2dBounds {
                size: Vec2::splat(2_f32 * RESOLUTION),
            },
            text: Text::from_section(
                game_result.result.clone(),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 0.5_f32 * RESOLUTION,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform: Transform::from_xyz(
                0_f32,
                0_f32,
                ZAxisLevel::Twelfth.as_f32(),
            ),
            ..default()
        });
    });

    game_result.set_draw_false();

}
