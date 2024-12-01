use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

const FILE_PATH: &str = "data.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub current: f64,
    pub income: f64,
    pub expenses: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub avatar: String,
    pub name: String,
    pub category: String,
    pub date: String,
    pub amount: f64,
    pub recurring: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    pub category: String,
    pub maximum: f64,
    pub theme: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pot {
    pub name: String,
    pub target: f64,
    pub total: f64,
    pub theme: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    pub balance: Balance,
    pub transactions: Vec<Transaction>,
    pub budgets: Vec<Budget>,
    pub pots: Vec<Pot>,
}

pub fn read_data() -> AppData {
    let content = fs::read_to_string(FILE_PATH).expect("Failed to read data.json");
    serde_json::from_str(&content).expect("Failed to parse JSON")
}

pub fn write_data(data: &AppData) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize data");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open data.json");
    file.write_all(json.as_bytes())
        .expect("Failed to write data");
}
