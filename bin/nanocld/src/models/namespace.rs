use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use nanocl_stubs::namespace::{Namespace, NamespacePartial};

use crate::schema::namespaces;

/// This structure represent the namespace in the database.
/// A namespace is a group of cargo or virtual machine that share the same network.
/// It is used to isolate the services.
#[derive(
  Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable,
)]
#[diesel(primary_key(name))]
#[diesel(table_name = namespaces)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceDb {
  /// The name as primary key of the namespace
  pub name: String,
  /// When the namespace was created
  pub created_at: chrono::NaiveDateTime,
  /// User defined metadata
  pub metadata: Option<serde_json::Value>,
}

impl NamespaceDb {
  /// Create a new namespace
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      created_at: chrono::Utc::now().naive_utc(),
      metadata: None,
    }
  }
}

impl From<&NamespacePartial> for NamespaceDb {
  fn from(p: &NamespacePartial) -> Self {
    Self {
      name: p.name.clone(),
      created_at: chrono::Utc::now().naive_utc(),
      metadata: p.metadata.clone(),
    }
  }
}

impl From<NamespaceDb> for Namespace {
  fn from(namespace: NamespaceDb) -> Self {
    Self {
      name: namespace.name,
      created_at: namespace.created_at,
      metadata: namespace.metadata,
    }
  }
}
