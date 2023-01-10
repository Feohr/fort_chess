//! dice_roll module.
//!
//! To handle the `dice roll` button plugin.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::{
    prelude::{
        Commands, Component, Query, ResMut, With, Plugin, App, UiColor, Res, Button, Interaction,
        Changed, Visibility, Entity, Text2dBundle, default, Transform, Vec2, Text, TextStyle,
        TextAlignment, Timer, Time, SystemSet,
    },
    text::Text2dBounds,
};
use crate::{
    RESOLUTION, ZAxisLevel,
    despawn_entity::DespawnEntity,
    game::GameAsset,
    listener::{
        possible_paths::{PossiblePaths, Paths},
        click::Click,
        button::{btn_spawn, style, SKIP_TURN_GAME_CLOSURES, BtnContainer},
    },
    font::{RegFontHandle, DEFAULT_FONT_CLR},
    state::FortChessState,
};
use fort_builders::{
    dice_roll,
    player::PlayerAction,
};

/// To hold the button text.
const DICE_ROLL_BTN_TEXT    : &str  = "Dice Roll";
/// Timer length
const TIMER_LEN             : f32   = 2_f32;
/// Text fadeout speed.
const FADEOUT_SPEED         : f32   = 2_f32;
/// Timer repeat.
const TIMER_REPEAT          : bool  = false;

