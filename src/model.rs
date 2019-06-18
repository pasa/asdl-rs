use std::convert::From;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Write;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase, MixedCase};

use serde::{
        Serialize,
};

use crate:: {
        Root,
        TypeKind::*
};

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
        let syntetic_prod_types: FxHashSet<ProdType> = res
            .sum_types
            .iter()
            .flat_map(|t| t.constructors.iter().map(|c| c.prod_type.clone()))
            .collect();
        res.prod_types.extend(syntetic_prod_types);
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
    let pt = ProdType { id: id.to_mixed_case(), fields };
    Constructor { id, prod_type: pt }
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
        Field::single(id, type_id)
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
