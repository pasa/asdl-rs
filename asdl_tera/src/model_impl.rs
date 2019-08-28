use asdl::ast;
use asdl::util::FieldNames;

use crate::model::*;

impl Asdl {
    pub(crate) fn new(root: &ast::Root) -> Self {
        let mut prod_types = Vec::new();
        let mut sum_types = Vec::new();
        for ty in root.types.iter() {
            match ty {
                ast::Type::SumType(sty) => sum_types.push(sty.type_id.to_string()),
                ast::Type::ProdType(pty) => prod_types.push(pty.type_id.to_string()),
            }
        }
        let types = root.types.iter().map(ty).map(|t| (t.id(), t)).collect();
        let comments = comments(&root.comments);
        Asdl { types, prod_types, sum_types, comments }
    }
}

impl Type {
    fn id(&self) -> String {
        match self {
            Type::SumType(sty) => sty.id.clone(),
            Type::ProdType(pty) => pty.id.clone(),
        }
    }
}

fn ty(ty: &ast::Type) -> Type {
    match ty {
        ast::Type::SumType(sty) => sum_type(sty).into(),
        ast::Type::ProdType(pty) => prod_type(pty).into(),
    }
}

impl SumType {
    fn new(
        id: String,
        constructors: Vec<Constructor>,
        attributes: Vec<Field>,
        comments: Vec<String>,
    ) -> Self {
        SumType { id, constructors, attributes, is_prod_type: false, comments }
    }
}

fn sum_type(ty: &ast::SumType) -> SumType {
    let id = ty.type_id.to_string();
    let constructors = ty.constructors.iter().map(constr).collect();
    let attributes = ty.attrs.as_ref().map(|a| fields(&a.fields)).unwrap_or_default();
    SumType::new(id, constructors, attributes, comments(&ty.comments))
}

impl Constructor {
    fn new(id: String, fields: Vec<Field>, comments: Vec<String>) -> Self {
        Constructor { id, fields, comments }
    }
}

fn constr(c: &ast::Constr) -> Constructor {
    Constructor::new(c.id.to_string(), fields(&c.fields), comments(&c.comments))
}

impl ProdType {
    fn new(id: String, fields: Vec<Field>, comments: Vec<String>) -> Self {
        ProdType { id, fields, is_prod_type: true, comments }
    }
}

fn prod_type(ty: &ast::ProdType) -> ProdType {
    ProdType::new(ty.type_id.to_string(), fields(&ty.fields), comments(&ty.comments))
}

impl Field {
    fn required(id: String, type_id: String) -> Self {
        Field { id, type_id, is_required: true, is_optional: false, is_repeated: false }
    }

    fn optional(id: String, type_id: String) -> Self {
        Field { id, type_id, is_required: false, is_optional: true, is_repeated: false }
    }

    fn repeated(id: String, type_id: String) -> Self {
        Field { id, type_id, is_required: false, is_optional: false, is_repeated: true }
    }
}

fn comments(comments: &Vec<&str>) -> Vec<String> {
    comments.iter().map(ToString::to_string).collect()
}

fn fields(fields: &Vec<ast::Field>) -> Vec<Field> {
    let mut names = FieldNames::default();
    fields.iter().map(|f| field(f, &mut names)).collect()
}

fn field(f: &ast::Field, names: &mut FieldNames) -> Field {
    match f {
        ast::Field::Required(f) => {
            Field::required(names.get_or_generate(&f.id, &f.type_id), f.type_id.to_string())
        }
        ast::Field::Optional(f) => {
            Field::optional(names.get_or_generate(&f.id, &f.type_id), f.type_id.to_string())
        }
        ast::Field::Repeated(f) => {
            Field::repeated(names.get_or_generate(&f.id, &f.type_id), f.type_id.to_string())
        }
    }
}
