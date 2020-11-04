use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .run();
}

fn hello_world() {
    println!("hello world!");
}

struct Person;

struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_person, name) in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
