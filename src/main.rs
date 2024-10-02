use crate::TileType::{Floor, Wall};
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::fs::*;
use std::io::*;
use std::path::Path;
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
    facing: i8,
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
        // let split: Vec<&str> = line.split("").collect();
        for ch in line.chars() {
            if ch.eq(&'*') {
                start_pos = vec2(column as f32, row as f32);
                let t_floor = Tile {
                    kind: Floor,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 0,
                };
                tiles.push(t_floor);
                let t_ceil = Tile {
                    kind: TileType::Ceiling,
                    x: column as f32,
                    y: 1.0,
                    z: row as f32,
                    facing: 0,
                };
                tiles.push(t_ceil);
            } else if ch.eq(&'▔') {
                let t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 0,
                };
                tiles.push(t);
            } else if ch.eq(&'▕') {
                let t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 1,
                };
                tiles.push(t);
            } else if ch.eq(&'▁') {
                let t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 2,
                };
                tiles.push(t);
            } else if ch.eq(&'▏') {
                let t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 3,
                };
                tiles.push(t);
            } else if ch.eq(&'#') {
                let mut t = Tile {
                    kind: Wall,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 0,
                };
                tiles.push(t);
                t.facing = 1;
                tiles.push(t);
                t.facing = 2;
                tiles.push(t);
                t.facing = 3;
                tiles.push(t);
            } else if ch.eq(&' ') {
                let t_floor = Tile {
                    kind: Floor,
                    x: column as f32,
                    y: 0.0,
                    z: row as f32,
                    facing: 0,
                };
                tiles.push(t_floor);
                let t_ceil = Tile {
                    kind: TileType::Ceiling,
                    x: column as f32,
                    y: 1.0,
                    z: row as f32,
                    facing: 0,
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

fn color_from_distance(cam: Vec3, point: Vec3) -> Color {
    let dist = cam.distance(point);
    let val = 1. / 2f32.powf(dist / 2.);
    let col = Color::new(val, val, val, 1.);
    col
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
    let mut right;
    let mut up;

    let (tiles, start_position) = load_level("./assets/level1.dat");

    let mut position = vec3(start_position.x, 0.5, start_position.y);

    let tex_floor = Texture2D::from_file_with_format(include_bytes!("../assets/Dirt_16.png"), None);
    let tex_wall = Texture2D::from_file_with_format(include_bytes!("../assets/Brick_08.png"), None);
    let tex_ceil = Texture2D::from_file_with_format(include_bytes!("../assets/Metal_17.png"), None);
    let bg = Texture2D::from_file_with_format(include_bytes!("../assets/UI/base.png"), None);

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
        
        draw_texture(&bg, 0.0, 0.0, WHITE);

        set_camera(&Camera3D {
            position,
            up,
            fovy: 1.5,
            aspect: Option::from(1.0),
            projection: Projection::Perspective,
            render_target: None,
            target: position + front,
            viewport: Option::from((0, screen_height() as i32 - 720, 720, 720)),
            // viewport: None,
        });

        for tile in &tiles {
            match tile.kind {
                Floor => {
                    let pos = vec3(tile.x - 0.5, tile.y, tile.z - 0.5);
                    draw_affine_parallelogram(
                        pos,
                        1. * Vec3::X,
                        1. * Vec3::Z,
                        Option::from(&tex_floor),
                        color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                    );
                }
                Wall => match tile.facing {
                    0 => {
                        let pos = vec3(tile.x + 0.5, tile.y + 1., tile.z - 0.5);
                        draw_affine_parallelogram(
                            pos,
                            -1. * Vec3::Y,
                            -1. * Vec3::X,
                            Option::from(&tex_wall),
                            color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                        )
                    }
                    1 => {
                        let pos = vec3(tile.x + 0.5, tile.y + 1., tile.z + 0.5);
                        draw_affine_parallelogram(
                            pos,
                            -1. * Vec3::Y,
                            -1. * Vec3::Z,
                            Option::from(&tex_wall),
                            color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                        )
                    }
                    2 => {
                        let pos = vec3(tile.x - 0.5, tile.y + 1., tile.z + 0.5);
                        draw_affine_parallelogram(
                            pos,
                            -1. * Vec3::Y,
                            1. * Vec3::X,
                            Option::from(&tex_wall),
                            color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                        )
                    }
                    3 => {
                        let pos = vec3(tile.x - 0.5, tile.y + 1., tile.z - 0.5);
                        draw_affine_parallelogram(
                            pos,
                            -1. * Vec3::Y,
                            1. * Vec3::Z,
                            Option::from(&tex_wall),
                            color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                        )
                    }
                    _ => {}
                },
                TileType::Ceiling => {
                    draw_affine_parallelogram(
                        vec3(tile.x - 0.5, tile.y, tile.z - 0.5),
                        1. * Vec3::X,
                        1. * Vec3::Z,
                        Option::from(&tex_ceil),
                        color_from_distance(position, vec3(tile.x, tile.y, tile.z)),
                    );
                }
            }
        }

        set_camera(&Camera2D {
            rotation: 0.0,
            zoom: Default::default(),
            target: Default::default(),
            offset: Default::default(),
            render_target: None,
            viewport: None,
        });

        set_default_camera();

        // let font = load_ttf_font("./assets/chomsky/Chomsky.ttf").await.unwrap();

        // let text_params = TextParams {
        //     font: Option::from(&font),
        //     font_size: 60,
        //     font_scale: 1.0,
        //     font_scale_aspect: 1.,
        //     rotation: 0.0,
        //     color: WHITE,
        // };

        // draw_text_ex("RECLAIM", 800., text_params.font_size as f32 * 1.1, text_params);

        next_frame().await
    }
}
