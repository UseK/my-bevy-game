use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

fn main() {
    println!("Hello, grid layout");
    App::new()
        .add_plugins(grid_window_plugin())
        .add_systems(Startup, spawn_layout)
        .run();
}

fn grid_window_plugin() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: [800., 600.].into(),
            title: "Bevy CSS Grid Layout Example".to_string(),
            ..default()
        }),
        ..default()
    })
}

fn spawn_layout(mut commands: Commands, assert_server: Res<AssetServer>) {
    let font: Handle<Font> = assert_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn(Camera2dBundle::default());
}

fn entire_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_rows: vec![GridTrack::auto()],
            grid_template_columns: vec![],
            ..default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..default()
    }
}
