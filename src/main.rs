use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(InputTimer::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        // .add_system(greet_people.system())
        .run();
}

struct Player;
struct InputTimer {
    up_timer: Option<Timer>,
    down_timer: Option<Timer>,
    left_timer: Option<Timer>,
    right_timer: Option<Timer>,
}

impl InputTimer {
    fn default() -> InputTimer {
        InputTimer {
            up_timer: None,
            down_timer: None,
            left_timer: None,
            right_timer: None,
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
    mut timer: ResMut<InputTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        // can only move in one direction per step
        // debounce movement in each direction
        if keyboard_input.pressed(KeyCode::Left) {
            if timer.left_timer.is_none() || timer.left_timer.as_ref().unwrap().just_finished {
                let translation = &mut transform.translation;
                // move the paddle horizontally
                *translation.x_mut() -= 10.0;
                *translation.x_mut() = translation.x().min(300.0).max(-300.0);
            }

            if let Some(t) = &timer.left_timer {
                let mut update = t.clone();
                update.tick(time.delta_seconds);
                timer.left_timer = Some(update);
            } else {
                timer.left_timer = Some(Timer::from_seconds(0.2, true));
            }
        } else {
            timer.left_timer = None;
        }
    }
}
