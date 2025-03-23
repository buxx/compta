use std::{collections::HashSet, fs};

fn main() {
    let raw = fs::read_to_string("data.csv").expect("Failed to read file");
    let headers: Vec<&str> = raw.lines().next().unwrap().split(";").collect();
    
    let debit_index = find_index(&headers, "Debit");
    let category_index = find_index(&headers, "Categorie");
    let sub_category_index = find_index(&headers, "Sous categorie");

    let categories = extract_unique_values(&raw, category_index);
    let sub_categories = extract_unique_values(&raw, sub_category_index);

    println!("# Par catégories");
    print_totals(&raw, debit_index, category_index, &categories);

    println!();
    println!("# Par sous catégories");
    print_totals(&raw, debit_index, sub_category_index, &sub_categories);
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

fn print_totals(raw: &str, debit_index: usize, index: usize, values: &[String]) {
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

        println!("{}: {}", value, total);
    }
}