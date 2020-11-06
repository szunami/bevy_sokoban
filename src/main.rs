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
        .with(Player);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-5, 0))
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(-70.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(-7, 0))
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
        .with(Box);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(70.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(GridLocation(7, 0))
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
    mut box_query: Query<(Entity, &Box, &mut GridLocation)>,
    wall_query: Query<(Entity, &Wall, &GridLocation)>,
    mut people_query: Query<(Entity, &Player, &mut GridLocation)>,
) {
    let immovables: HashMap<GridLocation, Entity> = {
        let mut tmp = HashMap::new();

        for (box_entity, _b, grid_location) in wall_query.iter() {
            tmp.insert(GridLocation(grid_location.0, grid_location.1), box_entity);
        }
        tmp
    };

    let movables: HashMap<GridLocation, Entity> = {
        let mut tmp = HashMap::new();

        for (box_entity, _b, grid_location) in box_query.iter_mut() {
            tmp.insert(GridLocation(grid_location.0, grid_location.1), box_entity);
        }
        tmp
    };

    for (_e, _player, mut grid_location) in people_query.iter_mut() {
        let mut to_move: Vec<Entity> = vec![];
        let mut new_x = grid_location.0;
        let new_y = grid_location.1;
        let mut delta = 0;
        // can only move in one direction per step
        // debounce movement in each direction
        if keyboard_input.just_pressed(KeyCode::Left) {
            delta = -1;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            delta = 1;
        }
        if delta == 0 {
            continue;
        }
        new_x += delta;

        while let Some(movable) = movables.get(&GridLocation(new_x, new_y)) {
            to_move.push(*movable);
            new_x += delta;
        }
        if let Some(_immovable) = immovables.get(&GridLocation(new_x, new_y)) {
            continue;
        }
        grid_location.0 += delta;
        for loc in to_move {
            let mut grid_location: Mut<GridLocation> = box_query.get_component_mut(loc).unwrap();
            grid_location.0 += delta;
        }
    }
}

fn render_grid_location_to_transform(mut query: Query<(&GridLocation, &mut Transform)>) {
    for (grid_location, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() = 10. * grid_location.0 as f32;
        *transform.translation.y_mut() = 10. * grid_location.1 as f32;
    }
}
