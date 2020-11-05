use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(render_grid_location_to_transform.system())
        .run();
}

struct Player;

struct Box;

struct GridLocation(i32, i32);

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
        .with(GridLocation(0, 0))
        .with(Player)


        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-5, 0))
        .with(Box);
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut GridLocation)>,
) {
    for (_player, mut grid_location) in query.iter_mut() {
        // can only move in one direction per step
        // debounce movement in each direction
        if keyboard_input.just_pressed(KeyCode::Left) {
            grid_location.0 -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            grid_location.0 += 1;
        }
    }
}

fn render_grid_location_to_transform(
    mut query: Query<(&GridLocation, &mut Transform)>,
) {
    for (grid_location, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() = 10. * grid_location.0 as f32;
        *transform.translation.y_mut() = 10. * grid_location.1 as f32;
    }
}