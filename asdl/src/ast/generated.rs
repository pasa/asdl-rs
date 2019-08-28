//This file is auotgenerated by using `cargo gen-syntax`

#![cfg_attr(rustfmt, rustfmt_skip)]

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Type<'a> {
    SumType(SumType<'a>),
    ProdType(ProdType<'a>),
}
impl<'a> From<SumType<'a>> for Type<'a> {
    fn from(n: SumType) -> Type {
        Type::SumType(n)
    }
}
impl<'a> From<ProdType<'a>> for Type<'a> {
    fn from(n: ProdType) -> Type {
        Type::ProdType(n)
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct SumType<'a> {
    pub type_id: TypeId<'a>,
    pub constructors: Vec<Constr<'a>>,
    pub attrs: Option<Attrs<'a>>,
    pub comments: Vec<&'a str>,
}
impl<'a> SumType<'a> {

    pub(crate) fn new(type_id: TypeId<'a>, constructors: Vec<Constr<'a>>, attrs: Option<Attrs<'a>>, comments: Vec<&'a str>) -> Self {
        SumType{ type_id, constructors, attrs, comments }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ProdType<'a> {
    pub type_id: TypeId<'a>,
    pub fields: Vec<Field<'a>>,
    pub comments: Vec<&'a str>,
}
impl<'a> ProdType<'a> {

    pub(crate) fn new(type_id: TypeId<'a>, fields: Vec<Field<'a>>, comments: Vec<&'a str>) -> Self {
        ProdType{ type_id, fields, comments }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Field<'a> {
    Required(Required<'a>),
    Optional(Optional<'a>),
    Repeated(Repeated<'a>),
}
impl<'a> From<Required<'a>> for Field<'a> {
    fn from(n: Required) -> Field {
        Field::Required(n)
    }
}
impl<'a> From<Optional<'a>> for Field<'a> {
    fn from(n: Optional) -> Field {
        Field::Optional(n)
    }
}
impl<'a> From<Repeated<'a>> for Field<'a> {
    fn from(n: Repeated) -> Field {
        Field::Repeated(n)
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Required<'a> {
    pub type_id: TypeId<'a>,
    pub id: Option<Id<'a>>,
}
impl<'a> Required<'a> {

    pub(crate) fn new(type_id: TypeId<'a>, id: Option<Id<'a>>) -> Self {
        Required{ type_id, id }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Optional<'a> {
    pub type_id: TypeId<'a>,
    pub id: Option<Id<'a>>,
}
impl<'a> Optional<'a> {

    pub(crate) fn new(type_id: TypeId<'a>, id: Option<Id<'a>>) -> Self {
        Optional{ type_id, id }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Repeated<'a> {
    pub type_id: TypeId<'a>,
    pub id: Option<Id<'a>>,
}
impl<'a> Repeated<'a> {

    pub(crate) fn new(type_id: TypeId<'a>, id: Option<Id<'a>>) -> Self {
        Repeated{ type_id, id }
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Root<'a> {
    pub types: Vec<Type<'a>>,
    pub comments: Vec<&'a str>,
}
impl<'a> Root<'a> {

    pub(crate) fn new(types: Vec<Type<'a>>, comments: Vec<&'a str>) -> Self {
        Root{ types, comments }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Constr<'a> {
    pub id: ConstrId<'a>,
    pub fields: Vec<Field<'a>>,
    pub comments: Vec<&'a str>,
}
impl<'a> Constr<'a> {

    pub(crate) fn new(id: ConstrId<'a>, fields: Vec<Field<'a>>, comments: Vec<&'a str>) -> Self {
        Constr{ id, fields, comments }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Attrs<'a> {
    pub fields: Vec<Field<'a>>,
}
impl<'a> Attrs<'a> {

    pub(crate) fn new(fields: Vec<Field<'a>>) -> Self {
        Attrs{ fields }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TypeId<'a>(pub(crate) &'a str);

impl<'a> ToString for TypeId<'a> {

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ConstrId<'a>(pub(crate) &'a str);

impl<'a> ToString for ConstrId<'a> {

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Id<'a>(pub(crate) &'a str);

impl<'a> ToString for Id<'a> {

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}