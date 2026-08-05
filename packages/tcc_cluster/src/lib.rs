//! TCC Launcher cluster management.

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: Uuid,
    pub name: String,
    pub version: String,
}

impl Cluster {
    pub fn new(name: String, version: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            version,
        }
    }
}

pub async fn list_clusters() -> Vec<Cluster> {
    Vec::new()
}