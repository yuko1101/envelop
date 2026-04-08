use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Env {
    pub variables: HashMap<String, EnvVariableOperation>,
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum EnvVariableOperation {
    Set(String),
    Affix {
        #[serde(default)]
        prefix: Vec<String>,
        #[serde(default)]
        suffix: Vec<String>,
        separator: String,
    },
    Unset,
}

#[derive(Serialize, Debug)]
pub struct EnvExpanded {
    pub variables: HashMap<String, EnvVariableOperationExpanded>,
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct EnvVariableOperationExpanded {
    pub prefix: Option<String>,
    pub value: Option<String>,
    pub suffix: Option<String>,
    pub separator: Option<String>,
}

impl Env {
    pub fn into_expanded(self) -> EnvExpanded {
        EnvExpanded {
            variables: self
                .variables
                .into_iter()
                .map(|(k, v)| (k, v.into_expanded()))
                .collect(),
            args: self.args,
            cwd: self.cwd,
        }
    }
}

impl EnvVariableOperation {
    pub fn into_expanded(self) -> EnvVariableOperationExpanded {
        match self {
            EnvVariableOperation::Set(value) => EnvVariableOperationExpanded {
                prefix: None,
                value: Some(value),
                suffix: None,
                separator: None,
            },
            EnvVariableOperation::Affix {
                prefix,
                suffix,
                separator,
            } => EnvVariableOperationExpanded {
                prefix: if prefix.is_empty() {
                    None
                } else {
                    Some(prefix.join(&separator))
                },
                value: None,
                suffix: if suffix.is_empty() {
                    None
                } else {
                    Some(suffix.join(&separator))
                },
                separator: Some(separator),
            },
            EnvVariableOperation::Unset => EnvVariableOperationExpanded {
                prefix: None,
                value: Some(String::new()),
                suffix: None,
                separator: None,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EvelopContext {
    pub target: PathBuf,
    pub env: EnvExpanded,
}
