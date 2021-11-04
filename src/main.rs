use std::sync::{Arc, Mutex};
use std::{thread, time::{Duration, Instant}};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum State {
    Pause,
    Resume,
}

fn main() {
    let mut total = Duration::new(0, 0);
    let state = Arc::new(Mutex::new(State::Resume));

    alarm(Instant::now(), state, &mut total);
}

fn alarm(start: Instant, state: Arc<Mutex<State>>, total: &mut Duration) {
    // Not sure why we clone this sht, checkout https://aeshirey.github.io/code/2020/12/23/arc-mutex-in-rust.html
    // Still have no idea what `Arc` and `Mutex` do on their own
    let thread_arc = state.clone();
    let j = thread::spawn(move || {
        let mut t = String::new();
        println!(" : (from thread 1)");
        let stdin = std::io::stdin();
        stdin.lock();
        stdin.read_line(&mut t).unwrap();

        let t_state = match t.trim() {
            "exit" => std::process::exit(0),
            "pause" => State::Pause,
            "resume" => State::Resume,
            // If it's anything but exit, print out amount of elapsed time
            _ => State::Resume,
        };

        *thread_arc.lock().unwrap() = t_state;
    });

    // Sleep for 1s
    thread::sleep(Duration::from_millis(1000));

    // println!("123");
    // Add elapsed to total
    if *state.lock().unwrap() == State::Resume {
        *total += start.elapsed();
    }

    // Print out, overwrite terminal
    let total_seconds = total.as_secs();
    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = (total_seconds / 3600) % 24;

    print!("\r{:02}:{:02}:{:02}", hours, minutes, seconds);

    // Then recursively call self with neew instant
    alarm(Instant::now(), state, total);
    j.join().unwrap();
}
