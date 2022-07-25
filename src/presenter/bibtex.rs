use crate::BibTexEntry;

pub struct Presenter {}

impl crate::presenter::Presenter<Vec<&&BibTexEntry>> for Presenter {
    fn present(&self, data: &Vec<&&BibTexEntry>) {
        for entry in data {
            print!("@{}{{", entry.entry_type);
            if !entry.cite_key.is_empty() {
                print!("{},", entry.cite_key);
            }
            println!();
            for (field_name, field_content) in entry.fields.iter() {
                println!("  {} = {:?},", field_name, field_content);
            }
            println!("}}");
        }
    }
}