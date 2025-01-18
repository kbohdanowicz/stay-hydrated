use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use clap::Parser;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetAsyncKeyState, GetCursorPos};
use rodio::{Decoder, OutputStream, Sink, Source};

fn main() {
    if cfg!(not(debug_assertions)) {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
    }

    let args = Args::parse();

    let last_interaction_time = Arc::new(Mutex::new(Instant::now()));

    {
        let last_interaction_time_clone = Arc::clone(&last_interaction_time);
        thread::spawn(move || {
            let mut last_mouse_pos = (0, 0);
            loop {
                // Check for keyboard input
                for key_code in 1..=254 {
                    if unsafe { GetAsyncKeyState(key_code) } != 0 {
                        *last_interaction_time_clone.lock().unwrap() = Instant::now();
                        // println!("New keyboard input");
                        break;
                    }
                }

                // Check for mouse movement
                let mut cursor_pos = POINT { x: 0, y: 0 };
                unsafe {
                    GetCursorPos(&mut cursor_pos);
                }
                let current_mouse_pos = (cursor_pos.x, cursor_pos.y);
                if current_mouse_pos != last_mouse_pos {
                    last_mouse_pos = current_mouse_pos;
                    *last_interaction_time_clone.lock().unwrap() = Instant::now();
                    // println!("New mouse input");
                }

                // Sleep to reduce CPU usage
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    let sound_duration =
        Duration::from_secs(&args.sound_interval * 60);
        // Duration::from_secs(6);

    let interaction_duration =
        Duration::from_secs(&args.interaction_interval * 60);
        // Duration::from_secs(2);

    let mut last_sound_time = Instant::now();

    // let mut counter = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
        // counter += 1;
        // if counter % 2 == 0 {
        //     let count = counter / 2;
        //     println!("{count}");
        // }

        let mut last_interaction = *last_interaction_time.lock().unwrap();

        if last_interaction.elapsed() >= interaction_duration {
            // println!("Resetting counters");
            last_sound_time = Instant::now();
            last_interaction = Instant::now();
            continue;
        }

        if last_sound_time.elapsed() >= sound_duration {
            // println!("Playing sound");
            match play_sound(&args) {
                Ok(_) => {}
                Err(e) => { eprintln!("Error playing sound: {}", e); }
            }
            last_sound_time = Instant::now();
        }
    }
}

fn play_sound(args: &Args)-> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.set_volume(args.volume);

    let file = BufReader::new(File::open(&args.path)?);
    let source = Decoder::new(file)?;

    // ? Clone the source to allow repeated plays
    let source = source.buffered();

    sink.append(source.clone());
    // ? Block until the sound finishes playing
    sink.sleep_until_end();
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./res/bubble-popping.mp3"))]
    path: String,
    #[arg(short, long, default_value_t = 0.35)]
    volume: f32,
    #[arg(short, long, default_value_t = 30)]
    sound_interval: u64,
    #[arg(short, long, default_value_t = 4)]
    interaction_interval: u64,
}
