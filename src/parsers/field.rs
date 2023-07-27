use crate::parsers::content::Content;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Field(pub String, pub Content);