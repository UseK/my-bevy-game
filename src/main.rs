use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(GreetTime(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_arrow_ui)
        .add_systems(Update, change_ui_color)
        .add_systems(Startup, add_ball)
        .add_systems(Update, move_ball)
        .add_systems(Update, change_ball_direction)
        .add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain())
        .run();
}

const BUTTON_SIZE: Vec2 = Vec2::new(40., 40.);

fn add_arrow_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    let spawn_button = |commands: &mut Commands, x: f32, y: f32, label: &str, direction: Direction| {
        let box_position = Vec2::new(450. + x, -250. + y);
        
        commands
            .spawn((
                Button,
                direction,
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5), // Gray
                    custom_size: Some(BUTTON_SIZE),
                    ..default()
                },
                Transform::from_translation(box_position.extend(0.)),
                GlobalTransform::default(),
            ))
            .with_children(|builder| {
                // Add text as child
                builder.spawn((
                    Text2d::new(label),
                    TextFont {
                        font: font.clone(),
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_translation(Vec3::Z),
                ));
            });
    };

    spawn_button(&mut commands, 0., 0., "S", Direction::Down);
    spawn_button(&mut commands, 0., BUTTON_SIZE.y * 1.1, "W", Direction::Up);
    spawn_button(&mut commands, -BUTTON_SIZE.x * 1.1, 0., "A", Direction::Left);
    spawn_button(&mut commands, BUTTON_SIZE.x * 1.1, 0., "D", Direction::Right);
}

fn change_ui_color(
    mut query: Query<(&mut Sprite, &Direction), With<Button>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut sprite, direction) in &mut query {
        let key = match direction {
            Direction::Up => KeyCode::KeyW,
            Direction::Down => KeyCode::KeyS,
            Direction::Left => KeyCode::KeyA,
            Direction::Right => KeyCode::KeyD,
        };
        
        if keyboard_input.pressed(key) {
            sprite.color = Color::srgb(1.0, 0.843, 0.0); // Gold
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5); // Gray
        }
    }
}

#[derive(Component)]
struct Ball;

fn add_ball(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d::default());
    
    // Spawn ball as simple sprite
    commands.spawn((
        Ball,
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Gray
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Direction::Up,
    ));
}

fn move_ball(mut query: Query<(&Direction, &mut Transform), With<Ball>>) {
    for (d, mut t) in &mut query {
        match d {
            Direction::Up => t.translation.y += 1.,
            Direction::Down => t.translation.y -= 1.,
            Direction::Left => t.translation.x -= 1.,
            Direction::Right => t.translation.x += 1.,
        }
    }
}

fn change_ball_direction(
    mut query: Query<&mut Direction, With<Ball>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut change = |input: KeyCode, direction: Direction| {
        if keyboard_input.just_pressed(input) {
            for mut d in &mut query {
                let d = d.as_mut();
                *d = direction.clone();
            }
        }
    };
    change(KeyCode::KeyW, Direction::Up);
    change(KeyCode::KeyA, Direction::Left);
    change(KeyCode::KeyS, Direction::Down);
    change(KeyCode::KeyD, Direction::Right);
}

#[derive(Component, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
