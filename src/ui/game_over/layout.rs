use bevy::prelude::*;
use crate::ui::components::*;
use crate::ui::styles::*;
use crate::game::score::resources::Score;

pub fn spawn_final_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>
) {
    build_final_menue(&mut commands, &asset_server, &score);
}

pub fn despawn_final_menu(
    mut commands: Commands,
    final_menu_query: Query<Entity, With<FinalMenu>>
) {
    if let Ok(final_menu) = final_menu_query.get_single() {
       commands.entity(final_menu).despawn_recursive(); 
    }
}

pub fn build_final_menue(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    score: &Res<Score>
) {

    commands.spawn(
        full_screen_div_node(FinalMenu)
    )
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
            
            // Centre text
            parent.spawn(text_node(&asset_server, String::from("Game Over")));

            // Right hand image
            parent.spawn(default_image_builder(&asset_server, "ball_red_large.png"));
        });
        
        parent.spawn(text_node(asset_server, format!("Score: {}", score.value)));
        parent.spawn(text_node(asset_server, format!("Highest Score: {}", score.highest_score)));

        // Play button
        parent.spawn(
            button_node(PlayButton)
        ).with_children(|parent| {
            parent.spawn(text_node(&asset_server, String::from("Play Again")));
        });

        // Quit Button
        parent.spawn(
            button_node(QuitButton)
        ).with_children(|parent| {
            parent.spawn(text_node(&asset_server, String::from("Quit")));
        });
    });
}


