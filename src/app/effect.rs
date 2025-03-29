pub enum Effect {
    IncreaseScale,
    DecreaseScale,
    SelectCategory(Option<String>),
    SelectSubCategory(Option<String>),
    SetFilterText(String),
}
