pub trait Filter<T> {
    fn accept(&self, element: &T) -> bool;
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

pub struct OrFilters<T> {
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> OrFilters<T> {
    pub fn new(filters: Vec<Box<dyn Filter<T>>>) -> OrFilters<T> {
        OrFilters { filters }
    }
}

impl<T> Filter<T> for OrFilters<T> {
    fn accept(&self, element: &T) -> bool {
        for filter in &self.filters {
            if filter.accept(element) { return true; }
        }
        false
    }
}