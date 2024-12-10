use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

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

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain())
        .run();
}
