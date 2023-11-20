use bollard_next::service::ContainerInspectResponse;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Node {
  pub name: String,
  pub ip_address: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct NodeContainerSummary {
  pub node: String,
  pub ip_address: String,
  pub container: ContainerInspectResponse,
}

impl NodeContainerSummary {
  pub fn new(
    node: String,
    ip_address: String,
    container: ContainerInspectResponse,
  ) -> Self {
    Self {
      node,
      ip_address,
      container,
    }
  }
}
