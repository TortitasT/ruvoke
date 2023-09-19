use freedesktop_entry_parser::{parse_entry, Entry};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "eu.tortitas.runst";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_searchbar() -> (gtk::SearchBar, gtk::SearchEntry) {
    let searchbar = gtk::SearchBar::builder().build();
    let searchentry = gtk::SearchEntry::builder()
        .hexpand(true)
        .placeholder_text("Search")
        .build();

    searchbar.connect_entry(&searchentry);
    searchbar.set_search_mode(true);
    searchbar.set_child(Some(&searchentry));

    (searchbar, searchentry)
}

fn populate_list_box(list_box: &gtk::ListBox, text: Option<&str>) {
    let entries = get_applications();

    for entry in entries {
        let description = entry
            .section("Desktop Entry")
            .attr("Name")
            .unwrap_or("No name");

        if let Some(text) = text {
            if !description.to_lowercase().contains(&text.to_lowercase()) {
                continue;
            }
        }

        let button = gtk::Button::builder().label(description).build();
        button.connect_clicked(move |_| {
            let command = match entry.section("Desktop Entry").attr("Exec") {
                Some(command) => command,
                None => {
                    println!("No command");
                    return;
                }
            };

            println!("Running {}", command);
            std::process::Command::new(std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()))
                .arg("-c")
                .arg(command)
                .spawn()
                .unwrap();
        });

        list_box.append(&button);
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app);
    let window = window.default_width(800).default_height(500);
    let window = window.opacity(0.8);
    let window = window.resizable(false);

    let vox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();

    let list_box = gtk::ListBox::builder().build();
    populate_list_box(&list_box, None);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&list_box)
        .build();

    let (searchbar, searchentry) = build_searchbar();
    searchentry.connect_changed(move |entry| {
        let text = entry.text();

        list_box.remove_all();
        populate_list_box(&list_box, Some(&text));
    });

    vox.append(&searchbar);
    vox.append(&scrolled_window);

    let window = window.child(&vox);

    let window = window.build();
    window.present();
}

fn get_applications() -> Vec<Entry> {
    let mut entries = Vec::new();

    let paths = std::fs::read_dir("/usr/share/applications").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        let entry = parse_entry(path).unwrap();
        entries.push(entry);
    }

    entries
}
