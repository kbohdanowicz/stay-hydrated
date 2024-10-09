extern crate rodio;
extern crate winapi;
extern crate clap;

use rodio::{source::Source, OutputStream};
use std::{fs::File, io::BufReader, thread, time::Duration};
use clap::{Parser};

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
    unsafe {
        winapi::um::wincon::FreeConsole();
    }

    let args = Args::parse();

    // Set up the audio stream
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    sink.set_volume(args.volume);

    let file = BufReader::new(File::open(args.path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();

    // Clone the source to allow repeated plays
    let source = source.buffered();

    loop {
        sink.append(source.clone());
        thread::sleep(Duration::from_secs(args.interval * 60));
    }
}
