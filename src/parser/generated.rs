use rowan:: {
    SyntaxNode,
    TreeArc,
    TransparentNewType,
};

use super::*;


#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Type(SyntaxNode);

unsafe impl TransparentNewType for Type {
    type Repr = rowan::SyntaxNode;
}

pub(crate) enum TypeKind<'a> {

    SumType(&'a SumType),
    ProdType(&'a ProdType),
}

impl<'a> From<&'a SumType> for &'a Type {
    fn from(n: &'a SumType) -> &'a Type {
        Type::cast(&n.0).unwrap()
    }
}

impl<'a> From<&'a ProdType> for &'a Type {
    fn from(n: &'a ProdType) -> &'a Type {
        Type::cast(&n.0).unwrap()
    }
}

impl Type {

    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            | SUM_TYPE
            | PROD_TYPE => Some(Type::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn kind(&self) -> TypeKind {
        match self.0.kind() {
            SUM_TYPE => TypeKind::SumType(SumType::cast(&self.0).unwrap()),
            PROD_TYPE => TypeKind::ProdType(ProdType::cast(&self.0).unwrap()),
            _ => unreachable!(),
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Field(SyntaxNode);

unsafe impl TransparentNewType for Field {
    type Repr = rowan::SyntaxNode;
}

pub(crate) enum FieldKind<'a> {

    Single(&'a Single),
    Opt(&'a Opt),
    Sequence(&'a Sequence),
}

impl<'a> From<&'a Single> for &'a Field {
    fn from(n: &'a Single) -> &'a Field {
        Field::cast(&n.0).unwrap()
    }
}

impl<'a> From<&'a Opt> for &'a Field {
    fn from(n: &'a Opt) -> &'a Field {
        Field::cast(&n.0).unwrap()
    }
}

impl<'a> From<&'a Sequence> for &'a Field {
    fn from(n: &'a Sequence) -> &'a Field {
        Field::cast(&n.0).unwrap()
    }
}

impl Field {

    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            | SINGLE
            | OPT
            | SEQUENCE => Some(Field::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn kind(&self) -> FieldKind {
        match self.0.kind() {
            SINGLE => FieldKind::Single(Single::cast(&self.0).unwrap()),
            OPT => FieldKind::Opt(Opt::cast(&self.0).unwrap()),
            SEQUENCE => FieldKind::Sequence(Sequence::cast(&self.0).unwrap()),
            _ => unreachable!(),
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}




#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Root(SyntaxNode);

unsafe impl TransparentNewType for Root {
    type Repr = rowan::SyntaxNode;
}

impl Root {
    
    #[allow(unused)]
    pub(crate) fn types(&self) -> impl Iterator<Item = &Type> {
        self.0.children().filter_map(Type::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            ROOT => Some(Root::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Constr(SyntaxNode);

unsafe impl TransparentNewType for Constr {
    type Repr = rowan::SyntaxNode;
}

impl Constr {
    
    #[allow(unused)]
    pub(crate) fn id(&self) -> &ConstrId {
        self.0.children().find_map(ConstrId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            CONSTR => Some(Constr::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Attrs(SyntaxNode);

unsafe impl TransparentNewType for Attrs {
    type Repr = rowan::SyntaxNode;
}

impl Attrs {
    
    #[allow(unused)]
    pub(crate) fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            ATTRS => Some(Attrs::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct TypeId(SyntaxNode);

unsafe impl TransparentNewType for TypeId {
    type Repr = rowan::SyntaxNode;
}

impl TypeId {
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            TYPE_ID => Some(TypeId::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct ConstrId(SyntaxNode);

unsafe impl TransparentNewType for ConstrId {
    type Repr = rowan::SyntaxNode;
}

impl ConstrId {
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            CONSTR_ID => Some(ConstrId::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Id(SyntaxNode);

unsafe impl TransparentNewType for Id {
    type Repr = rowan::SyntaxNode;
}

impl Id {
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            ID => Some(Id::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct SumType(SyntaxNode);

unsafe impl TransparentNewType for SumType {
    type Repr = rowan::SyntaxNode;
}

impl SumType {
    
    #[allow(unused)]
    pub(crate) fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn constructors(&self) -> impl Iterator<Item = &Constr> {
        self.0.children().filter_map(Constr::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn attrs(&self) -> Option<&Attrs> {
        self.0.children().find_map(Attrs::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SUM_TYPE => Some(SumType::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct ProdType(SyntaxNode);

unsafe impl TransparentNewType for ProdType {
    type Repr = rowan::SyntaxNode;
}

impl ProdType {
    
    #[allow(unused)]
    pub(crate) fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            PROD_TYPE => Some(ProdType::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Single(SyntaxNode);

unsafe impl TransparentNewType for Single {
    type Repr = rowan::SyntaxNode;
}

impl Single {
    
    #[allow(unused)]
    pub(crate) fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn id(&self) -> Option<&Id> {
        self.0.children().find_map(Id::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SINGLE => Some(Single::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Opt(SyntaxNode);

unsafe impl TransparentNewType for Opt {
    type Repr = rowan::SyntaxNode;
}

impl Opt {
    
    #[allow(unused)]
    pub(crate) fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn id(&self) -> Option<&Id> {
        self.0.children().find_map(Id::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            OPT => Some(Opt::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Sequence(SyntaxNode);

unsafe impl TransparentNewType for Sequence {
    type Repr = rowan::SyntaxNode;
}

impl Sequence {
    
    #[allow(unused)]
    pub(crate) fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    #[allow(unused)]
    pub(crate) fn id(&self) -> Option<&Id> {
        self.0.children().find_map(Id::cast)
    }
    
    #[allow(unused)]
    pub(crate) fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SEQUENCE => Some(Sequence::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }

    #[allow(unused)]
    pub(crate) fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[allow(unused)]
pub(crate) fn kind_name(kind: SyntaxKind) -> &'static str {
    match kind {
        SUM_TYPE => "SumType",
        PROD_TYPE => "ProdType",
        SINGLE => "Single",
        OPT => "Opt",
        SEQUENCE => "Sequence",
        ROOT => "Root",
        CONSTR => "Constr",
        ATTRS => "Attrs",
        TYPE_ID => "TypeId",
        CONSTR_ID => "ConstrId",
        ID => "Id",
        SUM_TYPE => "SumType",
        PROD_TYPE => "ProdType",
        SINGLE => "Single",
        OPT => "Opt",
        SEQUENCE => "Sequence",
        _ => "Undefined"
    }
}