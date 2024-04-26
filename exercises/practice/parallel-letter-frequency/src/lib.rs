use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;

/// # Panics
/// The function panics if `worker_count` is 0.
#[must_use]
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    assert!(worker_count > 0, "At least one worker is needed");
    if input.len() == 0 {
        return  Default::default();
    }

    // Use a channel to send each worker result back, instead of the JoinHandle, to be able to merge
    // the results in order of worker completion (faster worker first) instead of a fixed order
    let (sender, receiver) = channel();

    let worker_input_size = input.len().div_ceil(worker_count);

    thread::scope(|scope|{
        for chunk in  input.chunks(worker_input_size) {
            let worker_sender = sender.clone();
            scope.spawn(move || work(chunk, &worker_sender));
        }

        let mut map = receiver.recv().unwrap();
        for _ in 1..input.chunks(worker_input_size).len() {
            for (chr, count) in receiver.recv().unwrap() {
                map.entry(chr)
                    .and_modify(|occurrences| *occurrences += count)
                    .or_insert(count);
            }
        }
        map
    })
}

fn work(input: &[&str], sender: &Sender<HashMap<char, usize>>) {
    let mut map = HashMap::new();

    for string in input {
        for chr in string.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            map.entry(chr)
                .and_modify(|occurrences| *occurrences += 1)
                .or_insert(1);
        }
    }

    sender.send(map).unwrap();
}
