pub mod markdown_tabular;
pub mod debug;
pub mod bibtex;

pub trait Presenter<T> {
    fn present(&self, data: &T);
}