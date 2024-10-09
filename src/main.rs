// ./res/bubble-popping.mp3
extern crate rodio;
extern crate winapi;
extern crate clap;

use rodio::{source::Source, OutputStream};
use std::{fs::File, io::BufReader, thread, time::Duration};
use clap::{Parser};

/// Simple program to play a sound every 15 minutes
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

fn main() {
    // Free the console on a Windows environment
    unsafe {
        winapi::um::wincon::FreeConsole();
    }

    let args = Args::parse();

    // Set up the audio stream
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // Set the volume
    sink.set_volume(args.volume);

    // File to be played
    let file = BufReader::new(File::open(args.path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();

    // Clone the source to allow repeated plays
    let source = source.buffered();

    // Main loop
    loop {
        // Add the sound source to the sink
        sink.append(source.clone());

        // Sleep for 15 minutes
        thread::sleep(Duration::from_secs(args.interval * 60));
    }
}
