use std::fmt::Write;
use rustc_hash::FxHashMap;

use crate::ast;

#[derive(Default)]
pub(crate) struct FieldNames {
    names_indexes: FxHashMap<String, u32>,
}

impl FieldNames {
    pub(crate) fn get_or_generate(
        &mut self,
        id: &Option<ast::Id>,
        type_id: &ast::TypeId,
    ) -> String {
        match id {
            Option::Some(id) => id.to_string(),
            Option::None => {
                let type_id = type_id.to_string();
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
