use std::convert::From;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Write;
use heck::MixedCase;

use serde:: Serialize;

use crate::parser;

#[derive(Serialize, Debug)]
pub(crate) struct Asdl {
    sum_types: Vec<SumType>,
    prod_types: Vec<ProdType>,
}

#[derive(Serialize, Debug)]
pub(crate) struct SumType {
    id: String,
    constructors: Vec<Constructor>,
}

#[derive(Serialize, Debug)]
pub(crate) struct Constructor {
    id: String,
    prod_type: ProdType,
}

#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct ProdType {
    id: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Field {
    id: String,
    type_id: String,
    is_single: bool,
    is_option: bool,
    is_sequence: bool,
}

impl Field {
    fn single(id: String, type_id: String) -> Field {
        Field { id, type_id, is_single: true, is_option: false, is_sequence: false }
    }

    fn option(id: String, type_id: String) -> Field {
        Field { id, type_id, is_single: false, is_option: true, is_sequence: false }
    }

    fn sequence(id: String, type_id: String) -> Field {
        Field { id, type_id, is_single: false, is_option: false, is_sequence: true }
    }
}

impl From<&parser::Root> for Asdl {
    fn from(root: &parser::Root) -> Self {
        let mut res = Asdl { prod_types: vec![], sum_types: vec![] };
        for d in root.types() {
            match d.kind() {
                parser::TypeKind::SumType(t) => {
                    res.sum_types.push(sum_type(t));
                }
                parser::TypeKind::ProdType(t) => {
                    res.prod_types.push(prod_type(t));
                }
            }
        }
        let syntetic_prod_types: FxHashSet<ProdType> = res
            .sum_types
            .iter()
            .flat_map(|t| t.constructors.iter().map(|c| c.prod_type.clone()))
            .collect();
        res.prod_types.extend(syntetic_prod_types);
        res
    }
}

fn sum_type(node: &parser::SumType) -> SumType {
    let id = node.type_id().text().to_string();
    let constructors = node.constructors().map(constructor).collect();
    SumType { id, constructors }
}

fn constructor(node: &parser::Constr) -> Constructor {
    let id = node.id().text().to_string();
    let mut names = FieldNames::default();
    let fields: Vec<Field> = node.fields().map(|f| field(f, &mut names)).collect();
    let pt = ProdType { id: id.to_mixed_case(), fields };
    Constructor { id, prod_type: pt }
}

fn field(node: &parser::Field, names: &mut FieldNames) -> Field {
    match node.kind() {
        parser::FieldKind::Single(f) => Field::single(
            names.get_or_generate(f.id(), f.type_id()),
            f.type_id().text().to_string(),
        ),
        parser::FieldKind::Opt(f) => Field::option(
            names.get_or_generate(f.id(), f.type_id()),
            f.type_id().text().to_string(),
        ),
        parser::FieldKind::Sequence(f) => Field::sequence(
            names.get_or_generate(f.id(), f.type_id()),
            f.type_id().text().to_string(),
        ),
    }
}

fn prod_type(node: &parser::ProdType) -> ProdType {
    let id = node.type_id().text().to_string();
    let mut names = FieldNames::default();
    let fields: Vec<Field> = node.fields().map(|f| field(f, &mut names)).collect();
    ProdType { id, fields }
}

#[derive(Default)]
struct FieldNames {
    names_indexes: FxHashMap<String, u32>,
}

impl FieldNames {
    fn get_or_generate(&mut self, id: Option<&parser::Id>, type_id: &parser::TypeId) -> String {
        match id {
            Option::Some(id) => id.text().to_string(),
            Option::None => {
                let type_id = type_id.text();
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
