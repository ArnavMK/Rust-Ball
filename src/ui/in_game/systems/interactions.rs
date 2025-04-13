use bevy::prelude::*;
use crate::ui::styles::*;
use crate::ui::events::*;
use crate::ui::components::*;
use crate::game::score::resources::Score;

pub fn resume_button_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ResumeButton>)>,
    mut event: EventWriter<OnResumeButtonClicked>
) {

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {

        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                event.send(OnResumeButtonClicked);
            }
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into();}
            Interaction::None => {*background_color = DEFAULT_BUTTON_COLOR.into();}
        }

    }
}

pub fn quit_button_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut event: EventWriter<AppExit>
) {

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {

        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                event.send(AppExit::Success);
            }
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into();}
            Interaction::None => {*background_color = DEFAULT_BUTTON_COLOR.into();}
        }
    }
}

pub fn update_score_ui(
    mut score_text_query: Query<&mut Text, With<ScoreDisplayText>>,
    score: Res<Score>
) {
    if let Ok(mut text) = score_text_query.get_single_mut() {
        text.sections[0].value = format!("Score: {}", score.value);
    }
}
