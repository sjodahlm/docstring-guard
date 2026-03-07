pub mod python;
pub mod rust;
pub use python::checker::check_file_for_docstrings;
// pub use rust::checker::run;
pub mod utils;

#[derive(PartialEq, Debug)]
pub struct MissingDocstring {
    pub file_name: String,
    pub name: String,
    pub line_number: usize,
}
