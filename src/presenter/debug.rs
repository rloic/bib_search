use crate::BibTexEntry;

pub struct Presenter {}

impl crate::presenter::Presenter<Vec<&&BibTexEntry>> for Presenter {
    fn present(&self, data: &Vec<&&BibTexEntry>) {
        println!("{:#?}", data);
    }
}