use std::process::exit;

use gtk::glib::clone;
use gtk::prelude::*;

use crate::{get_applications, populate_list_box};

pub fn build_searchbar(list_box: &gtk::ListBox) -> (gtk::SearchBar, gtk::SearchEntry) {
    let searchbar = gtk::SearchBar::builder().build();
    let searchentry = gtk::SearchEntry::builder()
        .hexpand(true)
        .placeholder_text("Search for apps and commands")
        .build();

    searchbar.connect_entry(&searchentry);
    searchbar.set_search_mode(true);
    searchbar.set_child(Some(&searchentry));

    searchentry.connect_search_changed(clone!(@strong list_box => move |searchentry| {
        let text = searchentry.text();

        list_box.remove_all();
        let count = populate_list_box(&list_box, Some(&text));

        if count == 0 {
            let label = gtk::Label::builder()
                .label("No results")
                .halign(gtk::Align::Center)
                .build();
            list_box.append(&label);

            return;
        }

        list_box.select_row(Some(
            &list_box
                .first_child()
                .unwrap()
                .downcast::<gtk::ListBoxRow>()
                .unwrap(),
        ));
    }));

    searchentry.connect_activate(clone!(@strong list_box => move |_| {
        let selected_row = match list_box.selected_row() {
            Some(row) => row,
            None => {
                list_box.select_row(Some(
                    &list_box
                        .first_child()
                        .unwrap()
                        .downcast::<gtk::ListBoxRow>()
                        .unwrap(),
                ));

                return;
            }
        };

        let text = selected_row.widget_name();

        let entries = get_applications();

        for entry in entries {
            let description = entry
                .section("Desktop Entry")
                .attr("Name")
                .unwrap_or("No name");

            if description != text {
                continue;
            }

            let command = match entry.section("Desktop Entry").attr("Exec") {
                Some(command) => command,
                None => {
                    println!("No command");
                    return;
                }
            };

            let command = command.split(' ').collect::<Vec<&str>>();

            println!("Running {}", command[0]);
            std::process::Command::new(std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()))
                .arg("-c")
                .arg(command[0])
                // .args(&command[1..])
                .spawn()
                .unwrap();

            exit(0);
        }
    }));

    (searchbar, searchentry)
}
