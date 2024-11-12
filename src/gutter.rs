use crate::{Position, Shape};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub const GUTTER_HEIGHT: f32 = 96.;

#[derive(Component)]
struct Gutter;

#[derive(Bundle)]
pub struct GutterBundle {
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

    pub fn spawn(
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
                    material: material_handle,
                    ..default()
                },
            ));
        }
    }
}
