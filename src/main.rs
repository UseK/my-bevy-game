use bevy::{
    prelude::*,
    render::color,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    text::Text2dBounds,
};

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

#[derive(Component)]
struct ButtonUI;
const BUTTON_SIZE: Vec2 = Vec2::new(40., 40.);

fn add_arrow_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    fn spawn_button(
        commands: &mut Commands,
        x: f32,
        y: f32,
        text: &str,
        direction: Direction,
        font: &Handle<Font>,
    ) {
        commands
            .spawn((ButtonUI, button_sprite_bundle(x, y), direction))
            .with_children(|builder| {
                builder.spawn(button_text_2d_bundle(text, font.clone()));
            });
    }
    fn button_sprite_bundle(x: f32, y: f32) -> SpriteBundle {
        let box_position = Vec2::new(450. + x, -250. + y);
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(BUTTON_SIZE),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.)),
            ..default()
        }
    }
    fn button_text_2d_bundle(text: &str, font: Handle<Font>) -> Text2dBundle {
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 60.0,
            ..default()
        };
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(text, text_style.clone())],
                justify: JustifyText::Center,
                ..default()
            },
            text_2d_bounds: Text2dBounds { size: BUTTON_SIZE },
            transform: Transform::from_translation(Vec3::Z),
            ..default()
        }
    }
    spawn_button(&mut commands, 0., 0., "S", Direction::Down, &font);
    spawn_button(
        &mut commands,
        0.,
        BUTTON_SIZE.y * 1.1,
        "W",
        Direction::Up,
        &font,
    );
    spawn_button(
        &mut commands,
        -BUTTON_SIZE.x * 1.1,
        0.,
        "A",
        Direction::Left,
        &font,
    );
    spawn_button(
        &mut commands,
        BUTTON_SIZE.x * 1.1,
        0.,
        "D",
        Direction::Right,
        &font,
    );
}

fn change_ui_color(
    mut query: Query<(&mut Sprite, &Direction), With<ButtonUI>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let change_color_only_when_pressed = |mut sprite: Mut<Sprite>, key: KeyCode| {
        if keyboard_input.pressed(key) {
            sprite.color = Color::GOLD
        } else {
            sprite.color = Color::GRAY
        }
    };
    for (sprite, direction) in &mut query {
        match direction {
            Direction::Up => change_color_only_when_pressed(sprite, KeyCode::KeyW),
            Direction::Down => change_color_only_when_pressed(sprite, KeyCode::KeyS),
            Direction::Left => change_color_only_when_pressed(sprite, KeyCode::KeyA),
            Direction::Right => change_color_only_when_pressed(sprite, KeyCode::KeyD),
        }
    }
}

#[derive(Component)]
struct Ball;

fn add_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let circle_shape = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: circle_shape,
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
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
