use app::MyApp;
use eframe::egui;

mod app;
mod extract;
mod line;

const SIGNIFICANT: f32 = 175.0;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MaCompta🦀",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

// fn render(raw: &str) -> String {
//     let mut result = String::new();
//     let headers: Vec<&str> = raw.lines().next().unwrap().split(";").collect();

//     let debit_index = find_index(&headers, "Debit");
//     let category_index = find_index(&headers, "Categorie");
//     let sub_category_index = find_index(&headers, "Sous categorie");

//     let categories = extract_unique_values(raw, category_index);

//     result.push_str("# ➡ Par catégories\n");
//     result.push('\n');
//     result.push_str(&print_totals(raw, debit_index, category_index, &categories));

//     result.push('\n');
//     result.push_str("# ➡ Par sous catégories\n");
//     result.push('\n');
//     result.push_str(&print_totals2(
//         raw,
//         debit_index,
//         category_index,
//         categories,
//         sub_category_index,
//     ));

//     result.push('\n');
//     result.push_str(&format!("# 🔥 Dépenses >= {:>.2}\n", SIGNIFICANT));
//     result.push('\n');

//     let mut significatives = raw
//         .lines()
//         .skip(1)
//         .filter_map(|line| {
//             let debit_str = line.split(";").nth(debit_index).unwrap();
//             if !debit_str.trim().is_empty() {
//                 Some((line, debit_str.replace(",", ".").parse::<f32>().unwrap()))
//             } else {
//                 None
//             }
//         })
//         .filter(|(_, v)| (v.abs() * 100.) as i32 >= (SIGNIFICANT * 100.) as i32)
//         .collect::<Vec<(&str, f32)>>();
//     significatives.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));

//     result.push_str(&format!(
//         "|{}|\n",
//         raw.lines().next().unwrap().replace(";", "|")
//     ));
//     result.push_str(&format!(
//         "|{}|\n",
//         raw.lines()
//             .next()
//             .unwrap()
//             .split(";")
//             .map(|_| "-")
//             .collect::<Vec<&str>>()
//             .join("|")
//     ));
//     for (line, _v) in significatives {
//         result.push_str(&format!("|{}|\n", line.replace(";", "|")))
//     }

//     result
// }

// fn find_index(headers: &[&str], target: &str) -> usize {
//     headers
//         .iter()
//         .position(|&h| h == target)
//         .expect("Header not found")
// }

// fn extract_unique_values(raw: &str, index: usize) -> Vec<String> {
//     raw.lines()
//         .skip(1)
//         .filter_map(|line| line.split(";").nth(index).map(String::from))
//         .collect::<HashSet<_>>()
//         .into_iter()
//         .collect()
// }

// fn extract_unique_values2(
//     raw: &str,
//     index1: usize,
//     index1_value: &str,
//     index2: usize,
// ) -> Vec<String> {
//     raw.lines()
//         .skip(1)
//         .filter(|line| line.split(";").nth(index1).map(String::from).unwrap() == index1_value)
//         .filter_map(|line| line.split(";").nth(index2).map(String::from))
//         .collect::<HashSet<_>>()
//         .into_iter()
//         .collect()
// }

// fn print_totals(raw: &str, debit_index: usize, index: usize, values: &[String]) -> String {
//     let mut result = String::new();
//     let mut totals = vec![];

//     for value in values {
//         let total: f32 = raw
//             .lines()
//             .skip(1)
//             .filter(|line| line.split(";").nth(index).unwrap() == value)
//             .filter_map(|line| {
//                 let debit_str = line.split(";").nth(debit_index).unwrap();
//                 if !debit_str.trim().is_empty() {
//                     debit_str.replace(",", ".").parse::<f32>().ok()
//                 } else {
//                     None
//                 }
//             })
//             .sum();

//         totals.push((value, total));
//         totals.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
//     }

//     result.push_str("||||\n");
//     result.push_str("|-|-|-|\n");
//     for (name, total) in totals {
//         result.push_str(&format!("|{}|{:>.2}|\n", name, total.abs()));
//     }

//     result
// }

// fn print_totals2(
//     raw: &str,
//     debit_index: usize,
//     category_index: usize,
//     categories: Vec<String>,
//     sub_category_index: usize,
// ) -> String {
//     let mut result = String::new();

//     for category in categories {
//         result.push_str(&format!("## {}\n", category));
//         result.push_str("||||\n");
//         result.push_str("|-|-|-|\n");

//         let sub_categories =
//             extract_unique_values2(raw, category_index, &category, sub_category_index);

//         let mut totals = vec![];
//         for sub_category in sub_categories {
//             let total: f32 = raw
//                 .lines()
//                 .skip(1)
//                 .filter(|line| line.split(";").nth(sub_category_index).unwrap() == sub_category)
//                 .filter_map(|line| {
//                     let debit_str = line.split(";").nth(debit_index).unwrap();
//                     if !debit_str.trim().is_empty() {
//                         debit_str.replace(",", ".").parse::<f32>().ok()
//                     } else {
//                         None
//                     }
//                 })
//                 .sum();

//             totals.push((sub_category, total));
//             totals.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
//         }

//         for (name, total) in &totals {
//             result.push_str(&format!("|{}|{:>.2}|\n", name, total.abs()));
//         }
//     }

//     result
// }
