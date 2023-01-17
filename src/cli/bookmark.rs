use std::io::{Seek, SeekFrom, Write};

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
    ///adds a new bookmark to the collection.
    Add(AddData),
}

impl Action {
    pub fn execute(&self, db: &mut Database) -> Result<(), VeloError> {
        match self {
            Action::Add(data) => {
                return {
                    let bookmark = Bookmark::new(&data.name, &data.url);
                    let clone = bookmark.clone();

                    //Now time to add the bookmark to the bookmarks array.
                    // TODO: Do error handling everywhere wherever visible.

                    db.content.get_mut("bookmarks").and_then(|x| {
                        x.as_array_mut()
                            .and_then(|y| Some(y.push(serde_json::to_value(bookmark).unwrap())))
                    });

                    let write = serde_json::to_string_pretty(&db.content).unwrap();

                    db.velocity_json.seek(SeekFrom::Start(0)).unwrap();
                    db.velocity_json.write_all(&write.as_bytes()).unwrap();
                    println!("{}: {} added successfully!", clone.name, clone.url);
                    Ok(())
                };
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct AddData {
    name: String,
    url: String,
}

impl Cli {
    pub fn perform_action(&self, db: &mut Database) -> Result<(), VeloError> {
        self.action.execute(db)
    }
}
