use derive_more::Constructor;

#[derive(Debug, Constructor, PartialEq)]
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

#[derive(Debug, Constructor)]
pub struct Lines {
    lines: Vec<Line>,
    categories: Vec<String>,
    sub_categories: Vec<(String, String)>,
    categories_totals: Vec<(String, f32)>,
    sub_categories_total: Vec<(String, String, f32)>,
    categories_histogram: Vec<(String, Vec<[f64; 2]>)>,
}

impl Lines {
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    pub fn sub_categories(&self) -> &[(String, String)] {
        &self.sub_categories
    }

    pub fn categories_totals(&self) -> &[(String, f32)] {
        &self.categories_totals
    }

    pub fn sub_categories_total(&self) -> &[(String, String, f32)] {
        &self.sub_categories_total
    }

    pub fn categories_histogram(&self) -> &Vec<(String, Vec<[f64; 2]>)> {
        &self.categories_histogram
    }
}
