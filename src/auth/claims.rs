use crate::auth::claims::AccessLevel::Write;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(strum_macros::Display, Debug)]
pub enum AccessLevel {
    Write,
    #[allow(unused)]
    Read,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub(crate) permissions: Option<HashSet<String>>,
}

impl Claims {
    pub fn validate_permissions(&self, required_access_level: String) -> bool {
        self.permissions
            .as_ref()
            .is_some_and(|existing_permissions| {
                // Users with WRITE permission, also have READ
                existing_permissions.contains(&required_access_level)
                    || existing_permissions.contains(&Write.to_string())
            })
    }
}
