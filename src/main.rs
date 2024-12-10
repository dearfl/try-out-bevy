use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct GreetPlugin;

impl GreetPlugin {
    fn add_people(mut commands: Commands) {
        commands.spawn((Person, Name("Alice".to_string())));
        commands.spawn((Person, Name("Bob".to_string())));
        commands.spawn((Person, Name("Charles".to_string())));
    }

    fn greet_people(
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>,
        people: Query<&Name, With<Person>>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            for name in people.iter() {
                println!("Hello {}!", name.0);
            }
        }
    }

    fn update_people(mut people: Query<&mut Name, With<Person>>) {
        for mut name in people.iter_mut() {
            name.0.make_ascii_uppercase();
        }
    }
}

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, Self::add_people)
            .add_systems(Update, (Self::update_people, Self::greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GreetPlugin)
        .run();
}
