use clap::{Args, Parser, Subcommand};

use crate::velocity::{database::Database, error::VeloError};

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
            Action::Add(data) => db.add_bookmark(&data.url),
        }
    }
}

#[derive(Args, Debug)]
pub struct AddData {
    pub url: String,
}

impl Cli {
    pub fn perform_action(&self, db: &mut Database) {
        self.action.execute(db);
    }
}
