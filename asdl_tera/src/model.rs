use std::collections::HashMap;
use serde::Serialize;
use asdl;
use super::Result;

#[derive(Serialize, Debug)]
pub struct Asdl {
    pub types: HashMap<String, Type>,
    pub prod_types: Vec<String>,
    pub sum_types: Vec<String>,
    pub comments: Vec<String>,
}

impl Asdl {
    pub fn parse(asdl: &str) -> Result<Asdl> {
        Ok(Asdl::new(asdl::Asdl::parse(asdl)?))
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Type {
    SumType(SumType),
    ProdType(ProdType),
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

#[derive(Serialize, Debug)]
pub struct SumType {
    pub id: String,
    pub constructors: Vec<Constructor>,
    pub attributes: Vec<Field>,
    pub is_prod_type: bool, //always false
    pub comments: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Constructor {
    pub id: String,
    pub fields: Vec<Field>,
    pub comments: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ProdType {
    pub id: String,
    pub fields: Vec<Field>,
    pub is_prod_type: bool, //always true
    pub comments: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Field {
    pub id: String,
    pub type_id: String,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_repeated: bool,
}
