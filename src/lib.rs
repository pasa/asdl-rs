mod model;
mod parser;

use tera::*;
use crate::model::*;
use failure::format_err;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase, MixedCase};

pub fn generate(asdl: &str, template: &str) -> String {
    let root: &parser::Root = &parser::parse(&asdl);
    let model = Asdl::from(root);
    let mut tera = Tera::default();
    tera.register_filter("camel", |arg, _| Ok(arg.as_str().unwrap().to_camel_case().into()));
    tera.register_filter("snake", |arg, _| Ok(arg.as_str().unwrap().to_snake_case().into()));
    tera.register_filter("mixed", |arg, _| Ok(arg.as_str().unwrap().to_mixed_case().into()));
    tera.register_filter("SCREAM", |arg, _| {
        Ok(arg.as_str().unwrap().to_shouty_snake_case().into())
    });
    tera.add_raw_template("_src", &template)
        .map_err(|e| format_err!("template parsing error: {:?}", e))
        .unwrap();
    tera.render("_src", &model)
        .map_err(|e| format_err!("template rendering error: {:?}", e))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_snapshot_matches;
    use insta::assert_debug_snapshot_matches;

    #[test]
    fn simple_successful_test() {
        let asdl = r"
            stm = Compound(stm s1, stm* s2)
                | Single(stm)
            noFileds = One | Two | Tree
            prodType = (noFileds? f, stm s1)
            ";
        let root: &parser::Root = &parser::parse(&asdl);
        assert_snapshot_matches!("simple_successful_test_syntax", root.debug_dump());
        let model = Asdl::from(root);
        assert_debug_snapshot_matches!("simple_successful_test_model", model)
    }
}
