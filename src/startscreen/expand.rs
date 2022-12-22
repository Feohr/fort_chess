/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::*;

/// When expand button is clicked.
const EXPAND_CLICK: Color = Color::DARK_GRAY;
/// When expand button is hovered.
const EXPAND_HOVER: Color = Color::GRAY;
/// When expand is idle.
pub(crate) const EXPAND_NORML: Color = Color::SILVER;

/// To signify expand input button component.
#[derive(Component)]
pub(crate) struct ExpandTextInputButton;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

pub(crate) fn expand_btn_click(
    asset_server:           Res<AssetServer>,
    mut expand_text_query:  Query<
        (&Interaction, &mut UiColor, &mut UiImage),
        (Changed<Interaction>, With<Button>, With<ExpandTextInputButton>),
    >,
) {
    expand_text_query
        .iter_mut()
        .for_each(|(&interaction, mut color, mut image)| {
            match interaction {
                Interaction::Clicked    => {
                    *color = UiColor::from(EXPAND_CLICK);
                    *image = UiImage(asset_server.load("spritesheet/unexpand.png"));
                },
                Interaction::Hovered    => *color = UiColor::from(EXPAND_HOVER),
                Interaction::None       => *color = UiColor::from(EXPAND_NORML),
            }
        });
}
