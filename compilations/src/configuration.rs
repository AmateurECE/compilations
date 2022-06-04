///////////////////////////////////////////////////////////////////////////////
// NAME:            configuration.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic to load configuration from disk.
//
// CREATED:         06/03/2022
//
// LAST EDITED:     06/03/2022
////

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
pub async fn load_secret(file_path: &str) -> Secret {
    todo!()
}

pub async fn load_configuration(file_path: &str) -> Configuration {
    todo!()
}


///////////////////////////////////////////////////////////////////////////////
