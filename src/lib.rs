pub mod python;
pub use python::checker::check_file_for_docstrings;
pub mod utils;
pub struct MissingDocstring {
    pub file_name: String,
    pub name: String,
    pub line_number: usize,
}
