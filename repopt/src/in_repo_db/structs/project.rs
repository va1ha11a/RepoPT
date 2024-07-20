use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectId(pub String);
#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectName(String);
#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectDescription(String);

#[derive(TypedBuilder)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[display(fmt = "Project ID: {id}\nName: {name}\nDescription: {description}")]
pub(crate) struct Project {
    id: ProjectId,
    name: ProjectName,
    description: ProjectDescription,
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl Project {
    pub(crate) fn id(&self) -> &ProjectId {
        &self.id
    }

    pub(crate) fn name(&self) -> &ProjectName {
        &self.name
    }
}
