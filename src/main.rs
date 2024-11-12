use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume as _, IntersectsVolume as _},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

const BALL_SIZE: f32 = 5.;

const PADDLE_SPEED: f32 = 1.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(x, y)),
        }
    }
}

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0., 0.)),
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");
    let mesh = meshes.add(Circle::new(BALL_SIZE));
    let material = materials.add(Color::srgb(1., 0., 0.));
    commands.spawn((
        BallBundle::new(1., 0.),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            ..default()
        },
    ));
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning paddles...");
    let mesh = meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let material = materials.add(Color::srgb(0., 1., 0.));
    commands.spawn((
        PaddleBundle::new(200., -25.),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            ..default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&wall) {
            return None;
        }

        let closest_point = wall.closest_point(ball.center());
        let offset = ball.center() - closest_point;

        let side = if offset.x.abs() > offset.y.abs() {
            if offset.x < 0. {
                Collision::Left
            } else {
                Collision::Right
            }
        } else {
            if offset.y > 0. {
                Collision::Top
            } else {
                Collision::Bottom
            }
        }; // TODO: cleaner

        Some(side)
    }

    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left | Collision::Right => {
                        ball_velocity.0.x = -ball_velocity.0.x;
                    }
                    Collision::Top | Collision::Bottom => {
                        ball_velocity.0.y = -ball_velocity.0.y;
                    }
                }
            }
        }
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_paddles, spawn_camera))
        .add_systems(
            Update,
            (
                move_ball,
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
