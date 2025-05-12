use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_systems(Startup, setup);
    app.add_systems(Update, handle_key);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Paddle::make_entity());
    commands.spawn(Camera2d);
}

fn handle_key(
    key: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
) {
    let shift: f32 = if key.pressed(KeyCode::KeyA) {
        -Paddle::speed()
    } else if key.pressed(KeyCode::KeyD) {
        Paddle::speed()
    } else {
        return;
    };

    paddle_transform.translation.x += shift;
}

#[derive(Component, Debug)]
struct Paddle;

impl Paddle {
    pub fn speed() -> f32 {
        1.
    }
    pub fn initial_pos() -> Vec2 {
        Vec2::new(0., 0.)
    }
    pub fn dim() -> Vec2 {
        Vec2::new(50., 6.)
    }
    pub fn sprite() -> Sprite {
        Sprite::from_color(
            Color::LinearRgba(LinearRgba::new(160., 10., 10., 255.)),
            Self::dim(),
        )
    }
    pub fn transform() -> Transform {
        let pos = Self::initial_pos();
        Transform::from_xyz(pos.x, pos.y, 1.)
    }
    pub fn make_entity() -> (Sprite, Transform, Self) {
        (Self::sprite(), Self::transform(), Self)
    }
}
