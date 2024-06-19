#![windows_subsystem = "windows"]
slint::include_modules!();
slint::slint!();
use native_dialog::FileDialog;
use pdf_extract::extract_text;
use rust_search::SearchBuilder;

static mut FILEPATH: String = String::new();

fn main() {
    let app: App = App::new().unwrap();
    let _weak = app.as_weak();
    app.on_fileexplorer(move || {
        let opendialog: Option<std::path::PathBuf> = FileDialog::new()
            .set_location("~/Downloads")
            .add_filter("PDF", &["pdf"])
            .show_open_single_dir()
            .unwrap();
        match opendialog {
            Some(_) => {
                let filepath = opendialog.unwrap();
                unsafe {
                    FILEPATH = filepath.into_os_string().into_string().unwrap();
                }
            }
            None {} => {}
        }
    });
    app.on_search({
        let handle: slint::Weak<App> = app.as_weak();
        move || {
            let weak: App = handle.unwrap();
            let mut fileresults: String = String::new();
            let binding = weak.get_texttosearch();
            let texttosearch: &str = binding.as_str();
            unsafe {
                let search: Vec<String> = SearchBuilder::default()
                    .location(FILEPATH.clone())
                    .ext("pdf")
                    .build()
                    .collect();
                for file in search {
                    let text = extract_text(&file).expect("Unable to read PDF");
                    if text.contains(texttosearch) {
                        fileresults = fileresults + "Found: " + &file + "\n\n";
                        weak.set_results((&fileresults).into());
                    }
                }
            };
        }
    });
    app.run().unwrap();
}

