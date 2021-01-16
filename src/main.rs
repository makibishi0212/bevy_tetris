use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: 500.,
            height: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
