use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::primitives::Circle,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};

fn main() {
    App::new()
        .insert_resource(GreetTime(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Startup, add_circle)
        .add_systems(Update, (update_people, greet_people).chain())
        .run();
}

fn add_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let circle_shape = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
    commands.spawn(MaterialMesh2dBundle {
        mesh: circle_shape,
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

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
