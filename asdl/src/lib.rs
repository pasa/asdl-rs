mod ast;
mod parser;
mod model;
mod model_impl;
mod util;

pub use model::*;

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_debug_snapshot_matches;

    #[test]
    fn simple_successful_test() {
        let asdl = r"
            // Root comment line 1
            // Root comment line 2

            // stm comment line 1
            // stm comment line 2
            stm = 
                // Compound comment line 1
                // Compound comment line 2
                Compound(stm s1, stm* s2) |
                // Single comment line 1
                // Single comment line 2 
                Single(stm)

            // noFields comment line 1
            // noFields comment line 2
            noFields = One | Two | Three

            // prodType comment line 1
            // prodType comment line 2
            prodType = (noFields? f, stm s1)
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
