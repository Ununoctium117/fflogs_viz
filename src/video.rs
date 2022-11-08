use std::{collections::HashMap, path::Path};

use cairo::{Context, Format, ImageSurface};

use crate::{
    positions::{Position, PositionHistory, Rect},
    ActorInfo,
};

// frames per second of the fight
const POSITION_SAMPLE_RATE: f64 = 4.0;

// fps out output video
const OUTPUT_FRAMERATE: u32 = 30;

fn draw_actor_on_frame(ctx: &Context, info: &ActorInfo, (rel_x, rel_y): Position, frame_size: f64) {
    let (r, g, b) = match info.subtype.as_str() {
        "WhiteMage" | "Scholar" | "Sage" | "Astrologian" => (0.247, 0.890, 0.133),
        "Gunbreaker" | "DarkKnight" | "Warrior" | "Paladin" => (0.137, 0.137, 0.980),
        "Reaper" | "Samurai" | "Ninja" | "Monk" | "Dragoon" => (0.490, 0.027, 0.027),
        "Bard" | "Dancer" | "Machinist" => (0.890, 0.259, 0.259),
        "RedMage" | "BlackMage" | "Summoner" => (0.89, 0.020, 0.020),
        other => {
            println!("unknown class: {}", other);
            (1.0, 1.0, 1.0)
        },
    };

    ctx.set_source_rgb(r, g, b);
    ctx.arc(
        frame_size * rel_x,
        frame_size * rel_y,
        5.0,
        0.0,
        std::f64::consts::TAU,
    );
    ctx.fill().unwrap();
}

fn concat_images_to_video(dir: impl AsRef<Path>, out_vid: impl AsRef<Path>, target_framerate: u32) {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.current_dir(dir.as_ref());
    cmd.arg("-y");
    cmd.arg("-f").arg("image2");
    cmd.arg("-r").arg(target_framerate.to_string());
    cmd.arg("-i").arg("%04d.png");
    cmd.arg("-c:v").arg("libx264");
    cmd.arg("-pix_fmt").arg("yuv420p");
    cmd.arg(out_vid.as_ref());

    let status = cmd.status().unwrap();
    assert!(status.success());
}

pub fn render_animations(
    history: &HashMap<i64, PositionHistory>,
    actors: &HashMap<i64, ActorInfo>,
    start_time: f64,
    end_time: f64,
    ((min_x, min_y), (max_x, max_y)): Rect,
    frame_size: u32,
    base_output_dir: impl AsRef<Path>,
) {
    std::fs::create_dir_all(base_output_dir.as_ref()).unwrap();

    let render_start_time = std::time::Instant::now();

    let frame_size = frame_size as i32;
    let frame_duration: f64 = 1000.0 / POSITION_SAMPLE_RATE;

    let arena_width = max_x - min_x;
    let arena_height = max_y - min_y;

    let mut timestamp = start_time;
    let mut frame_idx = 0;

    while timestamp < end_time {
        let frame_filename = {
            let mut p = base_output_dir.as_ref().to_path_buf();
            p.push(format!("{:04}.png", frame_idx));
            p
        };

        let image_surface = ImageSurface::create(Format::Rgb24, frame_size, frame_size).unwrap();
        let ctx = Context::new(&image_surface).unwrap();

        ctx.scale(1.0, 1.0);

        for (id, history) in history {
            let info = actors.get(id).unwrap();
            match info.type_.as_str() {
                "Player" => {
                    let position = history.get_position_at(timestamp);

                    let rel_x = (position.0 - min_x) / arena_width;
                    let rel_y = (position.1 - min_y) / arena_height;
                    draw_actor_on_frame(&ctx, info, (rel_x, rel_y), frame_size as f64);
                }

                _ => {}
            }
        }

        let mut f = std::io::BufWriter::new(std::fs::File::create(frame_filename).unwrap());
        image_surface.write_to_png(&mut f).unwrap();

        timestamp += frame_duration;
        frame_idx += 1;

        if frame_idx % 10 == 0 {
            println!("Rendered frame {}", frame_idx);
        }
    }

    println!("Rendered all frames in {:?}", std::time::Instant::now() - render_start_time);
    concat_images_to_video(&base_output_dir, "__result.mp4", OUTPUT_FRAMERATE);
}
