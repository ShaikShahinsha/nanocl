use diesel::prelude::*;
use nanocl_stubs::vm_spec::{VmSpecPartial, VmSpecUpdate};

use crate::schema::vms;

use super::NamespaceDb;

/// This structure represent the vm in the database.
/// A vm is a virtual machine that is running on the server.
/// The vm is linked to a namespace.
/// We use the `spec_key` to link to the vm spec.
/// The `key` is used to identify the vm and is generated as follow: `namespace_name-vm_name`.
#[derive(Clone, Debug, Queryable, Identifiable, Insertable, Associations)]
#[diesel(primary_key(key))]
#[diesel(table_name = vms)]
#[diesel(belongs_to(NamespaceDb, foreign_key = namespace_name))]
pub struct VmDb {
  /// The key of the vm
  pub key: String,
  /// The name of the vm
  pub name: String,
  /// The created at date
  pub created_at: chrono::NaiveDateTime,
  /// The namespace name reference
  pub namespace_name: String,
  /// The status key
  pub status_key: String,
  /// The spec key reference
  pub spec_key: uuid::Uuid,
}

/// This structure is used to update a vm in the database.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = vms)]
pub struct VmUpdateDb {
  /// The key of the vm
  pub key: Option<String>,
  /// The namespace name reference
  pub namespace_name: Option<String>,
  /// The name of the vm
  pub name: Option<String>,
  /// The spec key reference
  pub spec_key: Option<uuid::Uuid>,
}

/// Arguments to create a new vm obj
pub struct VmObjCreateIn {
  pub namespace: String,
  pub spec: VmSpecPartial,
  pub version: String,
}

/// Arguments to create a new vm history entry
pub struct VmObjPutIn {
  pub spec: VmSpecPartial,
  pub version: String,
}

/// Arguments to update a vm history entry
pub struct VmObjPatchIn {
  pub spec: VmSpecUpdate,
  pub version: String,
}
