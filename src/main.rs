use bevy::prelude::*;
use std::collections::HashMap;

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

struct Movable;

struct Wall;

#[derive(Hash, Eq, PartialEq, Debug)]
struct GridLocation(i32, i32);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(0, 0))
        .with(Movable)
        .with(Player);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-5, 0))
        .with(Movable)
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(-70.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-7, 0))
        .with(Movable)
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.3, 0.7).into()),
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-10, 0))
        .with(Wall);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(5, 0))
        .with(Movable)
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(70.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(7, 0))
        .with(Movable)
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.3, 0.7).into()),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(10, 0))
        .with(Wall);
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wall_query: Query<(Entity, &Wall, &GridLocation)>,
    mut set: QuerySet<(
        Query<(Entity, &Movable, &mut GridLocation)>,
        Query<(Entity, &Player, &GridLocation)>,
    )>,
) {
    let mut delta = 0;
    if keyboard_input.just_pressed(KeyCode::Left) {
        delta = -1;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        delta = 1;
    }
    if delta == 0 {
        return;
    }

    let immovables: HashMap<GridLocation, Entity> = {
        let mut tmp = HashMap::new();
        for (wall_entity, _wall, wall_grid_location) in wall_query.iter_mut() {
            tmp.insert(
                GridLocation(wall_grid_location.0, wall_grid_location.1),
                wall_entity,
            );
        }
        tmp
    };

    let movables: HashMap<GridLocation, Entity> = {
        let mut tmp = HashMap::new();
        for (movable_entity, _movable, grid_location) in set.q0_mut().iter_mut() {
            tmp.insert(
                GridLocation(grid_location.0, grid_location.1),
                movable_entity,
            );
        }
        tmp
    };

    let mut to_move: Vec<Entity> = vec![];

    for (_player_entity, _player, player_grid_location) in set.q1().iter() {
        let mut tmp_to_move = vec![];

        let mut current_x = player_grid_location.0;
        let current_y = player_grid_location.1;
        while let Some(movable) = movables.get(&GridLocation(current_x, current_y)) {
            tmp_to_move.push(*movable);
            current_x += delta;
        }
        if let Some(_immovable) = immovables.get(&GridLocation(current_x, current_y)) {
            continue;
        }
        to_move.append(&mut tmp_to_move);
    }

    for loc in to_move {
        let mut grid_location: Mut<GridLocation> = set.q0_mut().get_component_mut(loc).unwrap();
        grid_location.0 += delta;
    }
}

fn render_grid_location_to_transform(mut query: Query<(&GridLocation, &mut Transform)>) {
    for (grid_location, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() = 10. * grid_location.0 as f32;
        *transform.translation.y_mut() = 10. * grid_location.1 as f32;
    }
}
