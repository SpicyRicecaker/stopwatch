## Current Questions

- **Why does this program not seem to work if I don't `println` something before `stdin`???**

- Exactly how does `Arc<Mutex<>>` work such that I dont need a `&mut` when mutating their internal values?

## Past Struggles

All I want to do is pause the time when user inputs.

But if I want to keep on displaying the realtime of the stopwatch while also taking in user input, it's impossible.

Actually, spawning a new thread and then taking user input from that seems pretty promising.

The problem is it's difficult to communicate with that thread.

The easiest option might be `Arc<Mutex<State>>`