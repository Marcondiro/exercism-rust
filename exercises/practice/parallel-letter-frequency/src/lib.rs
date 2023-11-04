use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    assert!(worker_count > 0, "At least one worker is needed");

    // Use a channel to send each worker result back, instead of the JoinHandle, to be able to merge
    // the results in order of worker completion (faster worker first) instead of a fixed order
    let (sender, receiver) = channel();

    // Compute the input size for each worker. Ceil division is done by hand since
    // [the std implementation](https://github.com/rust-lang/rust/issues/88581) is not ready yet.
    let worker_input_size = (input.len() + worker_count - 1) / worker_count;

    let mut input_iterator = input.iter();

    let join_handles: Vec<JoinHandle<()>> = (0..worker_count)
        .map(|_| {
            let worker_input: Vec<String> = input_iterator
                .by_ref()
                .take(worker_input_size)
                .map(|s| s.to_lowercase())
                .collect();
            let worker_sender = sender.clone();

            thread::spawn(move || work(&worker_input, &worker_sender))
        })
        .collect();

    let mut map = receiver.recv().unwrap();
    for _ in 1..worker_count {
        for entry in receiver.recv().unwrap() {
            map.entry(entry.0)
                .and_modify(|occurrences| *occurrences += entry.1)
                .or_insert(entry.1);
        }
    }

    for handle in join_handles {
        handle.join().unwrap();
    }

    map
}

fn work(input: &[String], sender: &Sender<HashMap<char, usize>>) {
    let mut map = HashMap::new();

    for string in input {
        for chr in string.chars().filter(|c| c.is_alphabetic()) {
            map.entry(chr)
                .and_modify(|occurrences| *occurrences += 1)
                .or_insert(1);
        }
    }

    sender.send(map).unwrap();
}
