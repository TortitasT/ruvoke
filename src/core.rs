use freedesktop_entry_parser::{parse_entry, Entry};

pub fn get_applications() -> Vec<Entry> {
    let mut entries = Vec::new();

    let paths = match std::fs::read_dir("/usr/share/applications") {
        Ok(paths) => paths,
        Err(_) => {
            println!("Error reading /usr/share/applications");
            return entries;
        }
    };

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        let entry = parse_entry(path).unwrap();
        entries.push(entry);
    }

    entries
}
