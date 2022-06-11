///////////////////////////////////////////////////////////////////////////////
// NAME:            configuration.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic to load configuration from disk.
//
// CREATED:         06/03/2022
//
// LAST EDITED:     06/11/2022
////

use std::fs::File;
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub listen_address: String,
    pub script_name: Option<String>,
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
    pub secret: String,
}

// Load the client secret from the filesystem.
pub async fn load_secret(file_path: &str) -> io::Result<Secret> {
    let input_file = File::open(file_path)?;
    serde_yaml::from_reader(input_file)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub async fn load_configuration(file_path: &str) -> io::Result<Configuration> {
    let input_file = File::open(file_path)?;
    serde_yaml::from_reader(input_file)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}


///////////////////////////////////////////////////////////////////////////////
