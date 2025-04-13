
use bevy::prelude::*;
use once_cell::sync::Lazy;

pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const CLICKED_BUTTON_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.26, 0.26, 0.26);


pub static BUTTON_STYLE: Lazy<Style> = Lazy::new(|| Style {
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    margin: UiRect::all(Val::Px(10.0)),
    ..default()
});
 
pub fn default_image_builder(asset_server: &Res<AssetServer>, image: &str) -> ImageBundle {

    ImageBundle {
        style: Style {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            margin: UiRect{
                left: Val::Px(8.0),
                right: Val::Px(8.0),
                top: Val::Px(8.0),
                bottom: Val::Px(8.0)
            },
            ..default()
        },
       image: UiImage {
           texture: asset_server.load(format!("sprites/{}", image)),
           ..default()
        },
        ..default()
    }
}

pub fn text_node(asset_server: &Res<AssetServer>, text: String) -> TextBundle {

    TextBundle {
        text: Text {
           sections: vec![
               TextSection::new(
                   text,
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
}

pub fn text_node_with_component<T: Component>(asset_server: &Res<AssetServer>, text: String, component: T) -> (TextBundle, T) {
    (text_node(asset_server, text), component)
}

pub fn button_node<T: Component>(component: T) -> (ButtonBundle, T){
    (
        ButtonBundle {
        style: BUTTON_STYLE.clone(),
        background_color: DEFAULT_BUTTON_COLOR.into(), 
        border_radius: BorderRadius::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
        ..default()
        },
        component
    )
}

pub fn full_screen_div_node<T: Component>(component: T) -> (NodeBundle, T) {
    (
        NodeBundle{
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::srgba(0.1,0.1,0.1,0.9).into(),
            ..default()
        },
        component
    )
}
