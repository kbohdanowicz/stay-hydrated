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

    let last_interaction_time =
        Arc::new(Mutex::new(Instant::now()));

    {
        let last_interaction_time_clone = Arc::clone(&last_interaction_time);
        thread::spawn(move || {
            let mut last_mouse_pos = (0, 0);
            loop {
                if keyboard_pressed() {
                    *last_interaction_time_clone.lock().unwrap() = Instant::now();
                    // println!("New keyboard input");
                }

                let current_mouse_pos = current_mouse_pos();
                if current_mouse_pos != last_mouse_pos {
                    last_mouse_pos = current_mouse_pos;
                    *last_interaction_time_clone.lock().unwrap() = Instant::now();
                    // println!("New mouse input");
                }

                // ? Sleep to reduce CPU usage
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    let work_duration =
        Duration::from_secs(&args.work_duration * 60);

    let required_inactivity_duration =
        Duration::from_secs(&args.required_inactivity_duration * 60);

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

        if last_interaction.elapsed() >= required_inactivity_duration {
            // println!("Resetting counters");
            last_sound_time = Instant::now();
            last_interaction = Instant::now();
            continue;
        }

        if last_sound_time.elapsed() >= work_duration {
            play_rest_start_sound(&args);
            last_sound_time = Instant::now();
        }
    }
}

fn current_mouse_pos() -> (i32, i32) {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    (cursor_pos.x, cursor_pos.y)
}

fn keyboard_pressed() -> bool {
    for key_code in 1..=254 {
        if unsafe { GetAsyncKeyState(key_code) } != 0 {
            return true
        }
    }
    false
}

fn play_rest_start_sound(args: &Args) {
    match play_sound(
        &args.rest_start_sfx_path,
        args.rest_start_sfx_volume,
    ) {
        Ok(_) => {}
        Err(e) => { eprintln!("Error playing sound: {}", e); }
    }
}

fn play_rest_end_sound(args: &Args) {
    match play_sound(
        &args.rest_end_sfx_path,
        args.rest_end_sfx_volume,
    ) {
        Ok(_) => {}
        Err(e) => { eprintln!("Error playing sound: {}", e); }
    }
}

fn play_sound(
    path: &String,
    volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    // println!("Playing sound");
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.set_volume(volume);

    let file = BufReader::new(File::open(path)?);
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
    #[arg(long, default_value_t = String::from("./res/bubble-popping.mp3"))]
    rest_start_sfx_path: String,
    #[arg(long, default_value_t = String::from("./res/mixkit-magic-notification-ring-2344.wav"))]
    rest_end_sfx_path: String,
    #[arg(long, default_value_t = 0.35)]
    rest_start_sfx_volume: f32,
    #[arg(long, default_value_t = 0.35)]
    rest_end_sfx_volume: f32,
    #[arg(long, default_value_t = 30)]
    work_duration: u64,
    #[arg(long, default_value_t = 4)]
    required_inactivity_duration: u64,
}
