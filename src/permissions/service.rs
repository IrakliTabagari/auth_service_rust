use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Thing;
use crate::permissions::models::Permission;
use std::sync::Arc;
use axum::Json;

pub async fn get_root() {}

pub async fn add_permission(db: Arc<Surreal<Mem>>, permission: Permission) -> Result<Thing, surrealdb::Error> {
    let res = db.create("permission").content(permission).await?;
    Ok(res)
}

pub async fn get_permissions(db: Arc<Surreal<Mem>>) -> Result<Vec<Permission>, surrealdb::Error> {
    let res = db.select("permission").await?;
    Ok(res)
}

pub async fn seed_permissions(db: Arc<Surreal<Mem>>) -> Result<(), surrealdb::Error> {
    let predefined_permissions = vec![
        Permission { id: None, name: "Private.Users.Create".to_string(), description: "Permission to create new private modules user".to_string() },
        Permission { id: None, name: "Private.Users.Update".to_string(), description: "Permission to update private modules user".to_string() },
        Permission { id: None, name: "Private.Users.View".to_string(), description: "Permission to view private modules user".to_string() },
        Permission { id: None, name: "Private.Users.Delete".to_string(), description: "Permission to delete private modules user".to_string() },
        Permission { id: None, name: "Public.Users.Create".to_string(), description: "Permission to create new public modules user".to_string() },
        Permission { id: None, name: "Public.Users.Update".to_string(), description: "Permission to update public modules user".to_string() },
        Permission { id: None, name: "Public.Users.View".to_string(), description: "Permission to view public modules user".to_string() },
        Permission { id: None, name: "Public.Users.Delete".to_string(), description: "Permission to delete public modules user".to_string() },
    ];

    for permission in predefined_permissions {
        let existing_permissions = get_permissions(db.clone()).await?;
        if !existing_permissions.iter().any(|p| p.name == permission.name) {
            add_permission(db.clone(), permission).await?;
        }
    }

    Ok(())
}