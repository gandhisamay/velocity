use std::{
    io::{Seek, SeekFrom, Write},
    process::Command,
};

use clap::{Args, Parser, Subcommand};

use crate::velocity::{bookmark::Bookmark, database::Database, error::VeloError};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    ///Adds a new bookmark to the collection.
    Add(AddData),
    ///Launches the bookmark saved by you in the collection.
    Open(BookmarkName),
}

impl Action {
    pub fn execute(&self, db: &mut Database) -> Result<(), VeloError> {
        match self {
            Action::Add(data) => {
                return {
                    let name = data.name.clone();
                    let url = data.url.clone();
                    let bookmark = Bookmark::new(data.name.clone(), data.url.clone());

                    //Now time to add the bookmark to the bookmarks array.
                    // TODO: Do error handling everywhere wherever visible.

                    db.content.get_mut("bookmarks").and_then(|x| {
                        x.as_array_mut()
                            .and_then(|y| Some(y.push(serde_json::to_value(bookmark).unwrap())))
                    });

                    let write = serde_json::to_string_pretty(&db.content).unwrap();

                    db.velocity_json.seek(SeekFrom::Start(0)).unwrap();
                    db.velocity_json.write_all(&write.as_bytes()).unwrap();
                    println!("{}: {} added successfully!", name, url);
                    Ok(())
                };
            }
            Action::Open(data) => {
                //Need to find the website in the database now.
                let storage = db
                    .content
                    .get_mut("bookmarks")
                    .and_then(|x| x.as_array_mut())
                    .unwrap();

                for bookmark_value in storage {
                    let bookmark =
                        serde_json::from_value::<Bookmark>(bookmark_value.clone()).unwrap();
                    match bookmark.map.get(&data.name) {
                        Some(url) => {
                            let website = format!("/usr/bin/brave-browser -- {}", url);
                            match Command::new("sh").arg("-c").arg(website).output() {
                                Ok(_) => println!("Bookmark launched successfully"),
                                Err(_) => {
                                    return Err(VeloError::BookmarkOpenFailure(
                                        "Failed to open the website".into(),
                                    ))
                                }
                            };
                            println!("{:?}, {:?}", &data.name, &url);
                        }
                        None => (),
                    };
                }

                Ok(())
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct AddData {
    name: String,
    url: String,
}

#[derive(Args, Debug)]
pub struct BookmarkName {
    name: String,
}

impl Cli {
    pub fn perform_action(&self, db: &mut Database) -> Result<(), VeloError> {
        self.action.execute(db)
    }
}
