use std::process::exit;

use gtk::glib;
use gtk::prelude::*;
use gtk::SearchEntry;

/// This module contains the key controller responsible for moving the cursor Up, Down and exiting
/// on Escape.

pub fn add_controller(searchentry: SearchEntry, list_box: gtk::ListBox) {
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        let keyname = match keyval.name() {
            Some(name) => {
                let name = name.to_string();
                name
            }
            None => return glib::Propagation::Proceed,
        };

        match keyname.as_str() {
            "Escape" => {
                exit(0);
            }
            "Up" => {
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

                        return glib::Propagation::Stop;
                    }
                };

                list_box.select_row(Some(
                    &selected_row
                        .prev_sibling()
                        .unwrap_or_else(|| list_box.last_child().unwrap())
                        .downcast::<gtk::ListBoxRow>()
                        .unwrap(),
                ));

                return glib::Propagation::Stop;
            }
            "Down" => {
                let selected_row = match list_box.selected_row() {
                    Some(row) => row,
                    None => {
                        list_box.select_row(Some(
                            &list_box
                                .first_child()
                                .unwrap_or_else(|| list_box.first_child().unwrap())
                                .downcast::<gtk::ListBoxRow>()
                                .unwrap(),
                        ));

                        return glib::Propagation::Stop;
                    }
                };

                list_box.select_row(Some(
                    &selected_row
                        .next_sibling()
                        .unwrap()
                        .downcast::<gtk::ListBoxRow>()
                        .unwrap(),
                ));

                return glib::Propagation::Stop;
            }
            _ => {}
        };

        glib::Propagation::Stop
    });
    searchentry.add_controller(key_controller);
}
