mod game;

use bevy::prelude::*;

use game::FlappyBirdPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Flappy Bird".to_string(),
                        resolution: (288., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FlappyBirdPlugin)
        .run();
}
