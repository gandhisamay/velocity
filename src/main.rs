mod velocity;
use velocity::database::Database;

fn main() {
    //first open a new file and also generate the related path to it using Pathbuf.
    let db = match Database::new() {
        Ok(db) => db,
        Err(err) => return println!("{}", err),
    };

    println!("{:?}", db);
    //cli interface will make it at the end after integrating all the features.
}
