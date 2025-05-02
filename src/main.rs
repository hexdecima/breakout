use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    let lil_block = Block {
        pos: Vector2::new(10., 10.),
        dim: Vector2::new(40., 16.),
        colour: LinearRgba::new(255., 0., 0., 255.),
    };

    app.add_plugins(DefaultPlugins).add_systems(
        Startup,
        move |mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<ColorMaterial>>| {
            commands.spawn(Camera2d);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(lil_block.dim.x, lil_block.dim.y))),
                MeshMaterial2d(materials.add(Color::LinearRgba(lil_block.colour))),
                Transform::from_xyz(lil_block.pos.x, lil_block.pos.y, 1.),
            ));
        },
    );

    app.run();
}

#[derive(Component)]
struct Block {
    pub pos: Vector2,
    pub dim: Vector2,
    pub colour: LinearRgba,
}

struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
