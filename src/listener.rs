//! hhh

/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/
use crate::game::GameAsset;
use crate::PlayerPosition;
use crate::{RESOLUTION, TILEDRAW};
use bevy::{
    input::Input,
    prelude::{
        default, Color, Commands, Component, CursorMoved, Entity, EventReader, MouseButton, Query,
        Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Vec3, Windows, With,
    },
};
use fort_builders::{
    board::{get_cursor_pos, in_pos},
    game::GameAction,
};

// The color of the piece picker.
const PICKER_CLR: Color = Color::SILVER;
// The color of the clicked piece.
const CLICKS_CLR: Color = Color::DARK_GRAY;

#[derive(Component)]
pub struct Picker;

#[derive(Component)]
pub struct Click;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/
/*-----------------------------------------------------------------------------------------------*/

/*████Listner Logic████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_picker(commands: &mut Commands, pickers: &Query<Entity, With<Picker>>) {
    for picker in pickers.iter() {
        commands.entity(picker).despawn();
    }
}

fn mouse_listener(
    commands: &mut Commands,
    pickers: &Query<Entity, With<Picker>>,
    game: &ResMut<GameAsset>,
    m_x: f32,
    m_y: f32,
) {
    clear_picker(commands, pickers);
    if in_pos(m_x, m_y, game.get().len()) {
        if is_picker_color(m_x, m_y, &game) {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: PICKER_CLR,
                        custom_size: Some(Vec2::new(
                            TILEDRAW.0 * RESOLUTION,
                            TILEDRAW.1 * RESOLUTION,
                        )),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(m_x * RESOLUTION, m_y * RESOLUTION, 6.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Picker);
        }
    }
}

fn is_picker_color(x: f32, y: f32, game: &ResMut<GameAsset>) -> bool {
    game.get().player().search(x, y).is_ok()
}
/*-----------------------------------------------------------------------------------------------*/

/*████ClickListener Logic████*/
/*-----------------------------------------------------------------------------------------------*/
fn clear_click(commands: &mut Commands, clicks: &Query<Entity, With<Click>>) {
    for click in clicks.iter() {
        commands.entity(click).despawn();
    }
}

fn click_listener(
    commands: &mut Commands,
    click: &mut ResMut<Input<MouseButton>>,
    clicks: &Query<Entity, With<Click>>,
    game: &mut ResMut<GameAsset>,
    m_x: f32,
    m_y: f32,
) {
    if !in_pos(m_x, m_y, game.get().len()) {
        return;
    }
    if click.just_pressed(MouseButton::Left) {
        clear_click(commands, clicks);
        match game.get().picked {
            true => {
                game.get_mut().set_pick_false();
                if let Err(_) = game.get().player().search(m_x, m_y) {
                    let pos = game.get().position;
                    game.get_mut().update(m_x as i32, m_y as i32, pos).unwrap();
                    game.get_mut().next();
                    return;
                }
            }
            false => {
                if let Ok(pos) = game.get().player().search(m_x, m_y) {
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: CLICKS_CLR,
                                custom_size: Some(Vec2::new(
                                    TILEDRAW.0 * RESOLUTION,
                                    TILEDRAW.1 * RESOLUTION,
                                )),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(m_x * RESOLUTION, m_y * RESOLUTION, 7.0),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Click);
                    game.get_mut().set_piece_pos(pos);
                    game.get_mut().set_pick_true();
                }
            }
        }
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Main Listener Function████*/
/*-----------------------------------------------------------------------------------------------*/
pub fn listener(
    mut commands: Commands,
    windows: Res<Windows>,
    mut events: EventReader<CursorMoved>,
    mut click: ResMut<Input<MouseButton>>,
    pickers: Query<Entity, With<Picker>>,
    clicks: Query<Entity, With<Click>>,
    mut game: ResMut<GameAsset>,
    mut position: ResMut<PlayerPosition>,
) {
    if let Some(window) = windows.get_primary() {
        for cursor in events.iter() {
            (position.0, position.1) = get_cursor_pos(
                cursor.position.x,
                cursor.position.y,
                window.height(),
                window.width(),
            );
            break;
        }
        mouse_listener(&mut commands, &pickers, &game, position.0, position.1);
        click_listener(
            &mut commands,
            &mut click,
            &clicks,
            &mut game,
            position.0,
            position.1,
        );
    }
}
/*-----------------------------------------------------------------------------------------------*/
