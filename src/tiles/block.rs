/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{
    RESOLUTION, TILESIZE, ZAxisLevel,
    state::FortChessState,
    startscreen::NameEntryValue,
};
use bevy::prelude::{
    default, App, Commands, Entity, Plugin, Transform, Vec2, Vec3, Color, Component, Res, Sprite,
    SpriteBundle, SystemSet,
};
use fort_builders::{
    BREADTH,
    board::{X_MAX, Y_MAX},
};

/// To hold the quadrant block color,
const BLOCK_COLOR: Color = Color::rgba(0.1_f32, 0.1_f32, 0.1_f32, 0.95_f32);

/// To identify block [`Component`].
#[derive(Component)]
pub(crate) struct Blocker;
/// To hold the quadrant block plugin.
pub(crate) struct FortBlockPlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for FortBlockPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for FortBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::BoardScreen)
                .with_system(muteblockq2)
                .with_system(muteblockq3)
            );
   }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Quadrant Blocking████*/
/*-----------------------------------------------------------------------------------------------*/
/// To block the [`Q2`] if the `PLAYER_COUNT` is less than `3`.
///
/// [`Q2`]: [`fort_builder::board::Quadrant::Q2`]
fn muteblockq2(
    mut commands:           Commands,
    name_entry_value_res:   Res<NameEntryValue>,
) {
    // If player count is greater than or equal to `3`.
    if name_entry_value_res.count() >= 3_usize { return }
    (-BREADTH..BREADTH)
        .into_iter()
        .for_each(|x| {
            (BREADTH..Y_MAX)
                .into_iter()
                .for_each(|y| {
                    let tile = spawn_block_sprite(
                        &mut commands,
                        BLOCK_COLOR,
                        Vec3::new(
                            x as f32 * RESOLUTION,
                            y as f32 * RESOLUTION,
                            ZAxisLevel::Fourteenth.as_f32(),
                        ),
                    );
                    commands.entity(tile).insert(Blocker);
                })
        })
}

/// To block the [`Q3`] if the `PLAYER_COUNT` is less than `4`.
///
/// [`Q3`]: [`fort_builder::board::Quadrant::Q3`]
fn muteblockq3(
    mut commands:           Commands,
    name_entry_value_res:   Res<NameEntryValue>,
) {
    // If player count is greater than or equal to `4` then.
    if name_entry_value_res.count() >= 4_usize { return }
    (BREADTH..X_MAX)
        .into_iter()
        .for_each(|x| {
            (-BREADTH..BREADTH)
                .into_iter()
                .for_each(|y| {
                    let tile = spawn_block_sprite(
                        &mut commands,
                        BLOCK_COLOR,
                        Vec3::new(
                            x as f32 * RESOLUTION,
                            y as f32 * RESOLUTION,
                            ZAxisLevel::Fourteenth.as_f32(),
                        ),
                    );
                    commands.entity(tile).insert(Blocker);
                })
        })
}
/*-----------------------------------------------------------------------------------------------*/

/// To return a block sprite when called,
fn spawn_block_sprite(
    commands:       &mut Commands,
    color:          Color,
    translation:    Vec3,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(
                        //width.
                        TILESIZE.0 * RESOLUTION,
                        //height.
                        TILESIZE.1 * RESOLUTION,
                )),
                ..default()
            },
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()
}
