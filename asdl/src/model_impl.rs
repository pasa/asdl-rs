use crate::model::*;
use crate::ast;
use crate::util::FieldNames;

impl Asdl {
    pub fn new(root: &ast::Root) -> Self {
        Asdl { types: root.types.iter().map(ty).collect() }
    }
}

fn ty(ty: &ast::Type) -> Type {
    match ty {
        ast::Type::SumType(sty) => sum_type(sty).into(),
        ast::Type::ProdType(pty) => prod_type(pty).into(),
    }
}

impl SumType {
    fn new(id: String, constructors: Vec<Constructor>, attributes: Vec<Field>) -> Self {
        SumType { id, constructors, attributes }
    }
}

fn sum_type(ty: &ast::SumType) -> SumType {
    let id = ty.type_id.to_string();
    let constructors = ty.constructors.iter().map(constr).collect();
    let attributes = if let Some(attrs) = &ty.attrs {
        let mut names = FieldNames::default();
        attrs.fields.iter().map(|f| field(f, &mut names)).collect()
    } else {
        vec![]
    };
    SumType::new(id, constructors, attributes)
}

impl Constructor {
    fn new(id: String, fields: Vec<Field>) -> Self {
        Constructor { id, fields }
    }
}

fn constr(c: &ast::Constr) -> Constructor {
    let mut names = FieldNames::default();
    let fields: Vec<Field> = c.fields.iter().map(|f| field(f, &mut names)).collect();
    Constructor::new(c.id.to_string(), fields)
}

impl ProdType {
    fn new(id: String, fields: Vec<Field>) -> Self {
        ProdType { id, fields }
    }
}

fn prod_type(ty: &ast::ProdType) -> ProdType {
    let mut names = FieldNames::default();
    let fields: Vec<Field> = ty.fields.iter().map(|f| field(f, &mut names)).collect();
    ProdType::new(ty.type_id.to_string(), fields)
}

impl Field {
    fn required(id: String, type_id: String) -> Self {
        Field { id, type_id, arity: Arity::Required }
    }

    fn optional(id: String, type_id: String) -> Self {
        Field { id, type_id, arity: Arity::Optional }
    }

    fn repeated(id: String, type_id: String) -> Self {
        Field { id, type_id, arity: Arity::Repeated }
    }
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
