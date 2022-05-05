use std::fmt::{Debug, Formatter};
use crate::parsers::content::Content::{Braced, Concatenated, Inlined, Quoted};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Content {
    Quoted(String),
    Braced(String),
    Inlined(String),
    Concatenated(Vec<Content>)
}

impl Debug for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Quoted(inner) => write!(f, "\"{}\"", inner)?,
            Braced(inner) => write!(f, "{{{}}}", inner)?,
            Inlined(inner) => write!(f, "{}", inner)?,
            Concatenated(contents) => {
                let mut i = 0;
                loop {
                    write!(f, "{:?}", &contents[i])?;
                    i += 1;
                    if i == contents.len() {
                        break;
                    }
                    write!(f, " # ")?;
                }
            }
        };
        Ok(())
    }
}

impl ToString for Content {
    fn to_string(&self) -> String {
        match self {
            Quoted(inner) => inner.clone(),
            Braced(inner) => inner.clone(),
            Inlined(inner) => inner.clone(),
            Concatenated(contents) => contents.iter().map(|it| it.to_string()).collect::<Vec<_>>().join(" ")
        }
    }
}