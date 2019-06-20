mod generated;

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

pub(crate) use self::generated::*;

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
                                          //

const SINGLE: SyntaxKind = SyntaxKind(10);
const OPT: SyntaxKind = Q_MARK;
const SEQUENCE: SyntaxKind = STAR;

//composite syntax kinds

const ROOT: SyntaxKind = SyntaxKind(21);
const SUM_TYPE: SyntaxKind = SyntaxKind(23);
const PROD_TYPE: SyntaxKind = SyntaxKind(24);
const CONSTR: SyntaxKind = SyntaxKind(25);
const FIELD: SyntaxKind = SyntaxKind(27);
const ID: SyntaxKind = SyntaxKind(28);

impl TypeId {
    pub fn text(&self) -> &SmolStr {
        let ident = self.syntax().first_token().unwrap();
        ident.text()
    }
}

impl ConstrId {
    pub fn text(&self) -> &SmolStr {
        let ident = self.syntax().first_token().unwrap();
        ident.text()
    }
}

impl Id {
    pub fn text(&self) -> &SmolStr {
        let ident = self.syntax().first_token().unwrap();
        ident.text()
    }
}

impl Root {
    pub(crate) fn debug_dump(&self) -> String {
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
                            let off = token.range().end();
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

    fn preorder_with_tokens(&self) -> impl Iterator<Item = WalkEvent<SyntaxElement>> {
        self.syntax().preorder_with_tokens().map(|event| match event {
            WalkEvent::Enter(n) => WalkEvent::Enter(n.into()),
            WalkEvent::Leave(n) => WalkEvent::Leave(n.into()),
        })
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

    reserve: Vec<(SyntaxKind, SmolStr)>,
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
                self.reserve();
                self.skip_ws();
                match self.current() {
                    Some(EQ) => self.reserve(),
                    None => return ParseStatus::Eof,
                    Some(_) => self.errors.push("expected `=`".to_string()),
                }
                self.type_def();
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
                self.builder.start_node(SUM_TYPE);
                self.bump_reserved();
                self.constructors();
            }
            Some(L_PAREN) => {
                self.builder.start_node(PROD_TYPE);
                self.bump();
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
                self.builder.start_node(CONSTR);
                self.bump();
                self.skip_ws();
                if let Some(L_PAREN) = self.current() {
                    self.bump();
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
        self.field();
        loop {
            self.skip_ws();
            match self.current() {
                Some(COMMA) => {
                    self.skip_ws();
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
        ParseStatus::Ok
    }

    fn field(&mut self) -> ParseStatus {
        if let Some(TYPE_ID) = self.current() {
            self.builder.start_node(FIELD);
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
            self.reserve()
        }
    }

    fn reserve(&mut self) {
        self.reserve.push(self.tokens.pop().unwrap());
    }

    fn bump_reserved(&mut self) {
        self.reserve.reverse();
        loop {
            match self.reserve.pop() {
                Some((kind, text)) => self.builder.token(kind, text),
                None => break
            }
        }
    }

    fn bump(&mut self) {
        self.bump_reserved();
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

pub(crate) fn parse(text: &str) -> TreeArc<Root> {
    let mut tokens = lex(text);
    tokens.reverse();
    Parser { tokens, builder: GreenNodeBuilder::new(), errors: Vec::new(), reserve: Vec::new() }
        .parse()
}
