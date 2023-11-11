use rusqlite::{Connection, Result};
use serde_json::json;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn read_files_in_dir(dir_path: &str) -> serde_json::Value {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension() == Some(OsStr::new("md")) {
            let content = fs::read_to_string(path).unwrap();
            let file = json!({
                "path": path.display().to_string(),
                "content": content
            });
            files.push(file);
        }
    }

    json!({ "files": files })
}

#[derive(Debug)]
struct Wiki {
    id: i32,
    wiki_folder_path: String,
}

fn init_sqlite_db_with_wiki_folder_path(folder_path: &str) {
    delete_file_if_exists("test.db");
    // TODO: unwrap is not good, change
    create_sqlite_db().unwrap();
    add_wiki_folder_path(folder_path).unwrap();
}

// if file exists on given path, delete it
fn delete_file_if_exists(file_path: &str) {
    if Path::new(file_path).exists() {
        let _ = fs::remove_file(file_path);
        println!("{} was deleted", file_path);
    } else {
        println!("File {} does not exist", file_path);
    }
}

// TODO: don't create tables if they already exist
// TODO: automatic migrations?
// create sqlite db with tables
fn create_sqlite_db() -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE wiki (
            id    INTEGER PRIMARY KEY,
            wiki_folder_path  TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE topics (
            topic_name  TEXT NOT NULL UNIQUE PRIMARY KEY,
            file_path  TEXT NOT NULL,
            file_content  TEXT NOT NULL,
            topic_content  TEXT NOT NULL,
            pretty_name  TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE notes (
            topic_id  INTEGER PRIMARY KEY,
            note  TEXT NOT NULL,
            url  TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE subnotes (
            note_id  INTEGER PRIMARY KEY,
            subnote  TEXT NOT NULL,
            subnote_order  INTEGER NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE links (
            topic_id  INTEGER PRIMARY KEY,
            title  TEXT NOT NULL,
            url  TEXT NOT NULL,
            description  TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE related_links (
            link_id  INTEGER PRIMARY KEY,
            title  TEXT NOT NULL,
            url  TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

// TODO: make it edit in case it already exists
// add wiki_folder_path to wiki table
fn add_wiki_folder_path(wiki_folder_path: &str) -> Result<()> {
    let conn = Connection::open("test.db")?;
    let wiki = Wiki {
        id: 0,
        wiki_folder_path: wiki_folder_path.to_string(),
    };
    conn.execute(
        "INSERT INTO wiki (wiki_folder_path) VALUES (?1)",
        &[&wiki.wiki_folder_path],
    )?;
    Ok(())
}
