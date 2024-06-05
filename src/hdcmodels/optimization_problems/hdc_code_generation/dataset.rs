use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use huggingface_hub::snapshot_download;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StackEntry {
    pub id: String,
    pub text: String,
    pub timestamp: String,
    pub score: i32,
}

pub struct CodeDataset {
    entries: Vec<StackEntry>,
}

impl CodeDataset {
    pub fn new() -> Self {
        let data_dir = snapshot_download("bigcode/the-stack", "data", None, None).unwrap();
        let data_path = Path::new(&data_dir).join("stackoverflow_code_snippets.jsonl");

        let file = File::open(data_path).expect("Failed to open dataset file");
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line.expect("Failed to read line from dataset");
            let entry: StackEntry = serde_json::from_str(&line).expect("Failed to parse JSON");
            entries.push(entry);
        }

        CodeDataset { entries }
    }

    pub fn get_entries(&self) -> &[StackEntry] {
        &self.entries
    }

    pub fn get_code_snippets(&self) -> Vec<String> {
        self.entries.iter().map(|entry| entry.text.clone()).collect()
    }

    pub fn get_batches(&self, batch_size: usize) -> Vec<Vec<String>> {
        let code_snippets = self.get_code_snippets();
        code_snippets.chunks(batch_size).map(|chunk| chunk.to_vec()).collect()
    }
}