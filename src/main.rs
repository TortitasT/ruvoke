use crate::core::get_applications;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use key_controller::add_controller;
use searchbar::build_searchbar;

mod core;
mod key_controller;
mod searchbar;

const APP_ID: &str = "eu.tortitas.runst";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn populate_list_box(list_box: &gtk::ListBox, text: Option<&str>) -> usize {
    let entries = get_applications();

    let mut count = 0;
    for entry in entries {
        let name = entry
            .section("Desktop Entry")
            .attr("Name")
            .unwrap_or("No name");

        if let Some(text) = text {
            if !name.to_lowercase().contains(&text.to_lowercase()) {
                continue;
            }
        }

        let vox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .margin_bottom(5)
            .spacing(20)
            .build();

        let inside_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let label = gtk::Label::builder()
            .label(name)
            .halign(gtk::Align::Start)
            .build();
        let description = gtk::Label::builder()
            .label(
                entry
                    .section("Desktop Entry")
                    .attr("Comment")
                    .unwrap_or("No description"),
            )
            .build();

        let icon =
            gtk::Image::from_icon_name(entry.section("Desktop Entry").attr("Icon").unwrap_or(""));
        icon.set_pixel_size(32);

        vox.append(&icon);
        inside_box.append(&label);
        inside_box.append(&description);
        vox.append(&inside_box);

        let row = gtk::ListBoxRow::builder().child(&vox).name(name).build();

        list_box.append(&row);

        count += 1;
    }

    count
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app);
    let window = window.default_width(800).default_height(500);
    let window = window.opacity(0.8);
    let window = window.resizable(false);
    let window = window.decorated(false);

    let vox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::Browse)
        .show_separators(true)
        .build();
    populate_list_box(&list_box, None);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&list_box)
        .build();

    let (searchbar, searchentry) = build_searchbar(&list_box);

    vox.append(&searchbar);
    vox.append(&scrolled_window);

    add_controller(searchentry, list_box, scrolled_window);

    let window = window.child(&vox);
    let window = window.build();

    window.present();
}
