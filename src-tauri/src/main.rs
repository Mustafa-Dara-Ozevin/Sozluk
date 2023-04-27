// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::fmt;

pub struct Word {
    pub name: String,
    pub definitions: Vec<String>,
    pub def_count: usize,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Word {
    pub fn new() -> Word {
        Word {
            name: String::new(),
            definitions: Vec::new(),
            def_count: 0,
        }
    }
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

}
#[tauri::command]
async fn search(query: &str) -> Result<Vec<String>, ()>   {
    let mut word = Word::new();
    let url = format!(
        "https://sozluk.gov.tr/gts?ara={}",
        query.trim().to_lowercase()
    );
    let body = reqwest::get(url).await.unwrap().json::<serde_json::Value>().await.unwrap();

    let dif_defs = body.to_string().matches("anlamlarListe").count();
    for j in 0..dif_defs {
        let mut definition_count: String = body[0]["anlam_say"].to_string(); // can't parse the string because it is ""number"" instead "number"
        definition_count.retain(|c| c != '"'); // use retain to remove extra double quotes
        word.def_count = definition_count.parse().unwrap();
        for i in 0..word.def_count {
            let meaning = body[j]["anlamlarListe"][i]["anlam"].to_string().replace("\"", "");
            if meaning != "null" {
                word.definitions.push(meaning);
            }
        }
    }
    return Ok(word.definitions)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
