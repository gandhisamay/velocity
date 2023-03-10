use serde_json::json;

use crate::velocity::bookmark::Bookmark;

use super::error::VeloError;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
// TODO: Store urls.

#[derive(Debug)]
pub struct Database {
    pub velocity_json: File,
    pub content: Box<serde_json::Value>,
}

impl Database {
    pub fn new() -> Result<Self, VeloError> {
        //Get the username from the environment variables.
        // TODO: If home env not found store it in the current directory.
        // Also store in the env the location of the velo file.
        let mut home_path = env::var("HOME").unwrap();
        //
        let mut f_path = PathBuf::new();
        //Just for development purposes.
        home_path.push_str("/projects/velocity");
        f_path.push(home_path);
        f_path.push(".velocity");
        f_path.set_extension("json");

        // println!("File path is {}", f_path.display());

        //No need to create the new file. This method will always open and write the template to
        //the file.
        let file_exists: bool = f_path.exists();
        // println!("velocity.json exists : {}", file_exists);

        let result = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&f_path);

        let mut file = match result {
            Ok(f) => f,
            Err(_) => {
                return Err(VeloError::FileIOError(
                    "Failed to open or create velocity.json file".into(),
                ))
            }
        };

        if !file_exists {
            let mut db = Self {
                velocity_json: file,
                content: Box::new(json!(null)),
            };
            match db.write_template() {
                Ok(_) => (),
                Err(err) => return Err(err),
            };

            Ok(db)
        } else {
            let mut json_content = String::new();
            match file.read_to_string(&mut json_content) {
                Ok(_) => (),
                Err(err) => println!(
                    "Error while reading the contents of the velocity.json file\n{}",
                    err
                ),
            }

            let content =
                Box::new(serde_json::from_str::<serde_json::Value>(&json_content).unwrap());

            let db = Self {
                velocity_json: file,
                content,
            };
            Ok(db)
        }
    }

    fn write_template(&mut self) -> Result<(), VeloError> {
        self.content = Box::new(json!({
            "bookmarks": Vec::<Bookmark>::new(),
        }));
        let boilerplate_string = serde_json::to_string_pretty(&self.content).unwrap();
        Ok(self
            .velocity_json
            .write_all(&boilerplate_string.as_bytes())
            .unwrap())
    }

    #[allow(dead_code)]
    pub fn print_database_content(&mut self) {
        //Print the file content
        let mut file_content = String::new();
        self.velocity_json
            .read_to_string(&mut file_content)
            .unwrap();
    }
}
