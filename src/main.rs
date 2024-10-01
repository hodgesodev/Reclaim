use crate::TileType::{Floor, Wall};
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::fs::*;
use std::io::*;
use std::path::Path;
use std::ptr::eq;
use std::vec::*;

const MOVE_SPEED: f32 = 1.0;
fn conf() -> Conf {
    Conf {
        window_title: String::from("Reclaim"),
        window_width: 1280,
        window_height: 960,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[derive(Clone, Copy)]
enum TileType {
    Wall,
    Floor,
    Ceiling,
}

#[derive(Copy, Clone)]
pub struct Tile {
    kind: TileType,
    x: f32,
    y: f32,
    z: f32,
}

fn load_level(level_dat: &str) -> (Vec<Tile>, Vec2) {
    // Create a path to the desired file
    let path = Path::new(level_dat);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    let level_vec: Vec<&str> = s.split("\n").collect();

    let mut row: i32 = 0;
    let mut column: i32 = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut start_pos: Vec2 = vec2(0.0, 0.0);

    for line in level_vec.iter() {
        let split: Vec<&str> = line.split("").collect();
        for ch in split.iter() {
            if eq(ch, &"*") {
                start_pos = vec2(column as f32, row as f32);
                let t_floor = Tile {
                    kind: Floor,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                };
                tiles.push(t_floor);
                let t_ceil = Tile {
                    kind: TileType::Ceiling,
                    x: column as f32,
                    y: 1.0,
                    z: row as f32,
                };
                tiles.push(t_ceil);
            }
            if eq(ch, &"#") {
                let t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.5,
                    z: row as f32,
                };
                tiles.push(t);
            } else if eq(ch, &" ") {
                let t_floor = Tile {
                    kind: Floor,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                };
                tiles.push(t_floor);
                let t_ceil = Tile {
                    kind: TileType::Ceiling,
                    x: column as f32,
                    y: 1.0,
                    z: row as f32,
                };
                tiles.push(t_ceil);
            }
            column += 1;
        }
        column = 0;
        row += 1;
    }

    (tiles, start_pos).clone()
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right ;
    let mut up ;

    show_mouse(false);

    let mut tiles: Vec<Tile>;
    let mut start_position: Vec2;

    (tiles, start_position) = load_level("./assets/level1.dat");

    let mut position = vec3(start_position.x, 0.5, start_position.y);

    loop {
        let _delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Up) {
            position += front * MOVE_SPEED;
        }
        if is_key_pressed(KeyCode::Down) {
            position -= front * MOVE_SPEED;
        }
        if is_key_pressed(KeyCode::Left) {
            yaw -= PI / 2.0;
        }
        if is_key_pressed(KeyCode::Right) {
            yaw += PI / 2.0;
        }

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(BLACK);

        // Going 3d!

        set_camera(&Camera3D {
            position,
            up,
            fovy: 1.5,
            aspect: Option::from(1.0),
            projection: Projection::Perspective,
            render_target: None,
            target: position + front,
            viewport: Option::from((0, screen_height() as i32 - 720, 720, 720)),
        });

        draw_grid_ex(
            20,
            1.0,
            BLACK,
            GRAY,
            vec3(0.5, 0.0, 0.5),
            Default::default(),
        );

        // draw_cube_wires(vec3(1.0, 0.5, -5.0), vec3(1., 1., 1.), GREEN);
        // draw_cube_wires(vec3(1.0, 0.5, 5.0), vec3(1., 1., 1.), BLUE);
        // draw_cube_wires(vec3(6.0, 0.5, 3.0), vec3(1., 1., 1.), RED);

        for tile in &tiles {
            match tile.kind {
                Floor => {
                    draw_affine_parallelogram(
                        vec3(tile.x + 0.5, tile.y, tile.z + 0.5),
                        1. * Vec3::X,
                        1. * Vec3::Z,
                        Option::from(&Texture2D::from_file_with_format(include_bytes!("../assets/Dirt_16.png"), None)),
                        WHITE,
                    );
                }
                Wall => {
                    draw_cube(
                        vec3(tile.x + 0.5, tile.y, tile.z + 0.5),
                        vec3(1.0, 1.0, 1.0),
                        Option::from(&Texture2D::from_file_with_format(include_bytes!("../assets/Brick_08.png"), None)),
                        WHITE,
                    );
                }
                TileType::Ceiling => {
                    draw_affine_parallelogram(
                        vec3(tile.x + 0.5, tile.y, tile.z + 0.5),
                        1. * Vec3::X,
                        1. * Vec3::Z,
                        Option::from(&Texture2D::from_file_with_format(include_bytes!("../assets/Metal_17.png"), None)),
                        WHITE,
                    );
                }
            }
        }

        // Back to screen space, render some text

        set_default_camera();

        next_frame().await
    }
}
