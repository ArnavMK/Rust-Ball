use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;


pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_main_menue(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu) = main_menu_query.get_single() {
       commands.entity(main_menu).despawn_recursive(); 
    }
}

pub fn build_main_menue(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {

    commands.spawn((
        NodeBundle{
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::srgba(0.0,0.0,0.0,0.0).into(),
            ..default()
        },
        MainMenu
    ))
    .with_children(|parent| {
    
        // Title
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(40.0),
                    height: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|parent| {
            

            // Left hande image
            parent.spawn(default_image_builder(&asset_server, "ball_blue_small.png"));

            parent.spawn(
                TextBundle {
                    text: Text {
                       sections: vec![
                           TextSection::new(
                               "Dogde Ball",
                               TextStyle {
                                   font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                   font_size: 64.0,
                                   color: Color::WHITE
                               }
                           )
                       ],
                       justify: JustifyText::Center,
                       ..default()
                    },
                    ..default()
                }
            );
            
            parent.spawn(default_image_builder(&asset_server, "ball_red_large.png"));
        });


        // Play button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE.clone(),
                background_color: DEFAULT_BUTTON_COLOR.into(), 
                ..default()
            },
            PlayButton
        ))
        .with_children(|parent| {
            
            parent.spawn(
                TextBundle {
                    text: Text {
                       sections: vec![
                           TextSection::new(
                               "Play",
                               TextStyle {
                                   font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                   font_size: 32.0,
                                   color: Color::WHITE
                               }
                           )
                       ],
                       justify: JustifyText::Center,
                       ..default()
                    },
                    ..default()
                }
            );
        });

        // Quit Button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE.clone(),
                background_color: DEFAULT_BUTTON_COLOR.into(), 
                ..default()
            },
            QuitButton
        ))
        .with_children(|parent| {
            
            parent.spawn(
                TextBundle {
                    text: Text {
                       sections: vec![
                           TextSection::new(
                               "Quit",
                               TextStyle {
                                   font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                   font_size: 32.0,
                                   color: Color::WHITE
                               }
                           )
                       ],
                       justify: JustifyText::Center,
                       ..default()
                    },
                    ..default()
                }
            );
        });
    }).id()

}
