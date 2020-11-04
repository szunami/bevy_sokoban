use bevy::prelude::*;

fn main() {
    App::build()
        // .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        // .add_system(greet_people.system())
        .run();
}

struct Player;

fn setup(mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())

        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Player);
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut x_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        *translation.x_mut() += time.delta_seconds * x_direction * 100.0;
        // bound the paddle within the walls
        *translation.x_mut() = translation.x().min(380.0).max(-380.0);

        let mut y_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Down) {
            y_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            y_direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        *translation.y_mut() += time.delta_seconds * y_direction * 100.0;
        // bound the paddle within the walls
        *translation.y_mut() = translation.y().min(380.0).max(-380.0);
    }
}
