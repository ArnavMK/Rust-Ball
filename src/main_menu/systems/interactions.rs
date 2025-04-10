use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use crate::main_menu::events::*;

pub fn play_button_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut event: EventWriter<OnPlayButtonClicked>
) {

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {

        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                event.send(OnPlayButtonClicked);
            }
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into();}
            Interaction::None => {*background_color = DEFAULT_BUTTON_COLOR.into();}
        }

    }
}

pub fn quit_button_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut event: EventWriter<OnQuitButtonClicked>
) {

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {

        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                event.send(OnQuitButtonClicked);
            }
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into();}
            Interaction::None => {*background_color = DEFAULT_BUTTON_COLOR.into();}
        }

    }
}
