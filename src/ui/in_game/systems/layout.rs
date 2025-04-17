use bevy::prelude::*;
use crate::ui::components::*;
use crate::ui::styles::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands, 
    pause_menu_query: Query<Entity, With<PauseMenu>>
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    
    commands.spawn(full_screen_div_node(PauseMenu))
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style{
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(50.0),
                    ..default()
                },
                background_color: Color::srgba(0.0,0.0,0.0,0.0).into(),
                ..default()
            }
        )
        .with_children(|parent| {
            
            // Resume Button
            parent.spawn(button_node(ResumeButton))
            .with_children(|parent| {
                parent.spawn(text_node(&asset_server, String::from("Resume")));
            });

            // Quit button
            
            parent.spawn(button_node(QuitButton))
            .with_children(|parent| {
                parent.spawn(text_node(&asset_server, String::from("Quit")));
            });

        });

    });
}


pub fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_score_ui(&mut commands, &asset_server);
}

pub fn despawn_score_ui(
    mut commands: Commands,
    score_ui_query: Query<Entity, With<ScoreDisplay>>
) {
    if let Ok(score_ui_entity) = score_ui_query.get_single() {
        commands.entity(score_ui_entity).despawn_recursive();
    }
}

fn build_score_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                width: Val::Auto,
                height: Val::Auto,
                margin: UiRect {top: Val::Px(10.0), right: Val::Px(10.0), bottom: Val::Px(10.0), left: Val::Px(10.0)},
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        },
        ScoreDisplay
    )).
    with_children(|parent| {
        parent.spawn(text_node_with_component(&asset_server, String::from("Score: "), ScoreDisplayText));
    });
}

pub fn spawn_countdown_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_countdown_ui(&mut commands, &asset_server);
}

pub fn despawn_countdown_ui(
    mut commands: Commands,
    countdown_ui_query: Query<Entity, With<CountdownMenu>>
) {
    if let Ok(entity) = countdown_ui_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn build_countdown_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
    commands.spawn(
        full_screen_div_node(CountdownMenu)
    )
    .with_children(|parent| {
        parent.spawn(text_node_with_component(asset_server, String::from("4"), CountdownText));
    });
}

pub fn spawn_fuel_ui(
    mut commands: Commands,
) {
    build_fuel_ui(&mut commands);
}

pub fn despawn_fuel_ui(
    mut commands: Commands,
    fuel_ui_query: Query<Entity, With<FuelUi>>
) {
    if let Ok(fuel_ui) = fuel_ui_query.get_single() {
        commands.entity(fuel_ui).despawn_recursive();
    }
}

fn build_fuel_ui(
    commands: &mut Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
        FuelUi
    )).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0), 
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.5, 0.5, 0.0).into(),
                    ..default()
                },
                FuelBar,
            ));
        });
    });
}

pub fn spawn_powerup_ui(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Auto,
                height: Val::Px(50.0),
                top: Val::Px(10.0),
                left: Val::Percent(45.0),
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        PowerupBar
    ));
}

pub fn despawn_powerup_ui(mut commands: Commands, powerup_bar_query: Query<Entity, With<PowerupBar>>) {
    if let Ok(entity) = powerup_bar_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

