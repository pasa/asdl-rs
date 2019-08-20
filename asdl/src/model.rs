use std::fmt::Write;
use rustc_hash::FxHashMap;
use serde::Serialize;
use crate::parser;

#[derive(Serialize, Debug)]
pub struct Asdl {
    pub types: FxHashMap<String, Type>,
    pub prod_types: Vec<String>,
    pub sum_types: Vec<String>,
}

impl Asdl {
    pub(crate) fn new(root: &parser::Root) -> Self {
        let mut prod_types = Vec::new();
        let mut sum_types = Vec::new();
        for ty in root.types() {
            match ty {
                parser::Type::SumType(sty) => sum_types.push(sty.type_id().id().to_string()),
                parser::Type::ProdType(pty) => prod_types.push(pty.type_id().id().to_string()),
            }
        }
        let types = root.types().map(ty).map(|t| (t.id(), t)).collect();
        Asdl { types, prod_types, sum_types }
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
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

fn ty(ty: &parser::Type) -> Type {
    match ty {
        parser::Type::SumType(sty) => sum_type(sty).into(),
        parser::Type::ProdType(pty) => prod_type(pty).into(),
    }
}

#[derive(Serialize, Debug)]
pub struct SumType {
    id: String,
    constructors: Vec<Constructor>,
    attributes: Vec<Field>,
    is_prod_type: bool, //always false
}

impl SumType {
    fn new(id: String, constructors: Vec<Constructor>, attributes: Vec<Field>) -> Self {
        SumType { id, constructors, attributes, is_prod_type: false }
    }
}

fn sum_type(ty: &parser::SumType) -> SumType {
    let id = ty.type_id().id().to_string();
    let constructors = ty.constructors().map(constr).collect();
    let attributes = if let Some(attrs) = ty.attrs() {
        let mut names = FieldNames::default();
        attrs.fields().map(|f| field(f, &mut names)).collect()
    } else {
        vec![]
    };
    SumType::new(id, constructors, attributes)
}

#[derive(Serialize, Debug)]
pub struct Constructor {
    id: String,
    fields: Vec<Field>,
}

impl Constructor {
    fn new(id: String, fields: Vec<Field>) -> Self {
        Constructor { id, fields }
    }
}

fn constr(c: &parser::Constr) -> Constructor {
    let mut names = FieldNames::default();
    let fields: Vec<Field> = c.fields().map(|f| field(f, &mut names)).collect();
    Constructor::new(c.id().id().to_string(), fields)
}

#[derive(Serialize, Debug)]
pub struct ProdType {
    id: String,
    fields: Vec<Field>,
    is_prod_type: bool, //always true
}

impl ProdType {
    fn new(id: String, fields: Vec<Field>) -> Self {
        ProdType { id, fields, is_prod_type: true }
    }
}

fn prod_type(ty: &parser::ProdType) -> ProdType {
    let mut names = FieldNames::default();
    let fields: Vec<Field> = ty.fields().map(|f| field(f, &mut names)).collect();
    ProdType::new(ty.type_id().id().to_string(), fields)
}

#[derive(Serialize, Debug)]
pub struct Field {
    id: String,
    type_id: String,
    is_single: bool,
    is_option: bool,
    is_sequence: bool,
}

impl Field {
    fn single(id: String, type_id: String) -> Self {
        Field { id, type_id, is_single: true, is_option: false, is_sequence: false }
    }

    fn option(id: String, type_id: String) -> Self {
        Field { id, type_id, is_single: false, is_option: true, is_sequence: false }
    }

    fn sequence(id: String, type_id: String) -> Self {
        Field { id, type_id, is_single: false, is_option: false, is_sequence: true }
    }
}

fn field(f: &parser::Field, names: &mut FieldNames) -> Field {
    match f {
        parser::Field::Single(f) => {
            Field::single(names.get_or_generate(f.id(), f.type_id()), f.type_id().id().to_string())
        }
        parser::Field::Opt(f) => {
            Field::option(names.get_or_generate(f.id(), f.type_id()), f.type_id().id().to_string())
        }
        parser::Field::Sequence(f) => Field::sequence(
            names.get_or_generate(f.id(), f.type_id()),
            f.type_id().id().to_string(),
        ),
    }
}

#[derive(Default)]
struct FieldNames {
    names_indexes: FxHashMap<String, u32>,
}

impl FieldNames {
    fn get_or_generate(&mut self, id: &Option<parser::Id>, type_id: &parser::TypeId) -> String {
        match id {
            Option::Some(id) => id.id().to_string(),
            Option::None => {
                let type_id = type_id.id();
                let index = self.names_indexes.entry(type_id.to_string()).or_insert(0);
                let res = if *index == 0 {
                    type_id.to_string()
                } else {
                    let mut buf = String::new();
                    write!(&mut buf, "{}{}", type_id, index).unwrap();
                    buf
                };
                *index += 1;
                res
            }
        }
    }
}
