use std::{collections::HashSet, fs};

fn main() {
    let raw = fs::read_to_string("data.csv").unwrap();
    let debit_index = raw
        .lines()
        .next()
        .unwrap()
        .split(";")
        .enumerate()
        .find_map(|(l, v)| if v == "Debit" { Some(l) } else { None })
        .unwrap();
    let category_index = raw
        .lines()
        .next()
        .unwrap()
        .split(";")
        .enumerate()
        .find_map(|(l, v)| if v == "Categorie" { Some(l) } else { None })
        .unwrap();
    let sub_category_index = raw
        .lines()
        .next()
        .unwrap()
        .split(";")
        .enumerate()
        .find_map(|(l, v)| if v == "Sous categorie" { Some(l) } else { None })
        .unwrap();
    let categories = raw
        .lines()
        .skip(1)
        .map(|l| {
            l.split(";")
                .collect::<Vec<&str>>()
                .get(category_index)
                .unwrap()
                .clone()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let sub_categories = raw
        .lines()
        .skip(1)
        .map(|l| {
            l.split(";")
                .collect::<Vec<&str>>()
                .get(sub_category_index)
                .unwrap()
                .clone()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    println!("# Par catégories");
    for category in categories {
        let total: f32 = raw
            .lines()
            .skip(1)
            .filter(|l| {
                l.split(";")
                    .collect::<Vec<&str>>()
                    .get(category_index)
                    .unwrap()
                    == &category
            })
            .map(|l| {
                l.split(";")
                    .collect::<Vec<&str>>()
                    .get(debit_index)
                    .unwrap()
                    .clone()
            })
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.replace(",", ".").parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
            .iter()
            .sum();

        println!("{category}: {total}")
    }

    println!("");
    println!("# Par sous catégories");
    for sub_category in sub_categories {
        let total: f32 = raw
            .lines()
            .skip(1)
            .filter(|l| {
                l.split(";")
                    .collect::<Vec<&str>>()
                    .get(sub_category_index)
                    .unwrap()
                    == &sub_category
            })
            .map(|l| {
                l.split(";")
                    .collect::<Vec<&str>>()
                    .get(debit_index)
                    .unwrap()
                    .clone()
            })
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.replace(",", ".").parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
            .iter()
            .sum();

        println!("{sub_category}: {total}")
    }
}
