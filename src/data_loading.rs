use crate::hdcmodels::hdcmodels::Dataset;

pub fn load_validation_dataset() -> Dataset {
    Dataset::load("validation")
}
