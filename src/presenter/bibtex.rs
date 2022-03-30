use crate::BibTexEntry;

pub struct Presenter {}

impl crate::presenter::Presenter<Vec<&&BibTexEntry>> for Presenter {
    fn present(&self, data: &Vec<&&BibTexEntry>) {

        for entry in data {
            println!("@{}{{{},", entry.entry_type, entry.cite_key);
            for (field_name, field_content) in entry.fields.iter() {
                println!("  {} = {:?},", field_name, field_content);
            }
            println!("}}");
        }

    }
}