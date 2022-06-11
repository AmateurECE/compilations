///////////////////////////////////////////////////////////////////////////////
// NAME:            resolver.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A mechanism to resolve URI's of service endpoints in axum.
//
// CREATED:         06/03/2022
//
// LAST EDITED:     06/11/2022
////

use std::collections::HashMap;

use derive_builder::Builder;

#[derive(Builder, Clone, Default)]
pub struct Resolver {
    hostname: String,
    script_name: Option<String>,
    #[builder(setter(custom))]
    routes: HashMap<String, String>,
}

impl ResolverBuilder {
    pub fn route(&mut self, app_name: String, path: String) -> &mut Self {
        if self.routes.is_none() {
            self.routes = Some(HashMap::new())
        }
        self.routes.as_mut().unwrap().insert(app_name, path);
        self
    }
}

impl Resolver {
    pub fn get(&self, app_name: &str) -> Option<String> {
        match self.routes.get(app_name) {
            Some(path) => Some(
                self.script_name.as_deref().unwrap_or("").to_owned() + path
            ),
            None => None,
        }
    }

    pub fn get_full(&self, app_name: &str) -> Option<String> {
        match self.routes.get(app_name) {
            Some(path) => Some(
                "https://".to_string() + &self.hostname
                    + self.script_name.as_deref().unwrap_or("")
                    + path
            ),
            None => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
