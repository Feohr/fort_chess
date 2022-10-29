use bevy::prelude::{
    Component, Plugin, Entity, With, Query, EventReader, Res, Commands, CursorMoved,
    SpriteBundle, Transform, App, Windows, Sprite, default, Color, Vec2, Vec3,
};
use fort_builders::{
    board::{
        LFT, BTM, get_full_width, get_full_height, in_pos},
};
use crate::{TILESIZE, RESOLUTION};
use crate::game::GameAsset;

#[derive(Component)]
struct Picker;

pub struct ListenerPlugin;
impl Plugin for ListenerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system(listener);
    }
}

fn get_cursor_pos(c_x: f32, c_y: f32, height: f32, width: f32) -> (f32, f32) {
    (
        (((c_x / width) * get_full_width()) + (LFT as f32)).round(),
        (((c_y / height) * get_full_height()) + (BTM as f32)).round(),
    )
}

fn clear_picker(
    commands: &mut Commands,
    pickers: &Query<Entity, With<Picker>>,
) {
    if let Ok(picker) = pickers.get_single() {
        commands.entity(picker).despawn();
    }
}

fn listener(
    mut commands: Commands,
    mut events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    pickers: Query<Entity, With<Picker>>,
    game: Res<GameAsset>,
) {
    if events.is_empty() { return }
    if let Some(window) = windows.get_primary() {
        for cursor in events.iter() {
            let (m_x, m_y) = get_cursor_pos(
                cursor.position.x,
                cursor.position.y,
                window.height(),
                window.width(),
            );
            clear_picker(&mut commands, &pickers);
            if in_pos(m_x, m_y, game.get().len()) {
                commands.spawn().insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.5, 0.4, 1.0, 0.9),
                        custom_size: Some(Vec2::new(
                                (TILESIZE.0 - 0.1) * RESOLUTION,
                                (TILESIZE.1 - 0.1) * RESOLUTION,
                        )),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            m_x * RESOLUTION,
                            m_y * RESOLUTION,
                            6.0,
                        ),
                        ..default()
                    },
                    ..default()
                }).insert(Picker);
            }
            break;
        }
    }
    events.clear();
}
