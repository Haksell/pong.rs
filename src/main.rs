mod ball;
mod gutter;
mod paddle;
mod score;

use crate::{
    ball::{handle_collisions, move_ball, Ball, BallBundle},
    gutter::GutterBundle,
    paddle::{move_paddles, PaddleBundle},
    score::{spawn_scoreboard, update_scoreboard, Score, Scored, Scorer},
};
use bevy::prelude::*;

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
            velocity.0.y = (ball_position.0.y - position.0.y).signum();
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
