use crate::model::*;
use asdl;

impl Asdl {
    pub(crate) fn new(model: asdl::Asdl) -> Self {
        let mut prod_types = Vec::new();
        let mut sum_types = Vec::new();
        for ty in model.types.iter() {
            match ty {
                asdl::Type::SumType(sty) => sum_types.push(sty.id.clone()),
                asdl::Type::ProdType(pty) => prod_types.push(pty.id.clone()),
            }
        }
        let types = model.types.into_iter().map(ty).map(|t| (t.id(), t)).collect();
        let comments = model.comments;
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

fn ty(ty: asdl::Type) -> Type {
    match ty {
        asdl::Type::SumType(sty) => sum_type(sty).into(),
        asdl::Type::ProdType(pty) => prod_type(pty).into(),
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

fn sum_type(ty: asdl::SumType) -> SumType {
    let id = ty.id;
    let constructors = ty.constructors.into_iter().map(constr).collect();
    let attributes = fields(ty.attributes);
    let comments = ty.comments;
    SumType::new(id, constructors, attributes, comments)
}

impl Constructor {
    fn new(id: String, fields: Vec<Field>, comments: Vec<String>) -> Self {
        Constructor { id, fields, comments }
    }
}

fn constr(c: asdl::Constructor) -> Constructor {
    Constructor::new(c.id, fields(c.fields), c.comments)
}

impl ProdType {
    fn new(id: String, fields: Vec<Field>, comments: Vec<String>) -> Self {
        ProdType { id, fields, is_prod_type: true, comments }
    }
}

fn prod_type(ty: asdl::ProdType) -> ProdType {
    ProdType::new(ty.id, fields(ty.fields), ty.comments)
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

fn fields(fields: Vec<asdl::Field>) -> Vec<Field> {
    fields.into_iter().map(field).collect()
}

fn field(f: asdl::Field) -> Field {
    match f.arity {
        asdl::Arity::Required => Field::required(f.id, f.type_id),
        asdl::Arity::Optional => Field::optional(f.id, f.type_id),
        asdl::Arity::Repeated => Field::repeated(f.id, f.type_id),
    }
}
