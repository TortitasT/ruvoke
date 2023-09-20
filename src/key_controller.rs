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
                        let first_child = match list_box.first_child() {
                            Some(child) => child,
                            None => return glib::Propagation::Stop,
                        };
                        let first_child = match first_child.downcast::<gtk::ListBoxRow>() {
                            Ok(row) => row,
                            Err(_) => return glib::Propagation::Stop,
                        };

                        list_box.select_row(Some(&first_child));

                        return glib::Propagation::Stop;
                    }
                };

                let prev_sibling = match selected_row.prev_sibling() {
                    Some(row) => row,
                    None => {
                        let last_child = match list_box.last_child() {
                            Some(child) => child,
                            None => return glib::Propagation::Stop,
                        };
                        let last_child = match last_child.downcast::<gtk::ListBoxRow>() {
                            Ok(row) => row,
                            Err(_) => return glib::Propagation::Stop,
                        };

                        list_box.select_row(Some(&last_child));

                        return glib::Propagation::Stop;
                    }
                };
                let prev_sibling = match prev_sibling.downcast::<gtk::ListBoxRow>() {
                    Ok(row) => row,
                    Err(_) => return glib::Propagation::Stop,
                };

                list_box.select_row(Some(&prev_sibling));

                return glib::Propagation::Stop;
            }
            "Down" => {
                let selected_row = match list_box.selected_row() {
                    Some(row) => row,
                    None => {
                        let first_child = match list_box.first_child() {
                            Some(child) => child,
                            None => return glib::Propagation::Stop,
                        };
                        let first_child = match first_child.downcast::<gtk::ListBoxRow>() {
                            Ok(row) => row,
                            Err(_) => return glib::Propagation::Stop,
                        };

                        list_box.select_row(Some(&first_child));

                        return glib::Propagation::Stop;
                    }
                };

                let next_sibling = match selected_row.next_sibling() {
                    Some(row) => row,
                    None => {
                        let first_child = match list_box.first_child() {
                            Some(child) => child,
                            None => return glib::Propagation::Stop,
                        };
                        let first_child = match first_child.downcast::<gtk::ListBoxRow>() {
                            Ok(row) => row,
                            Err(_) => return glib::Propagation::Stop,
                        };

                        list_box.select_row(Some(&first_child));

                        return glib::Propagation::Stop;
                    }
                };
                let next_sibling = match next_sibling.downcast::<gtk::ListBoxRow>() {
                    Ok(row) => row,
                    Err(_) => return glib::Propagation::Stop,
                };

                list_box.select_row(Some(&next_sibling));

                return glib::Propagation::Stop;
            }
            _ => {}
        };

        glib::Propagation::Stop
    });
    searchentry.add_controller(key_controller);
}
