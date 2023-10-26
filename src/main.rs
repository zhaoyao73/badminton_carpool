use std::collections::hash_map::HashMap;
use std::fs;
use std::io::Result;

const DATAFILE: &'static str = "data/data.md";
const MAX_COLUMNS: usize = 5;
const DATA_COLUMN: usize = 1;
const DATA_ROW: usize = 2;

#[derive(Default, Clone)]
struct Stat {
    going: u32,
    come_back: u32,
    drives: u32,
}

fn read_data_file(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
}

fn parse_cols(cols: &Vec<&str>, collected_data: &mut HashMap<String, Stat>) -> Result<()> {
    let mut col = DATA_COLUMN;
    let date = cols[col];
    col += 1;
    let going = cols[col].trim_start().trim_end();
    col += 1;
    let come_back = cols[col].trim_start().trim_end();
    col += 1;
    let driver = cols[col].trim_start().trim_end();

    for name in going.split(',') {
        collected_data
            .entry(name.to_string())
            .or_insert(Stat::default())
            .going += 1;
    }

    for name in come_back.split(',') {
        collected_data
            .entry(name.to_string())
            .or_insert(Stat::default())
            .come_back += 1;
    }
    for name in driver.split(',') {
        match name.split('\'').next() {
            Some(child_name) => {
                collected_data
                    .entry(child_name.to_string())
                    .or_insert(Stat::default())
                    .drives += 1
            }
            None => eprintln!("unable to analyze {} driver info.", driver),
        };
    }
    let go_width = 18;
    let back_width = go_width;
    println!(
        "date:{} going:{:go_width$} back:{:back_width$} driver:{}",
        date, going, come_back, driver,
    );
    Ok(())
}

fn parse_rows(rows: &Vec<&str>) -> Option<HashMap<String, Stat>> {
    let mut collected_data = HashMap::<String, Stat>::new();
    // markdown format
    // 0. title
    // 1. |----|
    for row in &rows[DATA_ROW..] {
        let cols: Vec<&str> = row.trim_start().trim_end().split('|').collect();
        if cols.is_empty() || cols.len() < MAX_COLUMNS {
            continue;
        }
        if !parse_cols(&cols, &mut collected_data).is_ok() {
            println!("unable to parse row:{:?}", cols);
        }
    }
    Some(collected_data)
}

fn parse_data_file(file_path: &str) -> Option<HashMap<String, Stat>> {
    match read_data_file(file_path) {
        Ok(content) => {
            let rows: Vec<&str> = content.split('\n').collect();
            if rows.is_empty() || rows.len() < 2 {
                return None;
            }
            parse_rows(&rows)
        }
        Err(error) => {
            eprintln!("Unable to read file {} error: {}", file_path, error);
            None
        }
    }
}

fn analyze(stats: &HashMap<String, Stat>) {
    println!("family  going back drive");
    if stats.is_empty() {
        return;
    }

    for (name, s) in stats.iter() {
        println!(
            "{:<6}  {:<5} {:<4} {}",
            name, s.going, s.come_back, s.drives
        );
    }
}

fn main() {
    if let Some(stats) = parse_data_file(DATAFILE) {
        analyze(&stats);
    }
}
