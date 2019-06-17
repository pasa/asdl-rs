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
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};

const L_PAREN: SyntaxKind = SyntaxKind(0); // '('
const R_PAREN: SyntaxKind = SyntaxKind(1); // ')'
const PIPE: SyntaxKind = SyntaxKind(2); // '|'
const Q_MARK: SyntaxKind = SyntaxKind(3); // '?'
const STAR: SyntaxKind = SyntaxKind(4); // '*'
const COMMA: SyntaxKind = SyntaxKind(5); // ','
const TYPE_ID: SyntaxKind = SyntaxKind(6);
const CONSTR_ID: SyntaxKind = SyntaxKind(7);
const WHITESPACE: SyntaxKind = SyntaxKind(8); // whitespaces is explicit
const EQ: SyntaxKind = SyntaxKind(9); // '='
const ERROR: SyntaxKind = SyntaxKind(10); // as well as errors

//composite syntax kinds

const ROOT: SyntaxKind = SyntaxKind(21);
const DEF: SyntaxKind = SyntaxKind(22);
const SUM_TYPE: SyntaxKind = SyntaxKind(23);
const PROD_TYPE: SyntaxKind = SyntaxKind(24);
const CONSTR: SyntaxKind = SyntaxKind(25);
const FIELDS: SyntaxKind = SyntaxKind(26);
const FIELD: SyntaxKind = SyntaxKind(27);
const ID: SyntaxKind = SyntaxKind(28);

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

ast_node!(Root, ROOT);
ast_node!(Def, DEF);
ast_node!(SumType, SUM_TYPE);
ast_node!(ProdType, PROD_TYPE);
ast_node!(Constr, CONSTR);
ast_node!(Fields, FIELDS);
ast_node!(Field, FIELD);

ast_token!(TypeId, TYPE_ID);
ast_token!(ConstrId, CONSTR_ID);
ast_token!(Id, ID);
ast_token!(Star, STAR);
ast_token!(QMark, Q_MARK);

#[derive(Debug)]
struct Type(SyntaxNode);

