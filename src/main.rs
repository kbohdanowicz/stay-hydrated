extern crate rodio;
extern crate winapi;
extern crate clap;

use rodio::{source::Source, OutputStream};
use std::{fs::File, io::BufReader, thread, time::Duration};
use clap::{Parser};

fn main() {
    if cfg!(not(debug_assertions)) {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
    }

    let args = Args::parse();

    loop {
        match play_sound(&args) {
            Ok(_) => {
                let duration = args.interval * 60;
                thread::sleep(Duration::from_secs(duration))},
            Err(e) => {
                eprintln!("Error playing sound: {}", e);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn play_sound(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;

    sink.set_volume(args.volume);

    let file = BufReader::new(File::open(&args.path)?);
    let source = rodio::Decoder::new(file)?;

    // ? Clone the source to allow repeated plays
    let source = source.buffered();

    sink.append(source.clone());

    // ? Allow sound to play
    thread::sleep(Duration::from_secs(2));

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./res/bubble-popping.mp3"))]
    path: String,
    #[arg(short, long, default_value_t = 0.35)]
    volume: f32,
    #[arg(short, long, default_value_t = 15)]
    interval: u64,
}
