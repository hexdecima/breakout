use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_systems(Update, || {
        println!("nyan!");
    });

    app.run();
}
