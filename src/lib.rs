use anyhow::{Ok, Result};
use itertools::Itertools;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{fs, thread};

const DELAY_BETWEEN_REQ: u64 = 5_000; // in ms

/// may panic
pub fn clean_original_dictionary() -> Result<()> {
    let dictionary = fs::read_to_string("./dictionary/french_original.txt")?;
    let only_words_dictionary = dictionary
        .lines()
        .map(|word| word.split('\t').next().expect("this is not a word"))
        .unique()
        .collect::<Vec<_>>();
    let only_words_dictionary_str = only_words_dictionary.join("\n");

    let mut file = File::create("./dictionary/clean_french.txt")?;
    file.write_all(only_words_dictionary_str.as_bytes())?;
    Ok(())
}

pub fn get_answer_words_count() -> Result<usize> {
    let html = reqwest::blocking::get("https://cemantix.certitudes.org/pedantix")?.text()?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("#wiki > h2 > span").unwrap();
    Ok(document.select(&selector).count())
}

#[derive(Serialize)]
pub struct PedantixApiReq {
    pub answer: Vec<String>,
    pub word: String,
}

#[derive(Deserialize)]
pub struct PedantixApiResp {
    pub score: HashMap<usize, String>,
}

pub struct Worker {
    words_to_check: Vec<String>,
}

impl Worker {
    pub fn spawn(words_to_check: Vec<String>) -> Self {
        Self { words_to_check }
    }

    pub fn search(&self, return_chan: Sender<String>, ans_word_count: usize) -> Result<()> {
        let client = reqwest::blocking::Client::new();

        'outer: for word in &self.words_to_check {
            let answer = vec![&word]
                .repeat(ans_word_count)
                .iter()
                .map(|s| s.to_string())
                .collect_vec();

            let req_body = PedantixApiReq {
                answer,
                word: word.to_owned(),
            };
            let req_body = serde_json::to_string(&req_body)?;

            let resp_body = client
                .post("https://cemantix.certitudes.org/pedantix/score")
                .body(req_body)
                .send()?
                .text()?;
            let pedantix_resp: PedantixApiResp = serde_json::from_str(&resp_body)?;

            for i in 0..ans_word_count {
                if pedantix_resp.score.get(&i).is_some()
                    && return_chan.send(word.to_owned()).is_err()
                {
                    break 'outer;
                }
            }
            thread::sleep(Duration::from_millis(DELAY_BETWEEN_REQ));
        }
        Ok(())
    }
}
