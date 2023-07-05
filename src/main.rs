mod config;
mod pixelbuffer;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, ValueHint};
use game_loop::game_loop;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use chip8_emulator_lib::emulator;

use pixelbuffer::{PixelBuffer, PixelBufferSize};

#[derive(Parser, Debug)]
#[clap(name = "chip8-emulator")]
struct Args {
    #[arg(value_hint = ValueHint::FilePath)]
    rom_path: PathBuf,
    #[arg(short, long, default_value_t = 400)]
    clock_speed: u16,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load()?;

    let size = PixelBufferSize {
        width: emulator::WIDTH as u32,
        height: emulator::HEIGHT as u32,
        pixel_size: config.pixel_size,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Chip8-Emulator")
        .with_inner_size(size.logical_size())
        .with_resizable(false)
        .build(&event_loop)
        .context("Could not crate window.")?;

    let mut input = WinitInputHelper::new();
    let mut pb = PixelBuffer::new(&window, size, config.on_color)
        .context("Could not create frame buffer.")?;

    let program = fs::read(args.rom_path).context("Could not read ROM file.")?;
    let emulator =
        emulator::Emulator::new(args.clock_speed, program).context("Could not create emulator.")?;

    game_loop(
        event_loop,
        window,
        emulator,
        emulator::FPS,
        0.1,
        move |g| {
            g.game.run_frame().unwrap_or_else(|e| {
                eprintln!("Error while running emulator: {}.", e);
                std::process::exit(1);
            });
        },
        move |g| {
            if g.game.should_redraw() {
                let fb = g.game.get_framebuffer();
                pb.set_pixels(|x, y| fb[y][x]).unwrap_or_else(|e| {
                    eprintln!("Error while drawing to frame buffer: {}.", e);
                    std::process::exit(1);
                });
            }
        },
        move |g, event| {
            if input.update(event) {
                if input.close_requested() {
                    g.exit();
                }

                let mut keys_pressed: HashSet<emulator::Key> = HashSet::new();
                for (&c, &k) in &config.keys {
                    if input.key_held(c) || input.key_pressed(c) {
                        keys_pressed.insert(k);
                    }
                }
                g.game.set_keys_pressed(keys_pressed);
            }
        },
    );
}
