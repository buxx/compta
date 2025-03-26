use std::{collections::HashSet, fs};

const SIGNIFICANT: f32 = 250.0;

fn main() {
    let raw = fs::read_to_string("data.csv").expect("Failed to read file");
    let headers: Vec<&str> = raw.lines().next().unwrap().split(";").collect();
    
    let debit_index = find_index(&headers, "Debit");
    let category_index = find_index(&headers, "Categorie");
    let sub_category_index = find_index(&headers, "Sous categorie");

    let categories = extract_unique_values(&raw, category_index);
    let sub_categories = extract_unique_values(&raw, sub_category_index);

    println!("# ➡️  Par catégories");
    println!();
    print_totals(&raw, debit_index, category_index, &categories);

    println!();
    println!("# ➡️  Par sous catégories");
    println!();
    print_totals(&raw, debit_index, sub_category_index, &sub_categories);

    println!();
    println!("# 🔥  Dépenses/Entrées >= {:>.2}", SIGNIFICANT);
    println!();

    
    let mut significatives = raw.lines().skip(1)
        .filter_map(|line| {
            let debit_str = line.split(";").nth(debit_index).unwrap();
            if !debit_str.trim().is_empty() {
                Some((line, debit_str.replace(",", ".").parse::<f32>().unwrap()))
            } else {
                None
            }
        }).filter(|(_, v)| (v.abs() * 100.) as i32 >= SIGNIFICANT as i32).collect::<Vec<(&str, f32)>>();
    significatives.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
    for (line, _v) in significatives {
        println!("* `{}`", line)
    }
}


fn find_index(headers: &[&str], target: &str) -> usize {
    headers.iter().position(|&h| h == target).expect("Header not found")
}

fn extract_unique_values(raw: &str, index: usize) -> Vec<String> {
    raw.lines()
        .skip(1)
        .filter_map(|line| line.split(";").nth(index).map(String::from))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

// fn extract_column(raw: &str, column_index: usize) -> Vec<Item> {

// }

fn print_totals(raw: &str, debit_index: usize, index: usize, values: &[String]) {
    let mut totals = vec![];

    for value in values {
        let total: f32 = raw.lines()
            .skip(1)
            .filter(|line| line.split(";").nth(index).unwrap() == value)
            .filter_map(|line| {
                let debit_str = line.split(";").nth(debit_index).unwrap();
                if !debit_str.trim().is_empty() {
                    debit_str.replace(",", ".").parse::<f32>().ok()
                } else {
                    None
                }
            })
            .sum();

        totals.push((value, total));
        totals.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
    }

    println!("||||");
    println!("|-|-|-|");
    for (name, total) in totals {
        println!("|{}|{:>.2}|", name, total.abs());
    }
}