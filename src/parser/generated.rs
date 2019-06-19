use rowan:: {
    SmolStr,
    SyntaxKind,
    GreenNode,
    GreenNodeBuilder,
    SyntaxElement,
    SyntaxNode,
    SyntaxToken,
    TreeArc,
    TransparentNewType,
    WalkEvent,
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
        Type::cast(&n.syntax).unwrap()
    }
}

impl<'a> From<&'a ProdType> for &'a Type {
    fn from(n: &'a ProdType) -> &'a Type {
        Type::cast(&n.syntax).unwrap()
    }
}


impl Type {
    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            | SUM_TYPE
            | PROD_TYPE => Some(Type::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    pub fn kind(&self) -> TypeKind {
        match self.0.kind() {
            SUM_TYPE => TypeKind::SumType(SumType::cast(&self.syntax).unwrap()),
            PROD_TYPE => TypeKind::ProdType(ProdType::cast(&self.syntax).unwrap()),
            _ => unreachable!(),
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
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
        Field::cast(&n.syntax).unwrap()
    }
}

impl<'a> From<&'a Opt> for &'a Field {
    fn from(n: &'a Opt) -> &'a Field {
        Field::cast(&n.syntax).unwrap()
    }
}

impl<'a> From<&'a Sequence> for &'a Field {
    fn from(n: &'a Sequence) -> &'a Field {
        Field::cast(&n.syntax).unwrap()
    }
}


impl Field {
    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            | SINGLE
            | OPT
            | SEQUENCE => Some(Field::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    pub fn kind(&self) -> FieldKind {
        match self.0.kind() {
            SINGLE => FieldKind::Single(Single::cast(&self.syntax).unwrap()),
            OPT => FieldKind::Opt(Opt::cast(&self.syntax).unwrap()),
            SEQUENCE => FieldKind::Sequence(Sequence::cast(&self.syntax).unwrap()),
            _ => unreachable!(),
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Root(SyntaxNode);

impl Root {
    
    fn types(&self) -> impl Iterator<Item = &Type> {
        self.0.children().filter_map(Type::cast)
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            ROOT => Some(Root::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Constr(SyntaxNode);

impl Constr {
    
    fn id(&self) -> &ConstrId {
        self.0.children().find_map(ConstrId::cast).unwrap()
    }
    
    
    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            CONSTR => Some(Constr::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct TypeId(SyntaxNode);

impl TypeId {
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            TYPE_ID => Some(TypeId::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct ConstrId(SyntaxNode);

impl ConstrId {
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            CONSTR_ID => Some(ConstrId::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Id(SyntaxNode);

impl Id {
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            ID => Some(Id::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Sequence(SyntaxNode);

impl Sequence {
    
    fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    
    fn id(&self) -> &Id {
        self.0.children().find_map(Id::cast).unwrap()
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SEQUENCE => Some(Sequence::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Single(SyntaxNode);

impl Single {
    
    fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    
    fn id(&self) -> &Id {
        self.0.children().find_map(Id::cast).unwrap()
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SINGLE => Some(Single::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct ProdType(SyntaxNode);

impl ProdType {
    
    fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    
    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.0.children().filter_map(Field::cast)
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            PROD_TYPE => Some(ProdType::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct Opt(SyntaxNode);

impl Opt {
    
    fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    
    fn id(&self) -> &Id {
        self.0.children().find_map(Id::cast).unwrap()
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            OPT => Some(Opt::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub(crate) struct SumType(SyntaxNode);

impl SumType {
    
    fn type_id(&self) -> &TypeId {
        self.0.children().find_map(TypeId::cast).unwrap()
    }
    
    
    fn constructors(&self) -> impl Iterator<Item = &Constr> {
        self.0.children().filter_map(Constr::cast)
    }
    
    

    fn cast(syntax: &SyntaxNode) -> Option<&Self> {
        match syntax.kind() {
            SUM_TYPE => Some(SumType::from_repr(syntax.into_repr())),
            _ => None,
        }
    }

    fn to_owned(&self) -> TreeArc<Self> {
        TreeArc::cast(self.0.to_owned())
    }
}
