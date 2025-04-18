use itertools::Itertools;
use thiserror::Error;

use crate::line::{Line, Lines};

pub trait TryIntoLines {
    fn into_lines(self) -> Result<Lines, TryIntoLinesError>;
}

#[derive(Debug, Error)]
pub enum TryIntoLinesError {}

impl TryIntoLines for String {
    fn into_lines(self) -> Result<Lines, TryIntoLinesError> {
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

        Ok(Lines::new(
            lines,
            categories,
            sub_categories,
            categories_totals,
            sub_categories_total,
        ))
    }
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
