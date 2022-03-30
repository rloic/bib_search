use crate::{BibTexEntry};

pub trait Filter<T> {
    fn accept(&self, element: &T) -> bool;
}

pub struct FieldContains {
    field_name: String,
    sequence: String,
}

impl FieldContains {
    pub fn new(field_name: &str, sequence: &str) -> FieldContains {
        FieldContains { field_name: field_name.to_lowercase(), sequence: sequence.to_lowercase() }
    }
}

impl Filter<BibTexEntry> for FieldContains {
    fn accept(&self, element: &BibTexEntry) -> bool {
        element.fields.get(self.field_name.as_str())
            .map(|it| simplify_latex(&it.to_string()).contains(&self.sequence))
            .unwrap_or(false)
    }
}

pub struct EntryTypeContains {
    sequence: String,
}

impl EntryTypeContains {
    pub fn new(sequence: &str) -> EntryTypeContains {
        EntryTypeContains { sequence: sequence.to_lowercase() }
    }
}

impl Filter<BibTexEntry> for EntryTypeContains {
    fn accept(&self, element: &BibTexEntry) -> bool {
        simplify_latex(&element.entry_type).contains(&self.sequence)
    }
}

pub struct CiteKeyContains {
    sequence: String,
}

pub struct AnyFieldsContains {
    sequence: String,
}

impl AnyFieldsContains {
    pub fn new(sequence: &str) -> AnyFieldsContains {
        AnyFieldsContains { sequence: sequence.to_lowercase() }
    }
}

impl Filter<BibTexEntry> for AnyFieldsContains {
    fn accept(&self, element: &BibTexEntry) -> bool {
        simplify_latex(&element.entry_type).contains(&self.sequence) ||
            simplify_latex(&element.cite_key).contains(&self.sequence) ||
            element.fields.iter().any(|(_, v)| simplify_latex(&v.to_string()).contains(&self.sequence))
    }
}

impl CiteKeyContains {
    pub fn new(sequence: &str) -> CiteKeyContains {
        CiteKeyContains { sequence: sequence.to_lowercase() }
    }
}

impl Filter<BibTexEntry> for CiteKeyContains {
    fn accept(&self, element: &BibTexEntry) -> bool {
        simplify_latex(&element.cite_key).contains(&self.sequence)
    }
}

pub struct NegateFilter<T> {
    filter: Box<dyn Filter<T>>,
}

impl<T> NegateFilter<T> {
    pub fn new(filter: Box<dyn Filter<T>>) -> NegateFilter<T> {
        NegateFilter { filter }
    }
}

impl<T> Filter<T> for NegateFilter<T> {
    fn accept(&self, element: &T) -> bool {
        !self.filter.accept(element)
    }
}

pub struct AndFilters<T> {
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> AndFilters<T> {
    pub fn new(filters: Vec<Box<dyn Filter<T>>>) -> AndFilters<T> {
        AndFilters { filters }
    }
}

impl<T> Filter<T> for AndFilters<T> {
    fn accept(&self, element: &T) -> bool {
        for filter in &self.filters {
            if !filter.accept(element) { return false; }
        }
        true
    }
}

fn simplify_latex(latex: &String) -> String {
    latex.replace('{', "")
        .replace('}', "")
        .replace('"', "")
        .replace('`', "")
        .replace('\'', "")
        .replace('\\', "")
        .replace('%', "")
        .to_lowercase()
}