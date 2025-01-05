mod game;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use game::FlappyBirdPlugin;

fn exit(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

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
        .add_systems(Update, exit.run_if(input_just_pressed(KeyCode::Escape)))
        .run();
}
