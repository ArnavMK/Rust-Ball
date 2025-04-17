use bevy::prelude::*;
use crate::ui::styles::*;
use crate::ui::events::*;
use crate::ui::components::*;
use crate::game::score::resources::Score;
use crate::game::player::resources::*;
use crate::game::player::events::*;

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

pub fn update_fuel_ui(
    fuel: Res<Fuel>,
    mut fuel_bar_query: Query<&mut Style, With<FuelBar>>
) {
    if let Ok(mut fuel_bar) = fuel_bar_query.get_single_mut() {
        fuel_bar.width = Val::Percent(fuel.amount);
    }
}

pub fn update_powerup_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_entity: Query<Entity, With<PowerupBar>>,
    mut powerup_collected: EventReader<PowerupCollected>,
    mut powerup_expired: EventReader<PowerupExpired>,
    active_powerups: Res<ActivePowerups>,
    powerup_symbols: Query<Entity, With<PowerupSymbol>>
) {
    if let Ok(entity) = player_entity.get_single() {

        // add a powerup symbol when powerup collected
        for event in powerup_collected.read() {
            spawn_powerup_symbol(&mut commands, &asset_server, entity, event.powerup.image_path(), event.powerup.duration());
        }
        
        // remove and redraw all powerups again when a powerup is expired
        for _ in powerup_expired.read() {
            for powerup_symbol in powerup_symbols.iter() {
                commands.entity(powerup_symbol).despawn_recursive();
            }
             
            for (powerup, ..) in active_powerups.powerups.iter() {
                spawn_powerup_symbol(&mut commands, &asset_server, entity, powerup.image_path(), powerup.duration());
            }
        }
    }
}

