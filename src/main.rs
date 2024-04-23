use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

#[derive(Resource)]
struct GreetTime(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Marcus".to_string())));
    commands.spawn((Person, Name("Wrench".to_string())));
    commands.spawn((Person, Name("Josh".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTime>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        query
            .into_iter()
            .for_each(|name| println!("Hello, {}", name.0));
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    query.iter_mut().for_each(|mut name| {
        if name.0 == "Marcus" {
            name.0 = "aiee".to_string();
        }
    });
}

fn main() {
    App::new()
        .insert_resource(GreetTime(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain())
        .run();
}
