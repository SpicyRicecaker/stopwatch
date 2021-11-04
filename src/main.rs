// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
use std::{future::Future, io::Write, thread, time::{Duration, Instant}};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(PartialEq, PartialOrd)]
enum State {
    Pause,
    Resume,
}

#[tokio::main]
async fn main() {
    let mut total = Duration::new(0, 0);
    let state = State::Resume;

    alarm(Instant::now(), state, &mut total).await;
}

async fn alarm(start: Instant, mut state: State, total: &mut Duration) -> Box<dyn Future<Output = ()>> {
    let (tx, mut rx): (Sender<State>, Receiver<State>) = mpsc::channel(1);

    let j = thread::spawn(move || async {
        let thread_tx = tx;

        let mut t = String::new();
        println!("spawned thread");
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

        thread_tx.send(t_state).await
    });

    // Sleep for 1s
    thread::sleep(Duration::from_millis(1000));

    // println!("123");
    // Add elapsed to total
    let state = rx.try_recv().unwrap();

    if state == State::Resume {
        *total += start.elapsed();
    }

    // Print out, overwrite terminal
    let total_seconds = total.as_secs();
    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = (total_seconds / 3600) % 24;

    print!("\r{:02}:{:02}:{:02}", hours, minutes, seconds);

    // Then recursively call self with neew instant
    return alarm(Instant::now(), state, total).await;
    // j.join().unwrap();
}
