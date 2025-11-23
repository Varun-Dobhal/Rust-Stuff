use chrono::{Date, DateTime, Utc};
use clap::{Parser};
use tabled::{Table, Tabled, settings::{Color, Style, object::{Columns, Rows}}};
use std::{
    fmt::format, fs::{self, File}, path::{self, Path, PathBuf}, vec
};
use owo_colors::{OwoColorize, style};
use strum::Display;

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug,Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size"}]
    len_btyes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Creating my First CLI Application >>>>>")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let get_files=get_files(&path);
            let mut table=Table::new(get_files);
            table.with(Style::rounded()); 
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);  
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);     
            table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW); 
            table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);  
            println!("{}",table);
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error reading directory".red());
    }
}

// Function to display files.
fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
}

// After getting Mapping Data into our Struct.
fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file.file_name().into_string().unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() { EntryType::Dir } else { EntryType::File },
            len_btyes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date:DateTime<Utc>=modi.into();
                format!("{}",date.format("%a %b %e %Y"))
            } else {
                String::default()
            }
        });
    }
}
