pub mod ast;
mod parser;
pub mod model;
mod model_impl;
pub mod util;

use std::error::Error;
use std::fmt;

pub use model::Asdl;

pub type Result<T> = std::result::Result<T, AsdlError>;

pub fn ast(asdl: &str) -> Result<ast::Root> {
    let (_, root) = parser::parse(asdl).unwrap();
    Ok(root)
}

pub fn model(asdl: &str) -> Result<Asdl> {
    ast(asdl).map(|a| Asdl::new(&a))
}

#[derive(Debug)]
pub struct AsdlError {
    details: String,
}

impl fmt::Display for AsdlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AsdlError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_debug_snapshot_matches;

    #[test]
    fn simple_successful_test() {
        let asdl = r"
            stm = Compound(stm s1, stm* s2)
                | Single(stm)
            noFileds = One | Two | Three
            prodType = (noFileds? f, stm s1)
            ";
        let (_, root) = parser::parse(&asdl).unwrap();
        assert_debug_snapshot_matches!("simple_successful_test_syntax", root);
        let model = Asdl::new(&root);
        assert_debug_snapshot_matches!("simple_successful_test_model", model)
    }

    #[test]
    fn attributes() {
        let asdl = r"
            stm = Compound(stm s1, stm* s2)
                | Single(stm)
                  attributes(prodType?)
            prodType = (stm s1)
            ";
        let (_, root) = parser::parse(&asdl).unwrap();
        assert_debug_snapshot_matches!("attributes_syntax", root);
        let model = Asdl::new(&root);
        assert_debug_snapshot_matches!("attributes_model", model)
    }
}