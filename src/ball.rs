use crate::{Position, Shape, Velocity};
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

const BALL_SPEED: f32 = 5.;
const BALL_SIZE: f32 = 5.;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Horizontal,
    Vertical,
}

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::splat(BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0., 0.)),
        }
    }

    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let shape = Circle::new(BALL_SIZE);
        let color = Color::srgb(1., 0., 0.);

        let mesh = meshes.add(shape);
        let material = materials.add(color);

        commands.spawn((
            Self::new(1., 1.),
            MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                ..default()
            },
        ));
    }
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

pub fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&wall) {
            return None;
        }

        let offset = ball.center() - wall.closest_point(ball.center());

        let side = if offset.x.abs() > offset.y.abs() {
            Collision::Horizontal
        } else {
            Collision::Vertical
        };

        Some(side)
    }

    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Horizontal => {
                        ball_velocity.0.x = -ball_velocity.0.x;
                    }
                    Collision::Vertical => {
                        ball_velocity.0.y = -ball_velocity.0.y;
                    }
                }
            }
        }
    }
}
