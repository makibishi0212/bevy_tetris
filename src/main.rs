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

struct Fix;

struct Free;

struct Materials {
    colors: Vec<Handle<ColorMaterial>>,
}

struct BlockPatterns(Vec<Vec<(i32, i32)>>);

struct GameTimer(Timer);
struct InputTimer(Timer);

struct GameBoard(Vec<Vec<bool>>);

struct NewBlockEvent;

fn next_block(block_patterns: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut pattern_index: usize = rng.gen();
    pattern_index %= block_patterns.len();

    block_patterns[pattern_index].clone()
}

fn next_color(colors: &Vec<Handle<ColorMaterial>>) -> Handle<ColorMaterial> {
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= colors.len();

    colors[color_index].clone()
}

fn spawn_block(
    commands: &mut Commands,
    materials: Res<Materials>,
    block_patterns: Res<BlockPatterns>,
    new_block_events: Res<Events<NewBlockEvent>>,
    mut new_block_events_reader: Local<EventReader<NewBlockEvent>>,
) {
    if new_block_events_reader
        .iter(&new_block_events)
        .next()
        .is_none()
    {
        return;
    }

    let new_block = next_block(&block_patterns.0);
    let new_color = next_color(&materials.colors);

    // ブロックの初期位置
    let initial_x = X_LENGTH / 2;
    let initial_y = Y_LENGTH;

    new_block.iter().for_each(|(r_x, r_y)| {
        spawn_block_element(
            commands,
            new_color.clone(),
            Position {
                x: (initial_x as i32 + r_x),
                y: (initial_y as i32 + r_y),
            },
        );
    });
}

fn spawn_block_element(commands: &mut Commands, color: Handle<ColorMaterial>, position: Position) {
    commands
        .spawn(SpriteBundle {
            material: color,
            ..Default::default()
        })
        .with(position)
        .with(Free)
        .current_entity()
        .unwrap();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut new_block_events: ResMut<Events<NewBlockEvent>>,
) {
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

    new_block_events.send(NewBlockEvent);
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

fn game_timer(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut input_timer: ResMut<InputTimer>,
) {
    game_timer.0.tick(time.delta_seconds());
    input_timer.0.tick(time.delta_seconds());
}

fn block_fall(
    commands: &mut Commands,
    timer: ResMut<GameTimer>,
    mut block_query: Query<(Entity, &mut Position, &Free)>,
    mut game_board: ResMut<GameBoard>,
    mut new_block_events: ResMut<Events<NewBlockEvent>>,
) {
    if !timer.0.finished() {
        return;
    }

    let cannot_fall = block_query.iter_mut().any(|(_, pos, _)| {
        if pos.x as u32 >= X_LENGTH || pos.y as u32 >= Y_LENGTH {
            return false;
        }

        // yが0、または一つ下にブロックがすでに存在する
        pos.y == 0 || game_board.0[(pos.y - 1) as usize][pos.x as usize]
    });

    if cannot_fall {
        block_query.iter_mut().for_each(|(entity, pos, _)| {
            commands.remove_one::<Free>(entity);
            commands.insert_one(entity, Fix);
            game_board.0[pos.y as usize][pos.x as usize] = true;
        });
        new_block_events.send(NewBlockEvent);
    } else {
        block_query.iter_mut().for_each(|(_, mut pos, _)| {
            pos.y -= 1;
        });
    }
}

fn block_horizontal_move(
    key_input: Res<Input<KeyCode>>,
    timer: ResMut<InputTimer>,
    game_board: ResMut<GameBoard>,
    mut free_block_query: Query<(Entity, &mut Position, &Free)>,
) {
    if !timer.0.finished() {
        return;
    }

    if key_input.pressed(KeyCode::Left) {
        let ok_move_left = free_block_query.iter_mut().all(|(_, pos, _)| {
            if pos.y as u32 >= Y_LENGTH {
                return pos.x > 0;
            }

            if pos.x == 0 {
                return false;
            }

            !game_board.0[(pos.y) as usize][pos.x as usize - 1]
        });

        if ok_move_left {
            free_block_query.iter_mut().for_each(|(_, mut pos, _)| {
                pos.x -= 1;
            });
        }
    }

    if key_input.pressed(KeyCode::Right) {
        let ok_move_right = free_block_query.iter_mut().all(|(_, pos, _)| {
            if pos.y as u32 >= Y_LENGTH {
                return pos.x as u32 <= X_LENGTH;
            }

            if pos.x as u32 == X_LENGTH - 1 {
                return false;
            }

            !game_board.0[(pos.y) as usize][pos.x as usize + 1]
        });

        if ok_move_right {
            free_block_query.iter_mut().for_each(|(_, mut pos, _)| {
                pos.x += 1;
            });
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .add_resource(BlockPatterns(vec![
            vec![(0, 0), (0, -1), (0, 1), (0, 2)],  // I
            vec![(0, 0), (0, -1), (0, 1), (-1, 1)], // L
            vec![(0, 0), (0, -1), (0, 1), (1, 1)],  // 逆L
            vec![(0, 0), (0, -1), (1, 0), (1, 1)],  // Z
            vec![(0, 0), (1, 0), (0, 1), (1, -1)],  // 逆Z
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],   // 四角
            vec![(0, 0), (-1, 0), (1, 0), (0, 1)],  // T
        ]))
        .add_resource(GameTimer(Timer::new(
            std::time::Duration::from_millis(400),
            true,
        )))
        .add_resource(InputTimer(Timer::new(
            std::time::Duration::from_millis(100),
            true,
        )))
        .add_resource(GameBoard(vec![vec![false; 25]; 25]))
        .add_plugins(DefaultPlugins)
        .add_event::<NewBlockEvent>()
        .add_startup_system(setup.system())
        .add_system(spawn_block.system())
        .add_system(position_transform.system())
        .add_system(game_timer.system())
        .add_system(block_fall.system())
        .add_system(block_horizontal_move.system())
        .run();
}
