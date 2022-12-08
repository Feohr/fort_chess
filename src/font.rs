//! Font module.
//!
//! Loads the font assets that will be used in the game.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::{Commands, Res, Handle, StartupStage, App, AssetServer, Plugin, Font};

/// Handle to get bold font.
pub(crate) struct BoldFontHandle(pub Handle<Font>);

/// Handle to get regular font.
pub(crate) struct RegFontHandle(pub Handle<Font>);

/// To handle the font loading.
pub(crate) struct FontHandlePlugin;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/*████Plugin for FontHandlePlugin████*/
/*-----------------------------------------------------------------------------------------------*/
impl Plugin for FontHandlePlugin {

    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_bold_font_handle      )
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_regular_font_handle   );
    }

}
/*-----------------------------------------------------------------------------------------------*/

/*████Bold Font Handle████*/
/*-----------------------------------------------------------------------------------------------*/
fn insert_bold_font_handle(
    mut commands: Commands,
    asset_server:   Res<AssetServer>,
) {

    commands.insert_resource(BoldFontHandle(
        asset_server.load("fonts/fira-sans.extrabold.ttf"),
    ));

}
/*-----------------------------------------------------------------------------------------------*/

/*████Regular Font Handle████*/
/*-----------------------------------------------------------------------------------------------*/
fn insert_regular_font_handle(
    mut commands: Commands,
    asset_server:   Res<AssetServer>,
) {

    commands.insert_resource(RegFontHandle(
        asset_server.load("fonts/fira-sans.regular.ttf"),
    ));

}
/*-----------------------------------------------------------------------------------------------*/
