use std::{fs, sync::mpsc, thread};

use anyhow::{Ok, Result};
use pedantix_solver::{get_answer_words_count, Worker};
use rand::{seq::SliceRandom, thread_rng};

const NUM_OF_WORKERS: usize = 10;

fn main() -> Result<()> {
    let ans_word_count = get_answer_words_count()?;

    let dictionary =
        fs::read_to_string("./dictionary/clean_french.txt").expect("No source of words found");
    let words = dictionary
        .lines()
        .map(|w| w.trim().to_string())
        .collect::<Vec<_>>();
    assert_eq!(words.len(), 691_954, "dictionary was corrupted");

    let (mut remaining_words, mut last_word_idx) = (words.len(), 0);
    let mut remaining_workers = NUM_OF_WORKERS;

    let (word_resp_chan, word_receiver_chan) = mpsc::channel::<String>();
    while remaining_workers > 0 {
        let worker_words = remaining_words / remaining_workers;
        let mut words_to_check = words[last_word_idx..(last_word_idx + worker_words)].to_vec();
        words_to_check.shuffle(&mut thread_rng()); // to maximize time spend

        last_word_idx += worker_words;
        remaining_words -= worker_words;
        remaining_workers -= 1;

        let worker = Worker::spawn(words_to_check);
        let worker_resp_chan = word_resp_chan.clone();
        thread::spawn(move || worker.search(worker_resp_chan, ans_word_count).unwrap());
    }

    let mut answer_words = vec![];
    for received_word in word_receiver_chan {
        println!("{received_word}");
        answer_words.push(received_word);
        if answer_words.len() >= ans_word_count {
            break;
        }
    }

    println!("{answer_words:?}");
    Ok(())
}