enum TypeKind<'a> {
    Sum(&'a SumType),
    Prod(&'a ProdType),
}

impl Type {
    fn cast(node: &SyntaxNode) -> Option<&Self> {
        if SumType::cast(node).is_some() || ProdType::cast(node).is_some() {
            Some(unsafe { std::mem::transmute(node) })
        } else {
            None
        }
    }

    fn kind(&self) -> TypeKind {
        SumType::cast(&self.0)
            .map(TypeKind::Sum)
            .or_else(|| ProdType::cast(&self.0).map(TypeKind::Prod))
            .unwrap()
    }
}

impl Root {
    fn defs(&self) -> impl Iterator<Item = &Def> {
        self.0.children().filter_map(Def::cast)
    }

    pub fn preorder_with_tokens(&self) -> impl Iterator<Item = WalkEvent<SyntaxElement>> {
        self.0.preorder_with_tokens().map(|event| match event {
            WalkEvent::Enter(n) => WalkEvent::Enter(n.into()),
            WalkEvent::Leave(n) => WalkEvent::Leave(n.into()),
        })
    }

    pub fn debug_dump(&self) -> String {
        let mut level = 0;
        let mut buf = String::new();
        macro_rules! indent {
            () => {
                for _ in 0..level {
                    buf.push_str("  ");
                }
            };
        }

        for event in self.preorder_with_tokens() {
            match event {
                WalkEvent::Enter(element) => {
                    indent!();
                    match element {
                        SyntaxElement::Node(node) => writeln!(buf, "{:?}", node).unwrap(),
                        SyntaxElement::Token(token) => {
                            writeln!(buf, "{:?}", token).unwrap();
                            let _off = token.range().end();
                        }
                    }
                    level += 1;
                }
                WalkEvent::Leave(_) => level -= 1,
            }
        }

        assert_eq!(level, 0);
        buf
    }
}

impl Def {
    fn type_id(&self) -> TypeId {
        self.0.children_with_tokens().find_map(TypeId::cast_elem).unwrap()
    }

    fn ty(&self) -> &Type {
        self.0.children().find_map(Type::cast).unwrap()
    }
}

impl SumType {
    fn constructors(&self) -> impl Iterator<Item = &Constr> {
        self.0.children().filter_map(Constr::cast)
    }
}

impl Constr {
    fn id(&self) -> ConstrId {
        self.0.children_with_tokens().find_map(ConstrId::cast_elem).unwrap()
    }

    fn fields(&self) -> Option<&Fields> {
        self.0.children().find_map(Fields::cast)
    }
}

impl ProdType {
    fn fields(&self) -> &Fields {
        self.0.children().find_map(Fields::cast).unwrap()
    }
}

impl Fields {
    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
}

impl Field {
    fn type_id(&self) -> TypeId {
        self.0.children_with_tokens().find_map(TypeId::cast_elem).unwrap()
    }

    fn id(&self) -> Option<Id> {
        self.0.children_with_tokens().find_map(Id::cast_elem)
    }

    fn star(&self) -> Option<Star> {
        self.0.children_with_tokens().find_map(Star::cast_elem)
    }

    fn q_mark(&self) -> Option<QMark> {
        self.0.children_with_tokens().find_map(QMark::cast_elem)
    }
}

struct Parser {
    /// input tokens, including whitespace,
    /// in *reverse* order.
    tokens: Vec<(SyntaxKind, SmolStr)>,
    /// the in-progress tree.
    builder: GreenNodeBuilder,
    /// the list of syntax errors we've accumulated
    /// so far.
    errors: Vec<String>,
}

enum ParseStatus {
    Eof,
    Ok,
}

impl Parser {
    fn parse(mut self) -> TreeArc<Root> {
        self.builder.start_node(ROOT.into());
        self.skip_ws();
        loop {
            match self.def() {
                ParseStatus::Ok => (),
                ParseStatus::Eof => {
                    break;
                }
            }
        }
        self.builder.finish_node();
        let green: GreenNode = self.builder.finish();
        let node = SyntaxNode::new(green, Some(Box::new(self.errors)));
        Root::cast(&node).unwrap().to_owned()
    }

    fn def(&mut self) -> ParseStatus {
        self.skip_ws();
        let t = match self.current() {
            None => return ParseStatus::Eof,
            Some(t) => t,
        };
        match t {
            TYPE_ID => {
                self.builder.start_node(DEF);
                self.bump();
                self.skip_ws();
                match self.current() {
                    Some(EQ) => self.bump(),
                    None => return ParseStatus::Eof,
                    Some(_) => self.errors.push("expected `=`".to_string()),
                }
                self.type_def();
                self.builder.finish_node();
            }
            ERROR => self.bump(),
            _ => unreachable!(),
        }
        ParseStatus::Ok
    }

    fn type_def(&mut self) -> ParseStatus {
        self.skip_ws();
        match self.current() {
            Some(CONSTR_ID) => {
                self.builder.start_node(SUM_TYPE.into());
                self.constructors();
            }
            Some(L_PAREN) => {
                self.builder.start_node(PROD_TYPE.into());
                self.fields();
            }
            Some(_) => self.errors.push("expected type declaration".to_string()),
            None => return ParseStatus::Eof,
        }
        self.builder.finish_node();
        ParseStatus::Ok
    }

    fn constructors(&mut self) -> ParseStatus {
        self.constructor();
        loop {
            self.skip_ws();
            match self.current() {
                Some(PIPE) => {
                    self.bump();
                    self.constructor();
                }
                Some(_) => {
                    break;
                }
                None => return ParseStatus::Eof,
            }
        }
        ParseStatus::Ok
    }

    fn constructor(&mut self) -> ParseStatus {
        self.skip_ws();
        match self.current() {
            Some(CONSTR_ID) => {
                self.builder.start_node(CONSTR.into());
                self.bump();
                self.skip_ws();
                if let Some(L_PAREN) = self.current() {
                    self.fields();
                }
            }
            Some(_) => self.errors.push("expected constructor id".to_string()),
            None => return ParseStatus::Eof,
        }
        self.builder.finish_node();
        ParseStatus::Ok
    }

    fn fields(&mut self) -> ParseStatus {
        self.skip_ws();
        if let Some(L_PAREN) = self.current() {
            self.builder.start_node(FIELDS.into());
            self.bump();
            self.skip_ws();
            self.field();
            loop {
                self.skip_ws();
                match self.current() {
                    Some(COMMA) => {
                        self.bump();
                        self.field();
                    }
                    Some(R_PAREN) => {
                        self.bump();
                        break;
                    }
                    Some(_) => self.errors.push("expected ',' or ')'".to_string()),
                    None => return ParseStatus::Eof,
                }
            }
            self.builder.finish_node();
        } else {
            self.errors.push("expected '('".to_string())
        }
        ParseStatus::Ok
    }

    fn field(&mut self) -> ParseStatus {
        self.skip_ws();
        if let Some(TYPE_ID) = self.current() {
            self.builder.start_node(FIELD.into());
            self.bump();
            self.start_or_qmark();
            self.id();
            self.builder.finish_node();
            ParseStatus::Ok
        } else {
            ParseStatus::Eof
        }
    }

    fn start_or_qmark(&mut self) -> ParseStatus {
        let t = match self.current() {
            None => return ParseStatus::Eof,
            Some(t) => t,
        };
        match t {
            STAR | Q_MARK => {
                self.bump();
            }
            _ => (),
        }
        ParseStatus::Ok
    }

    fn id(&mut self) -> ParseStatus {
        self.skip_ws();
        let t = match self.current() {
            None => return ParseStatus::Eof,
            Some(t) => t,
        };
        match t {
            TYPE_ID | CONSTR_ID => {
                self.bump_replace(ID);
            }
            _ => (),
        }
        ParseStatus::Ok
    }

    fn skip_ws(&mut self) {
        while self.current() == Some(WHITESPACE) {
            self.bump()
        }
    }

    fn bump(&mut self) {
        let (kind, text) = self.tokens.pop().unwrap();
        self.builder.token(kind, text);
    }

    fn bump_replace(&mut self, new_kind: SyntaxKind) {
        let (_, text) = self.tokens.pop().unwrap();
        self.builder.token(new_kind, text);
    }

    fn current(&self) -> Option<SyntaxKind> {
        self.tokens.last().map(|(kind, _)| *kind)
    }
}

fn lex(text: &str) -> Vec<(SyntaxKind, SmolStr)> {
    fn tok(t: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(t.0)
    }

    fn kind(t: m_lexer::TokenKind) -> SyntaxKind {
        match t.0 {
            0 => L_PAREN,
            1 => R_PAREN,
            2 => PIPE,
            3 => Q_MARK,
            4 => STAR,
            5 => COMMA,
            6 => TYPE_ID,
            7 => CONSTR_ID,
            8 => WHITESPACE,
            9 => EQ,
            10 => ERROR,
            _ => unreachable!(),
        }
    }

    let lexer = m_lexer::LexerBuilder::new()
        .error_token(tok(ERROR))
        .tokens(&[
            (tok(L_PAREN), r"\("),
            (tok(R_PAREN), r"\)"),
            (tok(PIPE), r"\|"),
            (tok(Q_MARK), r"\?"),
            (tok(STAR), r"\*"),
            (tok(COMMA), r","),
            (tok(EQ), r"="),
            (tok(TYPE_ID), r"([a-z][[[:alpha:]]_[0-9]]*)"),
            (tok(CONSTR_ID), r"([A-Z][[[:alpha:]]_[0-9]]*)"),
            (tok(WHITESPACE), r"\s+"),
        ])
        .build();

    lexer
        .tokenize(text)
        .into_iter()
        .map(|t| (t.len, kind(t.kind)))
        .scan(0usize, |start_offset, (len, kind)| {
            let s: SmolStr = text[*start_offset..*start_offset + len].into();
            *start_offset += len;
            Some((kind, s))
        })
        .collect()
}

fn parse(text: &str) -> TreeArc<Root> {
    let mut tokens = lex(text);
    tokens.reverse();
    Parser { tokens, builder: GreenNodeBuilder::new(), errors: Vec::new() }.parse()
}

pub fn generate(asdl: &str, template: &str) -> String {
    let root: &Root = &parse(&asdl);
    let model = Asdl::from(root);
    let mut tera = Tera::default();
    tera.register_filter("camel", |arg, _| Ok(arg.as_str().unwrap().to_camel_case().into()));
    tera.register_filter("snake", |arg, _| Ok(arg.as_str().unwrap().to_snake_case().into()));
    tera.add_raw_template("_src", &template)
        .map_err(|e| format_err!("template parsing error: {:?}", e))
        .unwrap();
    tera.render("_src", &model)
        .map_err(|e| format_err!("template rendering error: {:?}", e))
        .unwrap()
}

mod model {

    use std::convert::From;
    use rustc_hash::FxHashMap;
    use std::fmt::Write;

    use serde::{
        Serialize,
};

    use crate:: {
        Root,
        TypeKind::*
};

    #[derive(Serialize, Debug)]
    pub(crate) struct Asdl {
        prod_types: Vec<ProdType>,
        sum_types: Vec<SumType>,
    }

    #[derive(Serialize, Debug)]
    pub(crate) struct ProdType {
        id: String,
        fields: Vec<Field>,
    }

    #[derive(Serialize, Debug)]
    pub(crate) struct SumType {
        id: String,
        constructors: Vec<Constructor>,
    }

    #[derive(Serialize, Debug)]
    pub(crate) struct Constructor {
        id: String,
        fields: Vec<Field>,
    }

    #[derive(Serialize, Debug)]
    pub(crate) struct Field {
        id: String,
        type_id: String,
        is_single: bool,
        is_option: bool,
        is_sequence: bool,
    }

    impl Field {
        fn single(id: String, type_id: String) -> Field {
            Field{ id, type_id, is_single: true, is_option: false, is_sequence: false}
        }

        fn option(id: String, type_id: String) -> Field {
            Field{ id, type_id, is_single: false, is_option: true, is_sequence: false}
        }

        fn sequence(id: String, type_id: String) -> Field {
            Field{ id, type_id, is_single: false, is_option: false, is_sequence: true}
        }
    }

    impl From<&Root> for Asdl {
        fn from(root: &Root) -> Self {
            let mut res = Asdl { prod_types: vec![], sum_types: vec![] };
            for d in root.defs() {
                let type_id = d.type_id().text().to_string();
                match d.ty().kind() {
                    Sum(t) => {
                        res.sum_types.push(sum_type(type_id, t));
                    }
                    Prod(t) => {
                        res.prod_types.push(prod_type(type_id, t));
                    }
                }
            }

            res
        }
    }

    fn sum_type(id: String, node: &crate::SumType) -> SumType {
        let constructors = node.constructors().map(constructor).collect();
        SumType { id, constructors }
    }

    fn constructor(node: &crate::Constr) -> Constructor {
        let id = node.id().text().to_string();
        let fields = match node.fields() {
            Option::Some(fs) => fields(fs),
            Option::None => vec![],
        };
        Constructor { id, fields }
    }

    fn fields(fields: &crate::Fields) -> Vec<Field> {
        let mut names = FieldNames::default();
        fields.fields().map(|f| field(f, &mut names)).collect()
    }

    fn field(node: &crate::Field, names: &mut FieldNames) -> Field {
        let type_id = node.type_id().text().to_string();
        let id = names.get_or_generate(node);
        if node.star().is_some() {
            Field::sequence(id, type_id)
        } else if node.q_mark().is_some() {
            Field::option(id, type_id)
        } else {
            Field::option(id, type_id)
        }
    }

    fn prod_type(type_id: String, node: &crate::ProdType) -> ProdType {
        ProdType { id: type_id, fields: fields(node.fields()) }
    }

    #[derive(Default)]
    struct FieldNames {
        names_indexes: FxHashMap<String, u32>,
    }

    impl FieldNames {
        fn get_or_generate(&mut self, f: &crate::Field) -> String {
            match f.id() {
                Option::Some(id) => id.text().to_string(),
                Option::None => {
                    let type_id = f.type_id().text();
                    let index = self.names_indexes.entry(type_id.to_string()).or_insert(0);
                    *index += 1;
                    let mut output = String::new();
                    write!(&mut output, "{}{}", type_id, index).unwrap();
                    output
                }
            }
        }
    }
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
            noFileds = One | Two| Tree
            prodType = (noFileds? f, stm s1)
            ";
        let root: &Root = &parse(asdl);
        let model = Asdl::from(root);
        assert_snapshot_matches!("simple_successful_test_syntax", root.debug_dump());
        assert_debug_snapshot_matches!("simple_successful_test_model", model)
    }

    #[test]
    fn generate_parser_structs() {
        let asdl = fs::read_to_string("src/parser/parser.asdl").unwrap();
        let template = fs::read_to_string("src/parser/generated.rs.tera").unwrap();
        let res = crate::generate(&asdl, &template);
        println!("{}", res);
    }

}
