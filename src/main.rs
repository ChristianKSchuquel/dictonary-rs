#[macro_use] extern crate prettytable;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Phonetic {
    text: Option<String>,
    audio: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Definition {
    definition: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 || args.len() > 1 {
        panic!("Usage: dictionaryrs <word>");
    }

    let api_url = "https://api.dictionaryapi.dev/api/v2/entries/en/";
    let search_url = format!("{}{}", api_url, args[0]);

    let client = reqwest::Client::new();
    let response = client.get(search_url).send().await.unwrap();

    let body = match response.status() {
        reqwest::StatusCode::OK => match response.json::<Vec<Entry>>().await {
            Ok(body) => body,
            Err(e) => { 
                eprintln!("An error occured while parsing the response body: {}", e);
                return
            }
        }
        reqwest::StatusCode::NOT_FOUND => {
            eprintln!("Word not found");
            return
        }
        other => panic!("Something bad happened: {}", other),
    };

    let mut table = prettytable::Table::new();
    table.add_row(row![Fgb => "Word", "Phonetic", "Class", "Definitions"]);

    if body[0].phonetics.is_empty() {
        table.add_row(row![&body[0].word, "Unavailable", &body[0].meanings[0].partOfSpeech, &body[0].meanings[0].definitions[0].definition]);
    }

    let mut phonetic = String::new();

    for x in &body[0].phonetics {
        if !x.text.is_none() {
            phonetic = x.text.clone().expect("couldn't get Phonetic");
        }
    }

    if phonetic.is_empty() {
        phonetic = "Unavailable".to_string();
    }

    table.add_row(row![&body[0].word, phonetic, &body[0].meanings[0].partOfSpeech, &body[0].meanings[0].definitions[0].definition]);

    if body.len() > 1 {
        let entry = &body[1];

        for x in &entry.meanings {
            for y in &x.definitions {
                table.add_row(row!["", "", x.partOfSpeech, y.definition]);
            }
        }
    }


    table.printstd();
}
