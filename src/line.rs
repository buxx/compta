use derive_more::Constructor;
use std::hash::{Hash, Hasher};

#[derive(Debug, Constructor, PartialEq, Clone)]
pub struct Line {
    date_raw: String,
    libelle_simplifie: String,
    libelle_operation: String,
    reference: String,
    categorie: String,
    sous_categorie: String,
    debit: Option<f32>,
    credit: Option<f32>,
}

impl Line {
    pub fn libelle_simplifie(&self) -> &str {
        &self.libelle_simplifie
    }

    pub fn libelle_operation(&self) -> &str {
        &self.libelle_operation
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn categorie(&self) -> &str {
        &self.categorie
    }

    pub fn sous_categorie(&self) -> &str {
        &self.sous_categorie
    }

    pub fn debit(&self) -> Option<f32> {
        self.debit
    }

    pub fn credit(&self) -> Option<f32> {
        self.credit
    }

    pub fn date_raw(&self) -> &str {
        &self.date_raw
    }
}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date_raw.hash(state);
        self.libelle_simplifie.hash(state);
        self.libelle_operation.hash(state);
        self.reference.hash(state);
        self.categorie.hash(state);
        self.sous_categorie.hash(state);
        self.debit.map(|v| (v * 100.0) as i32).hash(state);
        self.credit.map(|v| (v * 100.0) as i32).hash(state);
    }
}

impl Eq for Line {}

#[derive(Debug, Constructor)]
pub struct Lines {
    name: String,
    pub lower_date: chrono::NaiveDate,
    pub higher_date: chrono::NaiveDate,
    lines: Vec<Line>,
    pub recurring: Vec<Line>,
    categories: Vec<String>,
    sub_categories: Vec<(String, String)>,
    categories_totals: Vec<(String, Vec<f32>, f32, f32)>,
    sub_categories_total: Vec<(String, String, Vec<f32>, f32, f32)>,
    categories_histogram: Vec<(String, bool, Vec<[f64; 2]>)>,
    sous_categories_histogram: Vec<(String, String, bool, Vec<[f64; 2]>)>,
    pub months_count: usize,
    pub all_months: Vec<chrono::NaiveDate>,
    pub active_months: Vec<chrono::NaiveDate>,
    pub recurring_months: usize,
    pub recurring_approx: f32,
    pub categories_histogram_display_expenses_only: bool,
    pub months_sums: Vec<(chrono::NaiveDate, f32)>,
}

impl Lines {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn recurring(&self) -> &[Line] {
        &self.recurring
    }

    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    pub fn sub_categories(&self) -> &[(String, String)] {
        &self.sub_categories
    }

    pub fn categories_totals(&self) -> &[(String, Vec<f32>, f32, f32)] {
        &self.categories_totals
    }

    pub fn sub_categories_total(&self) -> &[(String, String, Vec<f32>, f32, f32)] {
        &self.sub_categories_total
    }

    pub fn categories_histogram(&self) -> &[(String, bool, Vec<[f64; 2]>)] {
        &self.categories_histogram
    }

    pub fn sous_categories_histogram(&self) -> &[(String, String, bool, Vec<[f64; 2]>)] {
        &self.sous_categories_histogram
    }

    pub fn months_count(&self) -> usize {
        self.months_count
    }

    pub fn all_months(&self) -> &Vec<chrono::NaiveDate> {
        &self.all_months
    }

    pub fn active_months(&self) -> &[chrono::NaiveDate] {
        &self.active_months
    }

    pub fn months_sums(&self) -> &Vec<(chrono::NaiveDate, f32)> {
        &self.months_sums
    }
}
