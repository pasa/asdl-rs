use rustc_hash::FxHashMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Asdl {
    pub types: FxHashMap<String, Type>,
    pub prod_types: Vec<String>,
    pub sum_types: Vec<String>,
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
}

#[derive(Serialize, Debug)]
pub struct Constructor {
    pub id: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Debug)]
pub struct ProdType {
    pub id: String,
    pub fields: Vec<Field>,
    pub is_prod_type: bool, //always true
}

#[derive(Serialize, Debug)]
pub struct Field {
    pub id: String,
    pub type_id: String,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_repeated: bool,
}
