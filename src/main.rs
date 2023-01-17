mod velocity;
use velocity::database::Database;

fn main() {
    //first open a new file and also generate the related path to it using Pathbuf.
    let mut db = match Database::new() {
        Ok(db) => db,
        Err(err) => return println!("{}", err),
    };

    db.add_bookmark("github.com".into()).unwrap();

    println!("{:?}", db);
    //cli interface will make it at the end after integrating all the features.
}
