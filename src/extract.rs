use std::collections::HashSet;

use chrono::Datelike;
use itertools::Itertools;
use thiserror::Error;

use crate::line::{Line, Lines};

pub trait TryIntoLines {
    fn into_lines(self, name: String) -> Result<Lines, TryIntoLinesError>;
}

#[derive(Debug, Error)]
pub enum TryIntoLinesError {}

impl TryIntoLines for String {
    fn into_lines(self, name: String) -> Result<Lines, TryIntoLinesError> {
        let mut lines = vec![];

        for line in self.lines().skip(1) {
            let columns: Vec<&str> = line.split(";").collect();

            let date_raw = columns[0];
            let libelle_simplifie = columns[1];
            let libelle_operation = columns[2];
            let reference = columns[3];
            let categorie = columns[6];
            let sous_categorie = columns[7];
            let debit = columns[8];
            let credit = columns[9];

            lines.push(Line::new(
                date_raw.to_string(),
                libelle_simplifie.to_string(),
                libelle_operation.to_string(),
                reference.to_string(),
                categorie.to_string(),
                sous_categorie.to_string(),
                debit.replace(",", ".").parse::<f32>().ok(),
                credit.replace(",", ".").parse::<f32>().ok(),
            ));
        }
        let categories: Vec<String> = lines
            .iter()
            .map(|l| l.categorie().to_string())
            .collect::<Vec<String>>()
            .into_iter()
            .unique()
            .collect();

        let mut sub_categories: Vec<(String, String)> = vec![];
        for category in &categories {
            for sub_category in lines
                .iter()
                .filter(|l| l.categorie() == category)
                .map(|l| l.sous_categorie().to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .unique()
                .collect::<Vec<String>>()
            {
                sub_categories.push((category.clone(), sub_category))
            }
        }

        let categories_totals: Vec<(String, f32)> = categories
            .iter()
            .map(|cat| {
                (
                    cat.clone(),
                    lines
                        .iter()
                        .filter(|l| l.categorie() == cat)
                        .map(|l| l.credit().unwrap_or_default() + l.debit().unwrap_or_default())
                        .sum(),
                )
            })
            .collect::<Vec<(String, f32)>>()
            .into_iter()
            .sorted_by_key(|(_, v)| (v * 100.) as i32)
            .collect();
        let sub_categories_total: Vec<(String, String, f32)> = sub_categories
            .iter()
            .map(|(cat, sub)| {
                (
                    cat.clone(),
                    sub.clone(),
                    lines
                        .iter()
                        .filter(|l| l.sous_categorie() == sub)
                        .map(|l| l.credit().unwrap_or_default() + l.debit().unwrap_or_default())
                        .sum(),
                )
            })
            .collect::<Vec<(String, String, f32)>>()
            .into_iter()
            .sorted_by_key(|(_, _, v)| (v * 100.) as i32)
            .collect();

        let dates = lines
            .iter()
            .map(|l| {
                let mut splitted = l.date_raw().split('/');
                let day = splitted.next().unwrap().parse::<u32>().unwrap();
                let month = splitted.next().unwrap().parse::<u32>().unwrap();
                let year = splitted.next().unwrap().parse::<i32>().unwrap();
                chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap()
            })
            .collect::<Vec<chrono::NaiveDate>>();
        let lower_date = dates
            .iter()
            .sorted()
            .collect::<Vec<&chrono::NaiveDate>>()
            .first()
            .copied()
            .unwrap()
            .clone();
        let higher_date = dates
            .iter()
            .sorted()
            .collect::<Vec<&chrono::NaiveDate>>()
            .last()
            .copied()
            .unwrap()
            .clone();
        let mut months = 1;

        let mut categories_histogram = vec![];
        for category in &categories {
            let category_lines = lines
                .iter()
                .filter(|l| l.categorie() == category)
                .collect::<Vec<&Line>>();

            let mut index = 0;
            let mut values = vec![];
            let mut current_date =
                chrono::NaiveDate::from_ymd_opt(lower_date.year(), lower_date.month(), 1).unwrap();
            while current_date <= higher_date {
                let category_month_total = category_lines
                    .iter()
                    .filter(|l| {
                        let mut splitted = l.date_raw().split('/');
                        let _ = splitted.next().unwrap().parse::<u32>().unwrap();
                        let month = splitted.next().unwrap().parse::<u32>().unwrap();
                        let year = splitted.next().unwrap().parse::<i32>().unwrap();
                        current_date.year() == year && current_date.month() == month
                    })
                    .map(|l| l.credit().unwrap_or(0.0) + l.debit().unwrap_or(0.0))
                    .sum::<f32>();

                values.push([(index + 1) as f64, category_month_total as f64]);

                current_date = current_date
                    .checked_add_months(chrono::Months::new(1))
                    .unwrap();
                index += 1;
            }

            let positive = values
                .iter()
                .map(|[_, v]| v)
                .sum::<f64>()
                .is_sign_positive();

            categories_histogram.push((category.clone(), positive, values));
            months = index;
        }

        let recurring_months = (((months as isize) - 1).max(1)) as usize;

        let mut sous_categories_histogram = vec![];
        for (category, sub_category) in &sub_categories {
            let sub_category_lines = lines
                .iter()
                .filter(|l| l.categorie() == category && l.sous_categorie() == sub_category)
                .collect::<Vec<&Line>>();

            let mut index = 0;
            let mut values = vec![];
            let mut current_date =
                chrono::NaiveDate::from_ymd_opt(lower_date.year(), lower_date.month(), 1).unwrap();
            while current_date <= higher_date {
                let sub_category_month_total = sub_category_lines
                    .iter()
                    .filter(|l| {
                        let mut splitted = l.date_raw().split('/');
                        let _ = splitted.next().unwrap().parse::<u32>().unwrap();
                        let month = splitted.next().unwrap().parse::<u32>().unwrap();
                        let year = splitted.next().unwrap().parse::<i32>().unwrap();
                        current_date.year() == year && current_date.month() == month
                    })
                    .map(|l| l.credit().unwrap_or(0.0) + l.debit().unwrap_or(0.0))
                    .sum::<f32>();

                values.push([(index + 1) as f64, sub_category_month_total as f64]);

                current_date = current_date
                    .checked_add_months(chrono::Months::new(1))
                    .unwrap();
                index += 1;
            }

            let positive = values
                .iter()
                .map(|[_, v]| v)
                .sum::<f64>()
                .is_sign_positive();

            sous_categories_histogram.push((
                category.clone(),
                sub_category.clone(),
                positive,
                values,
            ));
        }

        let mut months_sums = vec![];
        let mut current_date =
            chrono::NaiveDate::from_ymd_opt(lower_date.year(), lower_date.month(), 1).unwrap();
        while current_date <= higher_date {
            let month_total = lines
                .iter()
                .filter(|l| {
                    let mut splitted = l.date_raw().split('/');
                    let _ = splitted.next().unwrap().parse::<u32>().unwrap();
                    let month = splitted.next().unwrap().parse::<u32>().unwrap();
                    let year = splitted.next().unwrap().parse::<i32>().unwrap();
                    current_date.year() == year
                        && current_date.month() == month
                        && l.categorie() != "Transaction exclue"
                })
                .map(|l| l.credit().unwrap_or(0.0) + l.debit().unwrap_or(0.0))
                .sum::<f32>();

            months_sums.push(month_total);

            current_date = current_date
                .checked_add_months(chrono::Months::new(1))
                .unwrap();
        }

        let recurring_approx = 0.0;
        let mut lines = Lines::new(
            name,
            lower_date,
            higher_date,
            lines,
            vec![],
            categories,
            sub_categories,
            categories_totals,
            sub_categories_total,
            categories_histogram,
            sous_categories_histogram,
            months,
            recurring_months,
            recurring_approx,
            true,
            months_sums,
        );

        let recurring = extract_recuring(&lines);
        lines.recurring = recurring;

        Ok(lines)
    }
}

pub fn extract_recuring(lines: &Lines) -> Vec<Line> {
    let mut recurring = vec![];

    for line in lines.lines() {
        let mut found_counter = 0;
        let mut current_date =
            chrono::NaiveDate::from_ymd_opt(lines.lower_date.year(), lines.lower_date.month(), 1)
                .unwrap();
        while current_date <= lines.higher_date {
            let month_lines = lines
                .lines()
                .iter()
                .filter(|l| {
                    let mut splitted = l.date_raw().split('/');
                    let _ = splitted.next().unwrap().parse::<u32>().unwrap();
                    let month = splitted.next().unwrap().parse::<u32>().unwrap();
                    let year = splitted.next().unwrap().parse::<i32>().unwrap();
                    current_date.year() == year && current_date.month() == month
                })
                .collect::<Vec<&Line>>();

            if !month_lines
                .iter()
                .filter(|l| {
                    l.libelle_simplifie() == line.libelle_simplifie()
                        && approx_eq_pct_ref(
                            l.debit().unwrap_or(0.0),
                            line.debit().unwrap_or(0.0),
                            lines.recurring_approx,
                        )
                        && approx_eq_pct_ref(
                            l.credit().unwrap_or(0.0),
                            line.credit().unwrap_or(0.0),
                            lines.recurring_approx,
                        )
                })
                .collect::<Vec<&&Line>>()
                .is_empty()
            {
                found_counter += 1;
            }
            current_date = current_date
                .checked_add_months(chrono::Months::new(1))
                .unwrap();
        }

        if found_counter >= lines.recurring_months {
            recurring.push(line.clone())
        }
    }

    let mut recurring_: Vec<Line> = vec![];
    for line in &recurring {
        let already = recurring_
            .iter()
            .find(|l| {
                l.libelle_simplifie() == line.libelle_simplifie()
                    && approx_eq_pct_ref(
                        l.debit().unwrap_or(0.0),
                        line.debit().unwrap_or(0.0),
                        lines.recurring_approx,
                    )
                    && approx_eq_pct_ref(
                        l.credit().unwrap_or(0.0),
                        line.credit().unwrap_or(0.0),
                        lines.recurring_approx,
                    )
            })
            .is_some();

        if !already {
            recurring_.push(line.clone());
        }
    }

    recurring_
}

pub fn approx_eq_pct_ref(reference: f32, actual: f32, pct: f32) -> bool {
    if reference.is_nan() || actual.is_nan() {
        return false;
    }
    if reference.is_infinite() || actual.is_infinite() {
        return reference == actual;
    }

    let ref_abs = reference.abs();
    if ref_abs == 0.0 {
        return (actual).abs() <= f32::EPSILON;
    } // compare to tiny epsilon if reference is 0
    let tol = ref_abs * pct;
    (reference - actual).abs() <= tol
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = "Date de comptabilisation;Libelle simplifie;Libelle operation;Reference;Informations complementaires;Type operation;Categorie;Sous categorie;Debit;Credit;Date operation;Date de valeur;Pointage operation
22/03/2025;AREA;CB AREA NFC FACT 200325;;;Carte bancaire;Transports;Peage et Stationnement;-2,4;;20/03/2025;24/03/2025;0
22/03/2025;AREA;CB AREA NFC FACT 200325;;;Carte bancaire;Transports;Peage et Stationnement;-2,4;;20/03/2025;24/03/2025;0
22/03/2025;BOULANGERIE DU;CB BOULANGERIE DU FACT 200325;;;Carte bancaire;Alimentation;Alimentation - autre;-3,45;;20/03/2025;24/03/2025;0
21/03/2025;VIREMENT VERS CPT DEPOT PART.;VIREMENT VERS CPT DEPOT PART.;2508085IN0103420;;Virement recu;Transaction exclue;Virement interne;;800;21/03/2025;21/03/2025;0
21/03/2025;RENFLOUEMENT;RENFLOUEMENT;2508085IN0073599;;Virement recu;Transaction exclue;Virement interne;;400;21/03/2025;21/03/2025;0
21/03/2025;INTERMARCHE;CB INTERMARCHE FACT 190325;;;Carte bancaire;Alimentation;Hyper/supermarche;-76,18;;19/03/2025;21/03/2025;0
";

    #[test]
    fn extract_by_category() {
        // Given/When
        let lines = RAW.to_string().into_lines().unwrap();

        // Then
        assert_eq!(
            lines.lines(),
            vec![
                Line::new(
                    "22/03/2025".to_string(),
                    "AREA".to_string(),
                    "CB AREA NFC FACT 200325".to_string(),
                    "".to_string(),
                    "Transports".to_string(),
                    "Peage et Stationnement".to_string(),
                    Some(-2.4),
                    None
                ),
                Line::new(
                    "22/03/2025".to_string(),
                    "AREA".to_string(),
                    "CB AREA NFC FACT 200325".to_string(),
                    "".to_string(),
                    "Transports".to_string(),
                    "Peage et Stationnement".to_string(),
                    Some(-2.4),
                    None
                ),
                Line::new(
                    "22/03/2025".to_string(),
                    "BOULANGERIE DU".to_string(),
                    "CB BOULANGERIE DU FACT 200325".to_string(),
                    "".to_string(),
                    "Alimentation".to_string(),
                    "Alimentation - autre".to_string(),
                    Some(-3.45),
                    None
                ),
                Line::new(
                    "21/03/2025".to_string(),
                    "VIREMENT VERS CPT DEPOT PART.".to_string(),
                    "VIREMENT VERS CPT DEPOT PART.".to_string(),
                    "2508085IN0103420".to_string(),
                    "Transaction exclue".to_string(),
                    "Virement interne".to_string(),
                    None,
                    Some(800.0)
                ),
                Line::new(
                    "21/03/2025".to_string(),
                    "RENFLOUEMENT".to_string(),
                    "RENFLOUEMENT".to_string(),
                    "2508085IN0073599".to_string(),
                    "Transaction exclue".to_string(),
                    "Virement interne".to_string(),
                    None,
                    Some(400.0)
                ),
                Line::new(
                    "21/03/2025".to_string(),
                    "INTERMARCHE".to_string(),
                    "CB INTERMARCHE FACT 190325".to_string(),
                    "".to_string(),
                    "Alimentation".to_string(),
                    "Hyper/supermarche".to_string(),
                    Some(-76.18),
                    None
                )
            ]
        )
    }
}
