use std::error::Error;
use std::fmt;
use crate::{parser, ast};

pub type Result<T> = std::result::Result<T, AsdlError>;

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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Asdl {
    pub types: Vec<Type>,
    pub comments: Vec<String>,
}

impl Asdl {
    pub fn parse(asdl: &str) -> Result<Asdl> {
        ast(asdl).map(|a| Asdl::new(&a))
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<&Type> {
        self.types.iter().find(|t| t.id() == name)
    }
}

fn ast(asdl: &str) -> Result<ast::Root> {
    let (_, root) = parser::parse(asdl).unwrap();
    Ok(root)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    SumType(SumType),
    ProdType(ProdType),
}

impl Type {
    fn id(&self) -> String {
        match self {
            Type::SumType(sty) => sty.id.clone(),
            Type::ProdType(pty) => pty.id.clone(),
        }
    }
}

impl From<SumType> for Type {
    fn from(n: SumType) -> Type {
        Type::SumType(n)
    }
}
impl From<ProdType> for Type {
    fn from(n: ProdType) -> Type {
        Type::ProdType(n)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SumType {
    pub id: String,
    pub constructors: Vec<Constructor>,
    pub attributes: Vec<Field>,
    pub comments: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Constructor {
    pub id: String,
    pub fields: Vec<Field>,
    pub comments: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProdType {
    pub id: String,
    pub fields: Vec<Field>,
    pub comments: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub id: String,
    pub type_id: String,
    pub arity: Arity,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Arity {
    Optional,
    Required,
    Repeated,
}
