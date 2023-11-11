use std::collections::hash_map::HashMap;
use std::fs;
use std::io::Result;

const DATAFILE: &'static str = "data/data.md";
const MAX_COLUMNS: usize = 5;
const DATA_COLUMN: usize = 1;
const DATA_ROW: usize = 2;

#[derive(Default, Clone, Copy)]
struct Stat {
    going: u32,
    back: u32,
    drive: u32,
    score: i32,
}

fn read_data_file(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
}

struct Record<'a> {
    date: &'a str,
    going: &'a str,
    back: &'a str,
    driver: &'a str,
}

struct RecordWidth(usize, usize, usize, usize);
fn get_record_print_width() -> RecordWidth {
    let date_width = "2023.10.10".chars().count();
    let go_width = 18;
    let back_width = go_width;
    let driver_width = go_width;

    RecordWidth(date_width, go_width, back_width, driver_width)
}

// total_families_least_common_mul=least_common_mul(1...N)
//  if N=3, 1,2,3's least common mul is 6
// driver credit: total_families_least_common_mul*(families-1)/families
// passenger debit: total_families_least_common_mul/families
// total_families: families in the whole car pool
// families: one record families
fn update_score(family_stat: &mut Stat, family_name: &str, driver_name: &str, total_families: u32, families: u32) {
    if family_name.eq(driver_name) {
        family_stat.score += (total_families * (families - 1)/ families) as i32;
    } else {
        family_stat.score -= (total_families / families) as i32;
    }
}

fn parse_cols(cols: &Vec<&str>, collected_data: &mut HashMap<String, Stat>) -> Result<()> {
    let col = DATA_COLUMN;

    let record = Record {
        date: cols[col],
        going: cols[col + 1].trim_start().trim_end(),
        back: cols[col + 2].trim_start().trim_end(),
        driver: cols[col + 3].trim_start().trim_end(),
    };

    let mut driver_family_name: String = String::new();
    // let mut driver_family = Stat {
    //     going: 0,
    //     back: 0,
    //     drive: 0,
    //     score: 0,
    // };
    for name in record.driver.split(',') {
        match name.split('\'').next() {
            Some(child_name) => {
                let family = collected_data
                    .entry(child_name.to_string())
                    .or_insert(Stat::default());
                family.drive += 1;
                if driver_family_name.is_empty() {
                    driver_family_name = child_name.to_string();
                    // driver_family = *family;
                } else {
                    panic!("Only support 1 family driver!");
                }
            }
            None => eprintln!("unable to analyze {} driver info.", record.driver),
        };
    }

    let going_count = record.going.split(',').count() as u32;
    for name in record.going.split(',') {
        let family = collected_data
            .entry(name.to_string())
            .or_insert(Stat::default());
        family.going += 1;
        update_score(family, name, &driver_family_name, 6, going_count);
    }

    let back_count = record.back.split(',').count() as u32;
    for name in record.back.split(',') {
        let family = collected_data
            .entry(name.to_string())
            .or_insert(Stat::default());
        family.back += 1;
        update_score(family, name, &driver_family_name, 6, back_count);
    }

    // update score back
    // let family = collected_data
        // .entry(driver_family_name)
        // .or_insert(Stat::default());
    // family.score = driver_family.score;

    let RecordWidth(date_width, go_width, back_width, driver_width) = get_record_print_width();
    println!(
        "{:date_width$} {:go_width$} {:back_width$} {:driver_width$}",
        record.date, record.going, record.back, record.driver,
    );
    Ok(())
}

fn parse_rows(rows: &Vec<&str>) -> Option<HashMap<String, Stat>> {
    let mut collected_data = HashMap::<String, Stat>::new();
    // markdown format
    // 0. title
    // 1. |----|

    // print title
    if !rows.is_empty() {
        let RecordWidth(date_width, go_width, back_width, driver_width) = get_record_print_width();

        println!(
            "{:date_width$} {:go_width$} {:back_width$} {:driver_width$}",
            "date", "going", "back", "driver"
        );
    }

    // print data
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
    println!();
    println!("family  going back drive score");
    if stats.is_empty() {
        return;
    }

    for (name, s) in stats.iter() {
        println!(
            "{:<6}  {:<5} {:<4} {:<5} {:<5}",
            name, s.going, s.back, s.drive, s.score
        );
    }
}

fn main() {
    if let Some(stats) = parse_data_file(DATAFILE) {
        analyze(&stats);
    }
}
