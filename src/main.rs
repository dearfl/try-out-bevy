use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct GreetPlugin;

impl GreetPlugin {
    fn add_people(mut commands: Commands) {
        commands.spawn((Person, Name("Alice".to_string())));
        commands.spawn((Person, Name("Bob".to_string())));
        commands.spawn((Person, Name("Charles".to_string())));
    }

    fn greet_people(people: Query<&Name, With<Person>>) {
        for name in people.iter() {
            println!("Hello {}!", name.0);
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
        app.add_systems(Startup, Self::add_people)
            .add_systems(Update, (Self::update_people, Self::greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GreetPlugin)
        .run();
}
