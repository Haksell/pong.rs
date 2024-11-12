use crate::{Ai, Player, Position, Shape, Velocity, GUTTER_HEIGHT};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PADDLE_SPEED: f32 = 4.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
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

    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Query<&Window>,
    ) {
        if let Ok(window) = window.get_single() {
            let window_width = window.resolution.width();
            let padding = 50.;
            let right_paddle_x = window_width / 2. - padding;
            let left_paddle_x = -window_width / 2. + padding;

            let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

            let mesh = meshes.add(shape);

            commands.spawn((
                Player,
                Self::new(right_paddle_x, 0.),
                MaterialMesh2dBundle {
                    mesh: mesh.clone().into(),
                    material: materials.add(Color::srgb(0., 1., 0.)),
                    ..default()
                },
            ));

            commands.spawn((
                Ai,
                Self::new(left_paddle_x, 0.),
                MaterialMesh2dBundle {
                    mesh: mesh.into(),
                    material: materials.add(Color::srgb(0., 0., 1.)),
                    ..default()
                },
            ));
        }
    }
}

pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2. {
                position.0 = new_position;
            }
        }
    }
}
