use serde_json::json;

use super::error::VeloError;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};
//TODO: Store urls.

#[derive(Debug)]
pub struct Database {
    pub velo: File,
}

impl Database {
    pub fn new() -> Result<Self, VeloError> {
        //Get the username from the environment variables.
        // TODO: If home env not found store it in the current directory.
        // Also store in the env the location of the velo file.
        // let mut home_path = env::var("HOME").unwrap();
        //
        let mut f_path = PathBuf::new();
        //Just for development purposes.
        let home_path: String = ".".into();
        f_path.push(home_path);
        f_path.push(".velocity");
        f_path.set_extension("json");

        println!("File path is {}", f_path.display());

        //No need to create the new file
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&f_path)
        {
            Ok(mut file) => {
                //Create a empty json inside the file.
                let empty_json = serde_json::to_string_pretty(&json!({})).unwrap();
                file.write_all(&empty_json.as_bytes()).unwrap();
                return Ok(Self { velo: file });
            }
            Err(_) => {
                return Err(VeloError::ConnectionError(
                    "Failed to create or open the velo database!".to_string(),
                ))
            }
        }
    }

    pub fn add_bookmark(&self, input: String) -> Result<(), VeloError> {
        unimplemented!()
    }
}
