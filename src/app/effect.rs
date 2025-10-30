pub enum Effect {
    ClearLines,
    IncreaseScale,
    DecreaseScale,
    SelectCategory(Option<String>),
    SelectSubCategory(Option<String>),
    SetFilterText(String),
    RecomputeRecurring,
}