/// To hold dice roll value.
struct DiceRollValue{
    value: usize,
    display: bool,
}
/// To hold the dice roll timer.
struct DiceRollTimer(Timer);
/// Plugin to handle `skip_turn` button.
pub(crate) struct DiceRollButtonPlugin;
/// To signify dice roll value counter.
#[derive(Component)]
struct DiceRollValueText;
/// To signify a DiceRoll Button.
#[derive(Component)]
pub(crate) struct DiceRollButton;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for DiceRollButtonPlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for DiceRollButtonPlugin {
    // The `dice roll` button was spawning before the `skip turn` button unpredictably. Hence,
    // the button spawning is pushed to on resume `BoardScreen`. Not the most elegant solution.
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(FortChessState::BoardScreen)
                .with_system(init_dice_roll_objects)
                .with_system(dice_roll_btn_spawn)
            )
            .add_system_set(
                SystemSet::on_update(FortChessState::BoardScreen)
                .with_system(dice_roll_btn_clicked   )
                .with_system(dice_roll_ui_text       )
                .with_system(clear_dice_roll_ui_text )
                .with_system(dice_roll_btn_visibility)
            );
   }
}
/*-----------------------------------------------------------------------------------------------*/
/*████Dice Roll Button Visibility████*/
/*-----------------------------------------------------------------------------------------------*/
/// Fucntion to check if the piece is at the other side of the board in enemy territory so that we
/// can make the `roll_dice` button visible.
fn dice_roll_btn_visibility(
    mut dice_roll_query:    Query<&mut Visibility, With<DiceRollButton>>,
    game:                   Res<GameAsset>,
) {
    // Matching to see if the current player piece is in opposite side.
    dice_roll_query
        .iter_mut()
        .for_each(|mut visibility| visibility.is_visible = game
                                                            .get()
                                                            .current_player()
                                                            .in_opposite_side()
                                                        && game.get().picked
        );
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Object Init████*/
/*-----------------------------------------------------------------------------------------------*/
/// Initializing dice roll object values.
fn init_dice_roll_objects(mut commands: Commands) {
    // Inserting the DiceRollValue object.
    commands.insert_resource(DiceRollValue::new());
    // Initializing the DiceRollTimer.
    commands.insert_resource(DiceRollTimer::init());
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Timer████*/
/*-----------------------------------------------------------------------------------------------*/
impl DiceRollTimer {
    /// To create a timer.
    #[inline]
    fn init() -> Self {
        DiceRollTimer(Timer::from_seconds(TIMER_LEN, TIMER_REPEAT))
    }
    /// To get a mutable reference to [`DiceRollTimer`] timer.
    #[inline]
    fn get_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
    /// To get an immutable reference to [`DiceRollTimer`] timer.
    #[inline]
    fn get(&self) -> &Timer {
        &self.0
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Value████*/
/*-----------------------------------------------------------------------------------------------*/
impl DiceRollValue {
    /// Creates a new dice roll value.
    #[inline]
    fn new() -> Self {
        DiceRollValue{
            value: usize::default(),
            display: false,
        }
    }
    /// To set the value of dice roll.
    #[inline]
    fn set(&mut self, value: usize) {
        self.value = value;
        self.display = true;
    }
    /// To set the dice roll value display as false.
    #[inline]
    fn undisplay(&mut self) {
        self.display = false;
    }
    /// To get the dice roll value.
    #[inline]
    fn get(&self) -> usize {
        self.value
    }
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Fade Out████*/
/*-----------------------------------------------------------------------------------------------*/
/// To handle the fadeout effect on the textbox.
fn clear_dice_roll_ui_text(
    mut commands:   Commands,
    mut dice_query: Query<(Entity, &mut Text), With<DiceRollValueText>>,
    mut dice_time:  ResMut<DiceRollTimer>,
    time:           Res<Time>,
) {
    // Ticking.
    dice_time.get_mut().tick(time.delta());
    dice_query
        .iter_mut()
        .for_each(|(entity, mut text)| {
            text.sections
                    .first_mut()
                    .expect("There are no text sections for dice roll value prompt").style.color
                    .set_a(dice_time.get().percent_left() * FADEOUT_SPEED);
            // Clear text when the timer is done.
            if dice_time.get().just_finished() {
                commands.entity(entity).despawn();
            }
        })
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Value Display████*/
/*-----------------------------------------------------------------------------------------------*/
/// To display the dice roll value on the screen.
fn dice_roll_ui_text(
    mut commands:           Commands,
    mut dice_roll_value:    ResMut<DiceRollValue>,
    mut dice_time:          ResMut<DiceRollTimer>,
    dice_query:             Query<Entity, With<DiceRollValueText>>,
    font:                   Res<RegFontHandle>,
) {
    if !dice_roll_value.display { return }
    // Cleaning up text so that new text can be displayed.
    commands.despawn_entity(&dice_query);
    // Dice roll timer reset.
    dice_time.get_mut().reset();
    commands.spawn_bundle(Text2dBundle {
        text_2d_bounds: Text2dBounds {
           size: Vec2::splat(3_f32 * RESOLUTION),
        },
        text: Text::from_section(
            format!("Dice Roll: {:>1?}", dice_roll_value.get()),
            TextStyle {
                font: font.get().clone(),
                font_size: 0.5_f32 * RESOLUTION,
                color: DEFAULT_FONT_CLR,
            },
        )
        .with_alignment(TextAlignment::CENTER_LEFT),
        transform: Transform::from_xyz(
                8_f32 * RESOLUTION,
                8_f32 * RESOLUTION,
                ZAxisLevel::Twelfth.as_f32(),
        ),
        ..default()
    })
    .insert(DiceRollValueText);
    // To stop from spawning more than one at a time.
    dice_roll_value.undisplay();
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Button Clicked████*/
/*-----------------------------------------------------------------------------------------------*/
/// To handle the button click interface.
fn dice_roll_btn_clicked(
    mut commands:           Commands,
    mut dice_roll_query:    Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<DiceRollButton>),
    >,
    mut game:               ResMut<GameAsset>,
    mut paths:              ResMut<PossiblePaths>,
    mut dice_roll_val:      ResMut<DiceRollValue>,
    paths_query:            Query<Entity, With<Paths>>,
    click_query:            Query<Entity, With<Click>>,
) {
    // Matching with the interaction to display the respective animations.
    dice_roll_query
        .iter_mut()
        .for_each(|(&interaction, mut color)| {
            match interaction {
                Interaction::Clicked => {
                    *color = UiColor::from(style::BTN_CLICKD_COLOR);
                    let roll = dice_roll();
                    // Set dice roll value.
                    dice_roll_val.set(roll);
                    if roll == 5_usize {
                        game.get_mut().current_player_mut().set_winner();
                        game.get_mut().set_play_false();
                    }
                    // To change the display and move the player along.
                    SKIP_TURN_GAME_CLOSURES
                        .into_iter()
                        .for_each(|game_closure| game_closure(game.get_mut()));
                    paths.clear();
                    commands.despawn_entity(&click_query);
                    commands.despawn_entity(&paths_query);
                },
                Interaction::Hovered => *color = UiColor::from(style::BTN_HOVERD_COLOR),
                Interaction::None    => *color = UiColor::from(style::BTN_BKGRND_COLOR),
            }
        });
}
/*-----------------------------------------------------------------------------------------------*/

/*████Dice Roll Button Setup████*/
/*-----------------------------------------------------------------------------------------------*/
/// To spawn a button.
#[inline]
fn dice_roll_btn_spawn(
    mut commands:   Commands,
    button:         Res<BtnContainer>,
) {
    btn_spawn( &mut commands, &button, DICE_ROLL_BTN_TEXT, DiceRollButton);
}
/*-----------------------------------------------------------------------------------------------*/
