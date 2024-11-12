mod ball;
mod paddle;
mod score;

use ball::{handle_collisions, move_ball, Ball, BallBundle};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use paddle::{move_paddles, PaddleBundle};
use score::{spawn_scoreboard, update_scoreboard, Score, Scored, Scorer};

const GUTTER_HEIGHT: f32 = 96.;

#[derive(Component)]
struct Gutter;

#[derive(Bundle)]
struct GutterBundle {
    gutter: Gutter,
    shape: Shape,
    position: Position,
}

impl GutterBundle {
    fn new(x: f32, y: f32, w: f32) -> Self {
        Self {
            gutter: Gutter,
            shape: Shape(Vec2::new(w, GUTTER_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }

    fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Query<&Window>,
    ) {
        if let Ok(window) = window.get_single() {
            let window_width = window.resolution.width();
            let window_height = window.resolution.height();

            let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
            let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

            let top_gutter = Self::new(0., top_gutter_y, window_width);
            let bottom_gutter = Self::new(0., bottom_gutter_y, window_width);

            let shape = Rectangle::from_size(top_gutter.shape.0);
            let color = Color::srgb(0., 0., 0.);

            let mesh_handle = meshes.add(shape);
            let material_handle = materials.add(color);

            commands.spawn((
                top_gutter,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: material_handle.clone(),
                    ..default()
                },
            ));

            commands.spawn((
                bottom_gutter,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle.clone(),
                    ..default()
                },
            ));
        }
    }
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}

fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
            position.0 = Vec2::new(0., 0.);
            velocity.0.x = match event.0 {
                Scorer::Ai => -1.,
                Scorer::Player => 1.,
            };
            velocity.0.y = 1.;
        }
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        velocity.0.y = keyboard_input.pressed(KeyCode::ArrowUp) as u8 as f32
            - keyboard_input.pressed(KeyCode::ArrowDown) as u8 as f32;
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                BallBundle::spawn,
                PaddleBundle::spawn,
                GutterBundle::spawn,
                spawn_scoreboard,
                spawn_camera,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                handle_player_input,
                detect_scoring,
                move_ai,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(update_score),
                move_paddles.after(handle_player_input),
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
