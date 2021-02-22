use bevy::prelude::*;
use rand::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;

const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

struct Position {
    x: i32,
    y: i32,
}

struct Materials {
    colors: Vec<Handle<ColorMaterial>>,
}

fn spawn_block_element(commands: &mut Commands, materials: Res<Materials>) {
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= materials.colors.len();

    commands
        .spawn(SpriteBundle {
            material: materials.colors[color_index].clone(),
            ..Default::default()
        })
        .with(Position { x: 1, y: 5 })
        .current_entity()
        .unwrap();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Materials {
        colors: vec![
            materials.add(Color::rgb_u8(64, 230, 100).into()),
            materials.add(Color::rgb_u8(220, 64, 90).into()),
            materials.add(Color::rgb_u8(70, 150, 210).into()),
            materials.add(Color::rgb_u8(220, 230, 70).into()),
            materials.add(Color::rgb_u8(35, 220, 241).into()),
            materials.add(Color::rgb_u8(240, 140, 70).into()),
        ],
    });
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>) {
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform, mut sprite)| {
            transform.translation = Vec3::new(
                (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
            sprite.size = Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32)
        });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(spawn_block_element.system())
        .add_system(position_transform.system())
        .run();
}
