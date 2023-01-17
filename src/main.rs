mod cli;
mod velocity;
use clap::Parser;
use cli::bookmark::Cli;
use velocity::database::Database;

fn main() {
    //first open a new file and also generate the related path to it using Pathbuf.
    let mut db = match Database::new() {
        Ok(db) => db,
        Err(err) => return println!("{}", err),
    };
    //
    // db.add_bookmark("github.com".into()).unwrap();
    // db.print_database_content();
    //cli interface will make it at the end after integrating all the features.
    let out = Cli::parse();
    out.perform_action(&mut db);
}
