mod model;
mod parser;


use std::fmt::Write;

use rowan:: {
    SmolStr,
    SyntaxKind,
    GreenNode,
    GreenNodeBuilder,
    SyntaxElement,
    SyntaxNode,
    SyntaxToken,
    TreeArc,
    TransparentNewType,
    WalkEvent,
};

use tera::*;
use crate::model::*;
use failure::format_err;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase, MixedCase};

macro_rules! ast_node {
    ($ast:ident, $kind:ident) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        #[repr(transparent)]
        struct $ast(SyntaxNode);
        unsafe impl TransparentNewType for $ast {
            type Repr = SyntaxNode;
        }
        impl $ast {
            #[allow(unused)]
            fn cast(node: &SyntaxNode) -> Option<&Self> {
                if node.kind() == $kind.into() {
                    Some(Self::from_repr(node))
                } else {
                    None
                }
            }
            #[allow(unused)]
            fn to_owned(&self) -> TreeArc<Self> {
                let owned = self.0.to_owned();
                TreeArc::cast(owned)
            }
        }
    };
}

macro_rules! ast_token {
    ($ast:ident, $kind:ident) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        #[repr(transparent)]
        struct $ast<'a>(SyntaxToken<'a>);
        unsafe impl<'a> TransparentNewType for $ast<'a> {
            type Repr = SyntaxToken<'a>;
        }
        impl<'a> $ast<'a> {
            #[allow(unused)]
            fn cast(node: SyntaxToken<'a>) -> Option<Self> {
                if node.kind() == $kind.into() {
                    Some($ast(node))
                } else {
                    None
                }
            }
            #[allow(unused)]
            fn cast_elem(elem: SyntaxElement<'a>) -> Option<Self> {
                match elem {
                    SyntaxElement::Token(t) => $ast::cast(t),
                    _ => None,
                }
            }

            #[allow(unused)]
            fn text(&self) -> &'a SmolStr {
                self.0.text()
            }
        }
    };
}


pub fn generate(asdl: &str, template: &str) -> String {
    let root: &parser::Root = &parser::parse(&asdl);
    let model = Asdl::from(root);
    let mut tera = Tera::default();
    tera.register_filter("camel", |arg, _| Ok(arg.as_str().unwrap().to_camel_case().into()));
    tera.register_filter("snake", |arg, _| Ok(arg.as_str().unwrap().to_snake_case().into()));
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
    use std::fs;

    #[test]
    fn simple_successful_test() {
        let asdl = r"
            stm = Compound(stm s1, stm* s2)
                | Single(stm)
            noFileds = One | Two | Tree
            prodType = (noFileds? f, stm s1)
            ";
        let root: &parser::Root = &parser::parse(&asdl);
        let model = Asdl::from(root);
        assert_snapshot_matches!("simple_successful_test_syntax", root.debug_dump());
        assert_debug_snapshot_matches!("simple_successful_test_model", model)
    }
}
