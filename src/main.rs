use bevy::{app::AppExit, prelude::*};
use std::{
    collections::{HashMap, HashSet},
    ops,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(render_grid_location_to_transform.system())
        .add_system(goal_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct Player;

struct Box;

struct Movable;

struct Wall;

struct Goal;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct GridLocation(i32, i32);

impl ops::Add<GridLocation> for GridLocation {
    type Output = GridLocation;

    fn add(self, rhs: GridLocation) -> Self::Output {
        GridLocation(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const SPRITE_WIDTH: f32 = 50.0;
const SPRITE_HEIGHT: f32 = 50.0;

fn setup_player(
    grid_location: GridLocation,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(
                SPRITE_WIDTH * grid_location.0 as f32,
                SPRITE_HEIGHT * grid_location.1 as f32,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
            ..Default::default()
        })
        .with(grid_location)
        .with(Movable)
        .with(Player);
}

fn setup_box(
    grid_location: GridLocation,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(
                SPRITE_WIDTH * grid_location.0 as f32,
                SPRITE_HEIGHT * grid_location.1 as f32,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
            ..Default::default()
        })
        .with(grid_location)
        .with(Movable)
        .with(Box);
}

fn setup_wall(
    grid_location: GridLocation,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(Vec3::new(
                SPRITE_WIDTH * grid_location.0 as f32,
                SPRITE_HEIGHT * grid_location.1 as f32,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
            ..Default::default()
        })
        .with(grid_location)
        .with(Wall);
}

fn setup_goal(
    grid_location: GridLocation,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.3, 0.7).into()),
            transform: Transform::from_translation(Vec3::new(
                SPRITE_WIDTH * grid_location.0 as f32,
                SPRITE_HEIGHT * grid_location.1 as f32,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(0.5 * SPRITE_WIDTH, 0.5 * SPRITE_HEIGHT)),
            ..Default::default()
        })
        .with(grid_location)
        .with(Goal);
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    // 11 high
    // 19 wide

    setup_wall(GridLocation(-9, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(-9, -0), &mut commands, &mut materials);
    setup_wall(GridLocation(-9, -1), &mut commands, &mut materials);
    setup_wall(GridLocation(-9, -2), &mut commands, &mut materials);

    setup_wall(GridLocation(-8, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(-7, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(-6, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, -2), &mut commands, &mut materials);
    
    setup_wall(GridLocation(-5, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, -4), &mut commands, &mut materials);

    setup_wall(GridLocation(-4, -4), &mut commands, &mut materials);
    setup_wall(GridLocation(-3, -4), &mut commands, &mut materials);
    setup_wall(GridLocation(-2, -4), &mut commands, &mut materials);
    setup_wall(GridLocation(-1, -4), &mut commands, &mut materials);
    setup_wall(GridLocation(0, -4), &mut commands, &mut materials);
    setup_wall(GridLocation(1, -4), &mut commands, &mut materials);

    setup_wall(GridLocation(1, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(2, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(3, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(4, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(5, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(6, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(7, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(8, -3), &mut commands, &mut materials);
    setup_wall(GridLocation(9, -3), &mut commands, &mut materials);
    
    setup_wall(GridLocation(9, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(9, -1), &mut commands, &mut materials);
    setup_wall(GridLocation(9, -0), &mut commands, &mut materials);
    setup_wall(GridLocation(9, 1), &mut commands, &mut materials);

    setup_wall(GridLocation(8, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(7, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(6, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(5, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(4, 1), &mut commands, &mut materials);

    setup_wall(GridLocation(4, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(3, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(2, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(1, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(0, 0), &mut commands, &mut materials);

    setup_wall(GridLocation(0, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(0, 2), &mut commands, &mut materials);
    setup_wall(GridLocation(0, 3), &mut commands, &mut materials);

    setup_wall(GridLocation(-1, 3), &mut commands, &mut materials);
    setup_wall(GridLocation(-1, 4), &mut commands, &mut materials);
    setup_wall(GridLocation(-1, 5), &mut commands, &mut materials);
    setup_wall(GridLocation(-1, 6), &mut commands, &mut materials);

    setup_wall(GridLocation(-2, 6), &mut commands, &mut materials);
    setup_wall(GridLocation(-3, 6), &mut commands, &mut materials);
    setup_wall(GridLocation(-4, 6), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, 6), &mut commands, &mut materials);

    setup_wall(GridLocation(-5, 5), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, 4), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, 3), &mut commands, &mut materials);

    setup_wall(GridLocation(-6, 3), &mut commands, &mut materials);
    setup_wall(GridLocation(-7, 3), &mut commands, &mut materials);

    setup_wall(GridLocation(-7, 2), &mut commands, &mut materials);
    setup_wall(GridLocation(-7, 1), &mut commands, &mut materials);

    setup_wall(GridLocation(-8, 1), &mut commands, &mut materials);

    // stray walls
    setup_wall(GridLocation(-5, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(-5, 1), &mut commands, &mut materials);

    setup_wall(GridLocation(-3, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(-3, 1), &mut commands, &mut materials);
    setup_wall(GridLocation(-2, 0), &mut commands, &mut materials);
    setup_wall(GridLocation(-2, 1), &mut commands, &mut materials);

    setup_wall(GridLocation(-3, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(-2, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(-2, -2), &mut commands, &mut materials);

    setup_wall(GridLocation(1, -2), &mut commands, &mut materials);

    setup_wall(GridLocation(3, -2), &mut commands, &mut materials);
    setup_wall(GridLocation(4, -2), &mut commands, &mut materials);

    setup_box(GridLocation(-7, -1), &mut commands, &mut materials);
    
    setup_box(GridLocation(-4, -1), &mut commands, &mut materials);
    setup_box(GridLocation(-4, 2), &mut commands, &mut materials);
    setup_box(GridLocation(-4, 4), &mut commands, &mut materials);

    setup_box(GridLocation(-2, 2), &mut commands, &mut materials);
    setup_box(GridLocation(-2, 3), &mut commands, &mut materials);

    setup_player(GridLocation(2, -2), &mut commands, &mut materials);

    setup_goal(GridLocation(7, 0), &mut commands, &mut materials);
    setup_goal(GridLocation(7, -1), &mut commands, &mut materials);
    setup_goal(GridLocation(7, -2), &mut commands, &mut materials);

    setup_goal(GridLocation(8, 0), &mut commands, &mut materials);
    setup_goal(GridLocation(8, -1), &mut commands, &mut materials);
    setup_goal(GridLocation(8, -2), &mut commands, &mut materials);
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wall_query: Query<(Entity, &Wall, &GridLocation)>,
    mut set: QuerySet<(
        Query<(Entity, &Movable, &mut GridLocation)>,
        Query<(Entity, &Player, &GridLocation)>,
    )>,
) {
    let delta = {
        let mut delta = GridLocation(0, 0);
        if keyboard_input.just_pressed(KeyCode::Left) {
            delta = GridLocation(-1, 0);
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            delta = GridLocation(1, 0);
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            delta = GridLocation(0, -1);
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            delta = GridLocation(0, 1);
        }
        if delta == GridLocation(0, 0) {
            return;
        }
        delta
    };

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

        let mut current_loc = *player_grid_location;

        while let Some(movable) = movables.get(&current_loc) {
            tmp_to_move.push(*movable);
            current_loc = current_loc + delta;
        }
        if let Some(_immovable) = immovables.get(&current_loc) {
            continue;
        }
        to_move.append(&mut tmp_to_move);
    }

    for loc in to_move {
        let mut grid_location: Mut<GridLocation> = set.q0_mut().get_component_mut(loc).unwrap();
        *grid_location = *grid_location + delta;
    }
}

const LERP_LAMBDA: f32 = 5.0;

fn render_grid_location_to_transform(
    time: Res<Time>,
    mut query: Query<(&GridLocation, &mut Transform)>
) {
    for (grid_location, mut transform) in query.iter_mut() {
        let target_x = SPRITE_WIDTH * grid_location.0 as f32;
        *transform.translation.x_mut() = transform.translation.x() * (1.0 - LERP_LAMBDA * time.delta_seconds) + target_x * LERP_LAMBDA * time.delta_seconds;
        let target_y = SPRITE_WIDTH * grid_location.1 as f32;
        *transform.translation.y_mut() = transform.translation.y() * (1.0 - LERP_LAMBDA * time.delta_seconds) + target_y * LERP_LAMBDA * time.delta_seconds;
    }
}

fn goal_system(
    mut app_exit_events: ResMut<Events<AppExit>>,

    box_query: Query<(&Box, &GridLocation)>,
    goal_query: Query<(&Goal, &GridLocation)>,
) {
    let boxes: HashSet<GridLocation> = {
        let mut tmp = HashSet::new();
        for (_box, grid_location) in box_query.iter() {
            tmp.insert(GridLocation(grid_location.0, grid_location.1));
        }
        tmp
    };

    if goal_query
        .iter()
        .all(|(_goal, grid_location)| boxes.contains(grid_location))
    {
        println!("You win!");
        app_exit_events.send(AppExit);
    }
}
